# Asteroid Shooter ☄️
A classic asteroid shooter game built in Rust using the Bevy game engine — vibe-coded with Claude as the AI coding partner.

# How to Play
Pilot your ship through an endless field of incoming asteroids. Shoot them down before they hit you — big asteroids split into smaller, faster ones when destroyed.
Controls
KeyActionA or ←Rotate leftD or →Rotate rightW or ↑Thrust forwardSpaceShootHHide / show controls hint

# Scoring
TargetPointsLarge asteroid20Medium asteroid50Small asteroid100
Asteroids spawn from the edges of the screen every 2 seconds and speed up as you progress. Your ship and all projectiles wrap around the screen edges. A collision with any asteroid ends the game.

# Running the Game 🚀 
## Prerequisites

Rust (installs cargo)
On Linux, you may also need:

bash  sudo apt install pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-dev
## Run
bashcargo run

The first build takes a few minutes — Bevy is a large dependency. Subsequent builds are much faster.

# Project Structure
src/
  main.rs       — all game logic (systems, components, setup)
Cargo.toml      — dependencies (bevy 0.14)
