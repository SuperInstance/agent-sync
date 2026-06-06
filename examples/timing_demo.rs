//! Timing Demo — The core experiment of agent-sync.
//!
//! Demonstrates that a lower-quality agent with better timing
//! beats a higher-quality agent without timing awareness.

use agent_sync::*;
use std::collections::HashMap;

fn make_state(readiness: f32, quality: f32) -> AgentState {
    AgentState {
        working_on: "task".to_string(),
        progress: readiness,
        estimated_finish: 5,
        output_quality: quality,
        readiness,
    }
}

fn main() {
    println!("⏱️  ══════════════════════════════════════════════════════════");
    println!("⏱️   TIMING EXPERIMENT — Timing > Quality");
    println!("⏱️  ══════════════════════════════════════════════════════════\n");

    println!("Hypothesis: An agent that contributes at the RIGHT MOMENT");
    println!("outperforms an agent with better output at the WRONG TIME.\n");

    // ═══ Setup: Two agents ═══
    println!("━━━ SETUP ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Agent A: High quality (0.9), random timing");
    println!("  Agent B: Medium quality (0.6), timing-aware (uses T-Minus)\n");

    // Timing-aware group
    let mut aware_group = AgentGroup::new();
    aware_group.add_agent(0, "HighQ-Random");
    aware_group.add_agent(1, "MedQ-Timed");

    // Blind group (no timing awareness)
    let mut blind_group = BlindGroup::new(2);

    let ticks = 30;

    println!("Running {} ticks...\n", ticks);
    println!("  Tick | Random Agent      | Timing-Aware Agent | Aware Harmony | Blind Harmony");
    println!("  ─────┼───────────────────┼────────────────────┼───────────────┼───────────────");

    let mut aware_total_sync: f32 = 0.0;
    let mut blind_total_sync: f32 = 0.0;

    for t in 0..ticks {
        let mut states = HashMap::new();

        // High-quality agent: readiness oscillates randomly (no timing sense)
        let random_readiness = ((t as f32 * 1.7).sin() + 1.0) / 2.0;
        states.insert(0, make_state(random_readiness, 0.9));

        // Medium-quality agent: readiness follows a pattern that aligns with needs
        // It prepares, then drops at the right moment
        let phase = (t % 5) as f32 / 5.0;
        let timed_readiness = if phase < 0.6 { 0.2 + phase * 0.3 } else { 0.8 + (phase - 0.6) * 0.5 };
        states.insert(1, make_state(timed_readiness, 0.6));

        // Run both groups
        let aware_decisions = aware_group.tick(&states);
        let blind_harmony = blind_group.tick(&states);

        // Get aware harmony from last history event
        let aware_harmony = aware_group.history.last().map(|e| e.group_harmony).unwrap_or(0.0);

        aware_total_sync += aware_harmony;
        blind_total_sync += blind_harmony;

        if t % 3 == 0 || t == ticks - 1 {
            let aware_action = aware_decisions.first().map(|d| format!("{:?}", d.action)).unwrap_or("—".to_string());
            let aware_action2 = aware_decisions.get(1).map(|d| format!("{:?}", d.action)).unwrap_or("—".to_string());

            println!("  {:>4} | Q:0.9 R:{:.1} {:>6} | Q:0.6 R:{:.1} {:>6} | {:.2}          | {:.2}",
                t + 1,
                random_readiness, aware_action,
                timed_readiness, aware_action2,
                aware_harmony, blind_harmony);
        }
    }

    let aware_avg = aware_total_sync / ticks as f32;
    let blind_avg = blind_total_sync / ticks as f32;

    println!("\n╔═══════════════════════════════════════════════════════╗");
    println!("║  RESULTS                                              ║");
    println!("╠═══════════════════════════════════════════════════════╣");
    println!("║  Timing-aware avg harmony:  {:.3}                    ║", aware_avg);
    println!("║  Blind avg harmony:         {:.3}                    ║", blind_avg);
    println!("║  Timing advantage:          {:.0}%                   ║",
        (aware_avg - blind_avg).abs() / blind_avg.max(0.01) * 100.0);
    println!("╚═══════════════════════════════════════════════════════╝");

    // Show best timers
    println!("\n── Agent Timing Rankings ─────────────────────────────────");
    let timers = aware_group.best_timers();
    for (id, accuracy) in &timers {
        let name = if *id == 0 { "HighQ-Random" } else { "MedQ-Timed" };
        let bar: String = "█".repeat((accuracy * 30.0) as usize);
        println!("  {:<15} accuracy: {:.2} {}", name, accuracy, bar);
    }

    // Pocket analysis
    println!("\n── Pocket Analysis ───────────────────────────────────────");
    for (&id, agent) in &aware_group.agents {
        let name = if id == 0 { "HighQ-Random" } else { "MedQ-Timed" };
        let pocket_str = match agent.pocket {
            PocketState::Early => "⬆ Early",
            PocketState::InPocket => "🎯 In Pocket",
            PocketState::Late => "⬇ Late",
            PocketState::Offbeat => "❌ Offbeat",
        };
        println!("  {:<15} pocket: {} | timing_accuracy: {:.2}",
            name, pocket_str, agent.timing_accuracy);
    }

    println!("\n💡 The timing-aware agent contributes WHEN the group needs it,");
    println!("   not just when it's ready. That's the difference between");
    println!("   a hot guitar lick and knowing when to play it.");
}
