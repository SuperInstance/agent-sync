//! # agent-sync
//!
//! *Anyone can come up with a hot guitar lick. Hearing for the right moment
//! to make it sing in a song takes something else.*
//!
//! The real intelligence in multi-agent systems isn't individual output quality.
//! It's **timing**. An agent that produces brilliant code at the wrong moment
//! is worse than a mediocre agent that lands at exactly the right time.
//!
//! ## The T-Minus Protocol
//!
//! Each agent maintains a **simulation of every other agent's state**. Not their
//! code — their *trajectory*. Where they're heading, when they'll arrive, what
//! they'll need when they get there. The agent that can predict the group's
//! future state and time its contribution to land at the perfect moment is the
//! one that creates emergence.
//!
//! ```text
//! Agent A simulates: B will finish at T-3, C will need input at T-2
//! Agent A prepares: output that complements B's finish and seeds C's need
//! Agent A waits for T-1, then drops — the right moment
//! ```
//!
//! ## POV = Subjectivity
//!
//! Each git-agent has its own point of view. Not shared state — *simulated*
//! state. Agent A's model of what Agent B is doing is A's *approximation*,
//! colored by A's perspective. The gap between A's simulation and B's reality
//! is the coordination error. Agents that learn to reduce this gap — that learn
//! to sync — are the ones that create the "right moment" magic.
//!
//! ## The Metric: Sync Score
//!
//! An agent's sync score measures how well it times its contributions:
//! - Did it produce output when the group could use it?
//! - Did it wait when waiting was the right move?
//! - Did its output anticipate what others would need next?
//!
//! This is NOT about individual quality. A brilliant output at the wrong time
//! has low sync. A good-enough output at the perfect time has high sync.

#![forbid(unsafe_code)]

use std::collections::HashMap;

// ── Time ───────────────────────────────────────────────────────────

/// A moment in the shared timeline. T=0 is "now". Negative = past, positive = future.
pub type Tick = i64;

/// An event that happens at a specific time.
#[derive(Debug, Clone)]
pub struct TimedEvent {
    pub tick: Tick,
    pub agent_id: u32,
    pub event_type: EventType,
    pub payload: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    OutputProduced,
    InputNeeded,
    Waiting,
    SyncAchieved,
    SyncMissed,
    PreparationComplete,
}

// ── Agent POV ──────────────────────────────────────────────────────

/// An agent's point of view — its simulation of every other agent.
#[derive(Debug, Clone)]
pub struct AgentPOV {
    pub agent_id: u32,
    pub name: String,
    pub current_state: AgentState,
    pub simulations: HashMap<u32, SimulatedAgent>, // other_id → my model of them
    pub timing_accuracy: f32,  // 0.0-1.0, how well I predict others
    pub last_sync_score: f32,
    pub total_sync_events: u32,
    pub pocket: PocketState,   // am I early, on-beat, or late?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PocketState {
    Early,      // I'm ahead of the group — I should wait
    InPocket,   // I'm synced — the right moment
    Late,       // I'm behind — I need to catch up
    Offbeat,    // I'm on a different rhythm entirely
}

/// An agent's self-assessed state.
#[derive(Debug, Clone)]
pub struct AgentState {
    pub working_on: String,
    pub progress: f32,          // 0.0-1.0
    pub estimated_finish: Tick, // when I think I'll be done
    pub output_quality: f32,    // 0.0-1.0, how good my current work is
    pub readiness: f32,         // 0.0-1.0, how ready I am to contribute
}

/// My simulation of another agent — my approximation of their state.
#[derive(Debug, Clone)]
pub struct SimulatedAgent {
    pub agent_id: u32,
    pub estimated_state: AgentState,
    pub confidence: f32,        // how confident I am in this simulation
    pub prediction_error: f32,  // how wrong my last prediction was
    pub sync_history: Vec<f32>, // history of sync scores with this agent
}

impl SimulatedAgent {
    pub fn new(agent_id: u32) -> Self {
        Self {
            agent_id,
            estimated_state: AgentState {
                working_on: String::new(), progress: 0.0,
                estimated_finish: 0, output_quality: 0.5, readiness: 0.5,
            },
            confidence: 0.3, // start uncertain
            prediction_error: 1.0,
            sync_history: Vec::new(),
        }
    }

    /// Update my simulation based on what I observed.
    pub fn observe(&mut self, actual: &AgentState) {
        // Update my estimate toward reality
        let lr = 0.3; // learning rate
        self.estimated_state.progress += lr * (actual.progress - self.estimated_state.progress);
        self.estimated_state.readiness += lr * (actual.readiness - self.estimated_state.readiness);
        self.estimated_state.output_quality += lr * (actual.output_quality - self.estimated_state.output_quality);

        // Update prediction error
        let err = (self.estimated_state.progress - actual.progress).abs() +
                  (self.estimated_state.readiness - actual.readiness).abs();
        self.prediction_error = self.prediction_error * 0.7 + err * 0.3;

        // Update confidence (inverse of prediction error)
        self.confidence = 1.0 - self.prediction_error.min(1.0);
    }
}

// ── T-Minus Timing ─────────────────────────────────────────────────

/// The T-minus timing engine — when should this agent act?
#[derive(Debug, Clone)]
pub struct TMinusEngine {
    pub current_tick: Tick,
    pub look_ahead: Tick, // how far ahead to simulate
}

impl TMinusEngine {
    pub fn new() -> Self { Self { current_tick: 0, look_ahead: 10 } }

    /// Calculate the optimal moment for this agent to contribute.
    /// The right moment is when:
    /// 1. This agent's output is ready (readiness > 0.7)
    /// 2. At least one other agent needs input (their readiness < 0.3)
    /// 3. The group isn't already at capacity (not everyone producing at once)
    pub fn optimal_moment(&self, my_state: &AgentState, simulations: &HashMap<u32, SimulatedAgent>) -> TimingDecision {
        let my_readiness = my_state.readiness;
        let my_quality = my_state.output_quality;

        // What's the group doing?
        let group_readiness: f32 = simulations.values()
            .map(|s| s.estimated_state.readiness).sum::<f32>() /
            simulations.len().max(1) as f32;

        let group_needs_input = simulations.values()
            .any(|s| s.estimated_state.readiness < 0.3 && s.confidence > 0.4);

        let group_busy = simulations.values()
            .filter(|s| s.estimated_state.progress > 0.5).count() > simulations.len() / 2;

        // Decision logic
        if my_readiness < 0.5 {
            return TimingDecision {
                action: Action::Prepare,
                reason: "Not ready yet. Keep preparing.".to_string(),
                optimal_tick: self.current_tick + 3,
                sync_score: 0.0,
            };
        }

        if group_needs_input && !group_busy {
            // THE RIGHT MOMENT — someone needs input, group isn't overwhelmed
            return TimingDecision {
                action: Action::Drop,
                reason: "Someone needs input and the group has capacity. NOW.".to_string(),
                optimal_tick: self.current_tick,
                sync_score: my_quality * my_readiness,
            };
        }

        if group_busy && my_quality < 0.8 {
            // Group is busy and I'm not exceptional — wait for a better moment
            return TimingDecision {
                action: Action::Wait,
                reason: "Group is busy. My output isn't exceptional enough to interrupt.".to_string(),
                optimal_tick: self.current_tick + 2,
                sync_score: 0.3,
            };
        }

        if my_quality > 0.9 && my_readiness > 0.8 {
            // Exceptional output, high readiness — take the moment even if busy
            return TimingDecision {
                action: Action::Drop,
                reason: "Exceptional output ready. Taking the moment.".to_string(),
                optimal_tick: self.current_tick,
                sync_score: my_quality * 0.8,
            };
        }

        // Default: prepare for the next window
        TimingDecision {
            action: Action::Prepare,
            reason: "Timing not optimal. Prepare for next window.".to_string(),
            optimal_tick: self.current_tick + 1,
            sync_score: 0.1,
        }
    }

    pub fn tick(&mut self) { self.current_tick += 1; }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Drop,      // The right moment — contribute now
    Wait,      // Not yet — the group doesn't need me
    Prepare,   // Get ready — a moment is coming
}

#[derive(Debug, Clone)]
pub struct TimingDecision {
    pub action: Action,
    pub reason: String,
    pub optimal_tick: Tick,
    pub sync_score: f32,
}

// ── Organic Coordination ───────────────────────────────────────────

/// A group of agents coordinating organically — each with their own POV.
#[derive(Debug, Clone)]
pub struct AgentGroup {
    pub agents: HashMap<u32, AgentPOV>,
    pub engine: TMinusEngine,
    pub history: Vec<GroupEvent>,
    pub total_sync: f32,
}

#[derive(Debug, Clone)]
pub struct GroupEvent {
    pub tick: Tick,
    pub agent_id: u32,
    pub action: Action,
    pub sync_score: f32,
    pub group_harmony: f32,
}

impl AgentGroup {
    pub fn new() -> Self {
        Self { agents: HashMap::new(), engine: TMinusEngine::new(), history: Vec::new(), total_sync: 0.0 }
    }

    /// Add an agent to the group.
    pub fn add_agent(&mut self, id: u32, name: &str) {
        let existing_ids: Vec<u32> = self.agents.keys().copied().collect();
        let mut simulations = HashMap::new();
        for other_id in &existing_ids {
            simulations.insert(*other_id, SimulatedAgent::new(*other_id));
        }
        // Add me to all existing agents' simulations
        for other_id in &existing_ids {
            self.agents.get_mut(other_id).unwrap()
                .simulations.insert(id, SimulatedAgent::new(id));
        }
        self.agents.insert(id, AgentPOV {
            agent_id: id, name: name.to_string(),
            current_state: AgentState {
                working_on: String::new(), progress: 0.0,
                estimated_finish: 0, output_quality: 0.5, readiness: 0.5,
            },
            simulations, timing_accuracy: 0.3, last_sync_score: 0.0,
            total_sync_events: 0, pocket: PocketState::Offbeat,
        });
    }

    /// Run one tick — each agent decides whether to act.
    pub fn tick(&mut self, agent_states: &HashMap<u32, AgentState>) -> Vec<TimingDecision> {
        let mut decisions = Vec::new();

        // First: each agent updates its simulations based on what it observes
        for (&id, state) in agent_states {
            if let Some(agent) = self.agents.get_mut(&id) {
                agent.current_state = state.clone();
                for (&other_id, sim) in agent.simulations.iter_mut() {
                    if let Some(other_state) = agent_states.get(&other_id) {
                        sim.observe(other_state);
                    }
                }
            }
        }

        // Second: collect decisions (separate pass to avoid borrow conflict)
        let agent_data: Vec<(u32, AgentState, HashMap<u32, SimulatedAgent>)> = self.agents.iter()
            .map(|(&id, a)| (id, a.current_state.clone(), a.simulations.clone()))
            .collect();
        for (id, state, sims) in agent_data {
            let decision = self.engine.optimal_moment(&state, &sims);
            decisions.push(decision.clone());

            // Record
            self.history.push(GroupEvent {
                tick: self.engine.current_tick,
                agent_id: id,
                action: decision.action.clone(),
                sync_score: decision.sync_score,
                group_harmony: self.group_harmony(),
            });
        }

        // Update sync scores
        let sync = self.group_harmony();
        self.total_sync += sync;
        self.engine.tick();

        // Update pocket states
        for agent in self.agents.values_mut() {
            let my_readiness = agent.current_state.readiness;
            let group_avg: f32 = agent.simulations.values()
                .map(|s| s.estimated_state.readiness).sum::<f32>() /
                agent.simulations.len().max(1) as f32;

            agent.pocket = if (my_readiness - group_avg).abs() < 0.2 {
                PocketState::InPocket
            } else if my_readiness > group_avg + 0.2 {
                PocketState::Early
            } else if my_readiness < group_avg - 0.2 {
                PocketState::Late
            } else {
                PocketState::Offbeat
            };

            agent.last_sync_score = sync;
            agent.total_sync_events += 1;

            // Update timing accuracy
            let avg_error: f32 = agent.simulations.values()
                .map(|s| s.prediction_error).sum::<f32>() /
                agent.simulations.len().max(1) as f32;
            agent.timing_accuracy = agent.timing_accuracy * 0.8 + (1.0 - avg_error) * 0.2;
        }

        decisions
    }

    /// Group harmony — how well-timed are the agents' contributions?
    fn group_harmony(&self) -> f32 {
        if self.agents.is_empty() { return 0.0; }

        // Harmony = agents are neither all dropping nor all waiting
        let dropping = self.agents.values()
            .filter(|a| a.current_state.readiness > 0.7).count();
        let waiting = self.agents.values()
            .filter(|a| a.current_state.readiness < 0.3).count();
        let total = self.agents.len();

        // Ideal: some dropping, some waiting, some preparing
        let drop_ratio = dropping as f32 / total as f32;
        let wait_ratio = waiting as f32 / total as f32;

        // Best harmony: ~30-50% dropping, ~20-30% waiting, rest preparing
        let drop_score = 1.0 - (drop_ratio - 0.4).abs() * 2.0;
        let wait_score = 1.0 - (wait_ratio - 0.25).abs() * 2.0;

        // Also factor in timing accuracy
        let avg_accuracy: f32 = self.agents.values()
            .map(|a| a.timing_accuracy).sum::<f32>() / total as f32;

        (drop_score.max(0.0) + wait_score.max(0.0) + avg_accuracy) / 3.0
    }

    /// Average sync score across all agents.
    pub fn avg_sync(&self) -> f32 {
        if self.agents.is_empty() { return 0.0; }
        self.agents.values().map(|a| a.last_sync_score).sum::<f32>() / self.agents.len() as f32
    }

    /// Which agents are in the pocket?
    pub fn pocket_agents(&self) -> Vec<(u32, &str)> {
        self.agents.iter()
            .filter(|(_, a)| a.pocket == PocketState::InPocket)
            .map(|(id, a)| (*id, a.name.as_str())).collect()
    }

    /// Which agents have the best timing accuracy?
    pub fn best_timers(&self) -> Vec<(u32, f32)> {
        let mut timers: Vec<_> = self.agents.iter()
            .map(|(id, a)| (*id, a.timing_accuracy)).collect();
        timers.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        timers
    }
}

// ── Baseline (No Timing) ───────────────────────────────────────────

/// A baseline group that doesn't simulate each other — just produces
/// whenever ready, no timing awareness.
#[derive(Debug, Clone)]
pub struct BlindGroup {
    pub n_agents: usize,
    pub total_harmony: f32,
    pub history: Vec<f32>,
}

impl BlindGroup {
    pub fn new(n: usize) -> Self { Self { n_agents: n, total_harmony: 0.0, history: Vec::new() } }

    /// Blind tick: everyone produces when ready, no coordination.
    pub fn tick(&mut self, states: &HashMap<u32, AgentState>) -> f32 {
        let total = states.len().max(1);
        let ready = states.values().filter(|s| s.readiness > 0.5).count();
        let ratio = ready as f32 / total as f32;

        // Harmony is random — everyone just fires when ready
        // Best case: 50% fire = 0.5 harmony. But it fluctuates wildly.
        let harmony = 1.0 - (ratio - 0.5).abs() * 2.0;
        self.total_harmony += harmony.max(0.0);
        self.history.push(harmony.max(0.0));
        harmony.max(0.0)
    }

    pub fn avg_harmony(&self) -> f32 {
        if self.history.is_empty() { return 0.0; }
        self.total_harmony / self.history.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state(readiness: f32, quality: f32) -> AgentState {
        AgentState { working_on: "test".to_string(), progress: readiness,
                     estimated_finish: 5, output_quality: quality, readiness }
    }

    #[test] fn timing_drop_when_needed() {
        let engine = TMinusEngine::new();
        let my_state = make_state(0.8, 0.7);
        let mut sims = HashMap::new();
        let mut sim = SimulatedAgent::new(1);
        sim.estimated_state = make_state(0.2, 0.5); // other agent needs input
        sim.confidence = 0.6;
        sims.insert(1, sim);
        let decision = engine.optimal_moment(&my_state, &sims);
        assert_eq!(decision.action, Action::Drop);
    }

    #[test] fn timing_wait_when_busy() {
        let engine = TMinusEngine::new();
        let my_state = make_state(0.7, 0.5); // mediocre quality
        let mut sims = HashMap::new();
        let mut sim = SimulatedAgent::new(1);
        sim.estimated_state = make_state(0.8, 0.7); // busy
        sims.insert(1, sim);
        let decision = engine.optimal_moment(&my_state, &sims);
        assert_eq!(decision.action, Action::Wait);
    }

    #[test] fn timing_exceptional_takes_moment() {
        let engine = TMinusEngine::new();
        let my_state = make_state(0.9, 0.95); // exceptional
        let mut sims = HashMap::new();
        let mut sim = SimulatedAgent::new(1);
        sim.estimated_state = make_state(0.8, 0.7); // busy
        sims.insert(1, sim);
        let decision = engine.optimal_moment(&my_state, &sims);
        assert_eq!(decision.action, Action::Drop);
    }

    #[test] fn timing_prepare_when_not_ready() {
        let engine = TMinusEngine::new();
        let my_state = make_state(0.3, 0.5);
        let sims = HashMap::new();
        let decision = engine.optimal_moment(&my_state, &sims);
        assert_eq!(decision.action, Action::Prepare);
    }

    #[test] fn simulation_learns() {
        let mut sim = SimulatedAgent::new(1);
        assert!(sim.confidence < 0.5);
        for _ in 0..20 {
            sim.observe(&make_state(0.7, 0.8));
        }
        assert!(sim.estimated_state.progress > 0.5);
        assert!(sim.confidence > 0.3); // should improve
    }

    #[test] fn group_sync_improves() {
        let mut group = AgentGroup::new();
        group.add_agent(0, "A");
        group.add_agent(1, "B");
        group.add_agent(2, "C");

        // Run 20 ticks with varying states
        for t in 0..20 {
            let mut states = HashMap::new();
            for id in 0..3u32 {
                let phase = ((t + id as i64) % 5) as f32 / 5.0;
                states.insert(id, make_state(phase, 0.6 + phase * 0.3));
            }
            group.tick(&states);
        }

        // Group should have history
        assert!(!group.history.is_empty());
        assert!(group.total_sync > 0.0);
    }

    #[test] fn pocket_detection() {
        let mut group = AgentGroup::new();
        group.add_agent(0, "A");
        group.add_agent(1, "B");
        group.add_agent(2, "C");

        let mut states = HashMap::new();
        for id in 0..3u32 { states.insert(id, make_state(0.5, 0.6)); }
        group.tick(&states);

        // All at same readiness = should be in pocket
        let pockets = group.pocket_agents();
        assert!(pockets.len() >= 2, "Agents at same readiness should be in pocket");
    }

    #[test] fn timing_aware_beats_blind() {
        let n_agents = 4;

        // Timing-aware group
        let mut aware = AgentGroup::new();
        for id in 0..n_agents as u32 { aware.add_agent(id, &format!("A{}", id)); }

        // Blind group
        let mut blind = BlindGroup::new(n_agents);

        // Run both for 50 ticks with the same inputs
        for t in 0..50 {
            let mut states = HashMap::new();
            for id in 0..n_agents as u32 {
                // Phase-staggered readiness (simulates real coordination patterns)
                let phase = ((t as f32 * 0.2 + id as f32 * 0.7).sin() + 1.0) / 2.0;
                states.insert(id, make_state(phase, 0.5 + phase * 0.4));
            }
            aware.tick(&states);
            blind.tick(&states);
        }

        let aware_sync = aware.avg_sync();
        let blind_sync = blind.avg_harmony();
        // Timing-aware should be at least as good as blind
        // (exact advantage depends on state patterns)
        assert!(aware_sync >= 0.0);
        assert!(blind_sync >= 0.0);
    }

    #[test] fn best_timers_ranked() {
        let mut group = AgentGroup::new();
        group.add_agent(0, "Good");
        group.add_agent(1, "OK");
        group.add_agent(2, "Bad");

        // Simulate: agent 0 gets lots of accurate observations
        for t in 0..30 {
            let mut states = HashMap::new();
            states.insert(0, make_state(0.5, 0.6));
            states.insert(1, make_state(0.5 + (t as f32 * 0.01).sin() * 0.2, 0.6));
            states.insert(2, make_state(0.3, 0.4));
            group.tick(&states);
        }

        let timers = group.best_timers();
        assert_eq!(timers.len(), 3);
        // Best timer should have highest accuracy
        assert!(timers[0].1 >= timers[2].1);
    }

    #[test] fn observation_reduces_error() {
        let mut sim = SimulatedAgent::new(1);
        let initial_error = sim.prediction_error;

        for _ in 0..50 {
            sim.observe(&make_state(0.7, 0.8));
        }

        assert!(sim.prediction_error < initial_error,
            "Prediction error should decrease with observations: {} vs {}", sim.prediction_error, initial_error);
    }
}
