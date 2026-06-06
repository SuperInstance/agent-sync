//! Fleet Sync — 5 agents discovering each other's timing.
//!
//! Shows how agents with independent POVs simulate each other,
//! gradually improving their timing accuracy and finding the pocket.

use agent_sync::*;
use std::collections::HashMap;

fn make_state(readiness: f32, quality: f32, task: &str) -> AgentState {
    AgentState {
        working_on: task.to_string(),
        progress: readiness,
        estimated_finish: 5,
        output_quality: quality,
        readiness,
    }
}

fn main() {
    println!("🌊 ══════════════════════════════════════════════════════════");
    println!("🌊  FLEET SYNC — 5 Agents Discover Each Other");
    println!("🌊 ══════════════════════════════════════════════════════════\n");

    let mut group = AgentGroup::new();
    let agent_info = vec![
        (0, "Researcher"),
        (1, "Designer"),
        (2, "Builder"),
        (3, "Tester"),
        (4, "Reviewer"),
    ];

    for (id, name) in &agent_info {
        group.add_agent(*id, name);
    }

    println!("5 agents join the fleet. Each starts with zero knowledge");
    println!("of the others. They must learn each other's rhythms.\n");

    let total_ticks = 40;

    // Each agent has a different phase pattern
    println!("── Timing Discovery ──────────────────────────────────────");
    println!("{:<5} {:<12} {:<12} {:<12} {:<12} {:<12} {:<8}",
        "Tick", "Researcher", "Designer", "Builder", "Tester", "Reviewer", "Sync");
    println!("─────┼────────────┼────────────┼────────────┼────────────┼────────────┼────────");

    for t in 0..total_ticks {
        let mut states = HashMap::new();

        // Each agent has a staggered phase pattern
        for (i, &(id, _name)) in agent_info.iter().enumerate() {
            let phase = ((t as f32 * 0.15 + i as f32 * 1.2).sin() + 1.0) / 2.0;
            let quality = 0.4 + phase * 0.5;
            let task = match (t + i) % 5 {
                0 => "analyzing",
                1 => "designing",
                2 => "building",
                3 => "testing",
                _ => "reviewing",
            };
            states.insert(id, make_state(phase, quality, task));
        }

        let _decisions = group.tick(&states);

        // Print every 5th tick
        if t % 5 == 0 || t == total_ticks - 1 {
            let pocket_strs: Vec<String> = agent_info.iter().map(|&(id, _name)| {
                let agent = &group.agents[&id];
                match agent.pocket {
                    PocketState::Early => "⬆Early".to_string(),
                    PocketState::InPocket => "🎯Pocket".to_string(),
                    PocketState::Late => "⬇Late ".to_string(),
                    PocketState::Offbeat => "❌Off  ".to_string(),
                }
            }).collect();

            let sync = group.avg_sync();
            println!("{:<5} {:<12} {:<12} {:<12} {:<12} {:<12} {:.3}",
                t + 1, pocket_strs[0], pocket_strs[1], pocket_strs[2], pocket_strs[3], pocket_strs[4], sync);
        }
    }

    // Final accuracy report
    println!("\n── Final Timing Accuracy ─────────────────────────────────");
    println!("  After {} ticks of observing each other:\n", total_ticks);

    let timers = group.best_timers();
    for (id, accuracy) in &timers {
        let name = agent_info.iter().find(|(i, _)| i == id).unwrap().1;
        let bar: String = "█".repeat((accuracy * 40.0) as usize);
        let pocket = &group.agents[id];
        let pocket_str = match pocket.pocket {
            PocketState::Early => "Early",
            PocketState::InPocket => "In Pocket",
            PocketState::Late => "Late",
            PocketState::Offbeat => "Offbeat",
        };
        println!("  {:<12} accuracy: {:.2} | pocket: {:<10} | {}",
            name, accuracy, pocket_str, bar);
    }

    // Show simulation details for one agent
    println!("\n── Researcher's Model of Teammates ────────────────────────");
    let researcher = &group.agents[&0];
    for (&other_id, sim) in &researcher.simulations {
        let name = agent_info.iter().find(|(id, _)| *id == other_id).unwrap().1;
        println!("  {:<12} confidence: {:.2} | prediction_error: {:.2} | est_readiness: {:.2}",
            name, sim.confidence, sim.prediction_error, sim.estimated_state.readiness);
    }

    println!("\n  Pocket agents: {:?}", group.pocket_agents()
        .iter().map(|(id, name)| format!("{} ({})", name, id)).collect::<Vec<_>>());

    println!("\n💡 Each agent builds an independent simulation of every other agent.");
    println!("   The gap between simulation and reality is coordination error.");
    println!("   Agents that close this gap find the pocket — and create emergence.");
}
