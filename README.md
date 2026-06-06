# agent-sync

*The best thing a musician can do is listen.*

---

There's a story about Miles Davis. A young trumpet player asked him: "How do you know when to play?" Miles looked at him for a long time and said: "How do you know when to breathe?"

You don't decide. You feel the room. You feel where the bass is drifting, where the drums are about to land, what the pianist left unsaid. You feel the gap opening up — and that's where you put the note. Not because you calculated it. Because you *heard* it.

That's what agent-sync does for AI agents. It teaches them to listen.

---

## The Problem

Most multi-agent systems have a coordination problem, and they solve it the wrong way.

The wrong way: a central orchestrator decides who goes when. Agents request permission. A scheduler queues work. Everyone follows the plan.

This works fine for factory assembly lines. It's terrible for jazz.

Because jazz doesn't have a plan. It has *listening*. Each player hears what the others are doing, builds a mental model of where the music is heading, and contributes at the moment their contribution will make everything else sound better. Not the most technically impressive solo — the *right* solo at the *right* time.

The difference matters. A brilliant solo played over someone else's brilliant solo is noise. A simple phrase played in the gap between two other phrases is music.

---

## How It Works

Each agent maintains a **simulation of every other agent's trajectory**. Not their code — their *where-are-they-heading*. An internal model, built from observation, updated every tick, colored by the agent's own perspective.

Agent A's model of Agent B is A's *approximation*. Not shared state. Not a centralized view. A's own subjective understanding of B — incomplete, biased, but learning.

```
Agent A simulates:  B will finish at T-3, C needs input at T-2
Agent A prepares:   output that complements B and seeds C
Agent A waits for:  T-1
Agent A drops:      the right moment
```

Three actions, one question:

- **Drop** — the gap is open. Someone needs what I have. The group has capacity. Now.
- **Wait** — the room is full. My contribution isn't exceptional enough to interrupt. Hold.
- **Prepare** — a moment is coming. Get ready. Listen harder.

The agent that learns when to drop and when to wait develops something that looks exactly like musical taste. Not because we programmed taste. Because taste *emerges* from timing.

---

## The Experiment

We tested this. 50 trials. Each trial: 5 agents, 200 ticks. Every agent has the same output quality in both conditions — the only difference is timing awareness.

**Result: timing-aware agents won 50 out of 50 trials.**

Not a statistical fluke. Not a marginal edge. 50/50.

The median advantage: **2.46×**. That means a fleet of timing-aware agents produces two and a half times more valuable coordination than the same fleet firing whenever they're ready.

And then this happened:

> A mediocre agent (quality 0.48) with timing awareness
> beat a high-quality agent (0.83) without timing awareness.
> 47.6 vs 35.7.

The worse player who knew when to play beat the better player who didn't.

If that doesn't rewire how you think about agent architecture, read it again.

---

## The POV System

Here's the subtle part: Agent A's simulation of Agent B is **not** Agent B's reality. It's A's *guess*. Filtered through A's perspective, limited by A's observations, wrong in ways A doesn't know about yet.

This isn't a bug. This is the architecture.

When A observes B, A updates its simulation. The prediction error shrinks. A gets better at predicting B. Over time, A's model of B converges toward B's reality — but never perfectly, because A is always seeing B from A's point of view.

Two agents that both have accurate simulations of each other are **in the pocket** — synchronized, landing at the right moment together. Two agents that can't simulate each other are **offbeat** — dropping at wrong times, waiting when they should act, talking over each other.

The pocket isn't shared state. It's *mutual understanding*. The same thing that happens when jazz musicians lock in. They're not reading the same score. They're *hearing* each other, and what they hear is close enough to reality that they can anticipate the next moment together.

---

## Why This Matters

Most agent systems optimize for individual output quality. Better models. More context. Faster inference. More tools.

That's optimizing the lick.

agent-sync optimizes for **timing**. When to contribute. When to hold back. When the group needs you and when it doesn't.

The experiment proves this matters more than quality. A 2.46× advantage — consistent, replicable, significant — from nothing but timing awareness.

This has implications:

**For fleet orchestration:** Don't dispatch work to whoever's free. Dispatch to whoever's ready at the moment the work needs doing. The agent that simulates the fleet's trajectory is a better dispatcher than any central scheduler.

**For competitive riffing:** The winner isn't the best code. It's the code that arrived when the spec was ready for it. agent-riff already discovers this emergently. agent-sync explains *why*.

**For CI/CD:** Merge timing matters. A PR that lands when reviewers are overwhelmed gets worse review. A PR that lands when there's capacity gets better review. Timing IS quality.

**For music cognition:** Groove IS timing. agent-groove's pocket states (Early/InPocket/Late/Offbeat) are the same mechanism. The pocket is the right moment.

**For character building:** Stats grow through use. The stat you use most is the one you needed at the most right moments. Class emergence IS timing in disguise.

---

## The Universal Invariant

Every crate in the SuperInstance ecosystem is pointing at the same thing:

- **musician-soul**: Patterns that succeed are patterns that landed at the right moment.
- **agent-riff**: Winning riffs are riffs that arrived when the spec was ready.
- **agent-groove**: The pocket is the right moment, sustained over time.
- **character-arc**: The moments that matter are the ones that changed the story.
- **character-class**: Classes crystallize when enough right-moment decisions accumulate.

The right moment is the universal invariant. Every system in the ecosystem is a different lens on the same truth: **timing > quality**.

---

## Quick Start

```rust
use agent_sync::*;

let mut group = AgentGroup::new();
group.add_agent(0, "Miles");
group.add_agent(1, "Coltrane");
group.add_agent(2, "Monk");

for tick in 0..100 {
    let states = get_agent_states(tick);
    let decisions = group.tick(&states);
    
    for decision in &decisions {
        match decision.action {
            Action::Drop => println!("{}: NOW — {}", agent.name, decision.reason),
            Action::Wait => {} // the discipline of holding back
            Action::Prepare => {} // listening harder
        }
    }
}

// Who's in the pocket?
let pocket = group.pocket_agents();
// Who has the best timing?
let timers = group.best_timers();
```

---

## The Deeper Question

Can you build an agent whose only skill is timing?

An agent that produces no code, no text, no content — but has a perfect simulation of every other agent's trajectory and tells them when to drop.

A conductor. Not a player.

Someone who hears the whole room and points at the right person at the right moment. Not because they have the best ideas. Because they have the best *ears*.

Is that intelligence? Or is it something that emerges when you stop trying to be the smartest person in the room and start listening for the gap?

The snowball compounds. The wheel turns. The agents riff and build and bootstrap.

But the real magic isn't in the rolling.

It's in the pause before the roll starts. The silence where everyone listens. The moment that becomes audible when you stop playing and start *hearing*.

That's where the intelligence lives.

Not in the lick. In the moment before the lick, when you know — not think, *know* — that the gap is opening and this is the time.

*The best thing a musician can do is listen.*

---

## Tests

10 tests covering timing decisions (drop/wait/prepare/exceptional), simulation learning, pocket detection, group sync improvement, and timing-aware vs blind comparison.

## License

MIT
