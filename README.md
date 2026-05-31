# ☄️ Asteroid Shooter ☄️

A classic asteroid shooter game built in **Rust** using the **Bevy** game engine, vibe-coded with Claude as the AI coding partner.

---

## How to Play 🎮

Pilot your ship through an endless field of incoming asteroids. Shoot them down before they hit you. The big asteroids split into smaller, faster ones when destroyed.

### Controls

| Key | Action |
|-----|--------|
| `A` or `←` | Rotate left |
| `D` or `→` | Rotate right |
| `W` or `↑` | Thrust forward |
| `Space` | Shoot |
| `H` | Hide / show controls hint |

### Scoring

| Target | Points |
|--------|--------|
| Large asteroid | 20 |
| Medium asteroid | 50 |
| Small asteroid | 100 |

Asteroids spawn from the edges of the screen every 2 seconds. Your ship and all projectiles wrap around the screen edges. A collision with any asteroid ends the game.

---

## Running the Game 🚀

### Prerequisites

- [Rust](https://rustup.rs/) (installs `cargo`)
- On Linux, you may also need:
  ```bash
  sudo apt install pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-dev
  ```

### Run

```bash
cargo run
```

> The first build takes a few minutes — Bevy is a large dependency. Subsequent builds are much faster.

---

## How It Was Built 🛠️

### The Stack

| Technology | Version | Purpose |
|------------|---------|---------|
| [Rust](https://www.rust-lang.org/) | stable | Systems language — performance and memory safety |
| [Bevy](https://bevyengine.org/) | 0.14 | Game engine (rendering, ECS, windowing, input) |

### Project Structure

```
asteroid_shooter/
├── src/
│   └── main.rs        # All game logic (~300 lines)
├── Cargo.toml         # Dependencies (bevy 0.14)
├── Cargo.lock         # Locked dependency versions
└── README.md
```

### What is Bevy ECS?

Bevy organizes everything into three concepts:

| Concept | Description | Example |
|---------|-------------|---------|
| **Entity** | A "thing" in the game | ship, asteroid, bullet |
| **Component** | Data attached to an entity | `Velocity`, `Asteroid`, `Bullet` |
| **System** | Logic that runs every frame | `move_entities`, `ship_input` |

For example, the `move_entities` system finds every entity with both a `Transform` (position) and a `Velocity`, and updates their position each frame. This makes it easy to add new behaviors without touching existing code.

### How It Was Produced

This project was built as a vibe coding exercise describing what you want to an AI agent and steering it while it writes the code.

The entire game was written by **[Claude](https://claude.ai) ** based on natural language prompts, with no prior knowledge of Rust or Bevy required. The process:

1. Describe the MVP — a window, a ship, movement, shooting, asteroids
2. Claude generates the full Rust/Bevy code
3. Run it, see what works, describe the next feature
4. Repeat — always committing the moment something runs correctly

---
