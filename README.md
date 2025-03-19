# Adventure Game Engine 🎮

A simple text-based adventure game engine written in Rust that reads game scenarios from a CSV file.

## Overview

This project implements a basic text-based adventure game engine where:
- Game content is read from a CSV file (`history.csv`)
- Players navigate through different scenarios by making choices
- Player life points can increase or decrease based on choices
- The game ends when the player reaches an endpoint or dies (0 life points)

## How It Works

The game engine:
1. Loads a game scenario from a CSV file
2. Presents the current situation to the player
3. Shows available options for the player to choose
4. Processes the player's choice and moves to the next scenario
5. Updates the player's life points accordingly
6. Continues until an endpoint is reached or the player dies

## CSV File Format

The game uses a CSV file with the following format:
```
TYPE;TAG;TEXT;LIFE_POINTS
```

Where:
- `TYPE`: Either "SITUACION" (situation) or "OPCION" (option)
- `TAG`: A unique identifier for the scenario or the target scenario for an option
- `TEXT`: The description of the situation or option
- `LIFE_POINTS`: Integer value that affects the player's life points

## Requirements

- Rust programming language
- CSV file with game content (named `history.csv` by default)

## Running the Game

1. Make sure you have Rust installed
2. Create a `history.csv` file with your game scenarios
3. Run the game with:

```bash
cargo run
```

## Example CSV Structure

```
SITUACION;INICIO;You are at a crossroads. Which path will you take?;0
OPCION;FOREST;Go into the dark forest;0
OPCION;MOUNTAIN;Climb the mountain;0
SITUACION;FOREST;You enter the forest and find a mysterious potion.;-10
OPCION;DRINK;Drink the potion;0
OPCION;LEAVE;Leave it and continue;0
```

## Future Improvements

- Add support for items and inventory
- Implement saving/loading game progress
- Add combat mechanics
- Support for multiple endings
- Add visual elements or simple ASCII art
