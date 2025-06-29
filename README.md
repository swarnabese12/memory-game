# 🧠 Memory Game

A simple emoji-based memory matching game built with Rust, using the `egui` and `eframe` GUI libraries. Flip cards, test your memory, and match all the pairs to win!


## 🕹️ Features

- 🎴 Emoji-based card matching
- 🧠 Memory challenge mechanics
- ⏱️ Turn-based logic with a short reveal timer
- 🎨 Built with a GUI using `egui` and `eframe`
- 🦀 Written entirely in Rust


## 📁 Project Structure

main.rs – Entry point and game logic

Card struct – Represents each card's emoji and state (Hidden, Revealed, Matched)

MemoryGameApp – The main game state and GUI renderer

## 📦 Dependencies

eframe – Native windowing and app framework

egui – Immediate mode GUI for Rust

rand – Random shuffle logic for the card grid

