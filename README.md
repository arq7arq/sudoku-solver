# Sudoku Solver 🧩

A simple command-line **Sudoku solver written in Rust**. It solves standard 9×9 Sudoku puzzles using a **recursive backtracking algorithm** and prints the resulting grid in the terminal.

## Features

- Solves standard 9×9 Sudoku puzzles.
- Uses recursive backtracking to find a solution.
- Checks each candidate against its row, column, and 3×3 box.
- Reads puzzles from a comma-separated text file.
- Displays the solved board with clear 3×3 box boundaries.
- Uses only the Rust standard library (no external dependencies).

## Getting Started

### Prerequisites

Install [Rust and Cargo](https://www.rust-lang.org/tools/install). This project uses **Rust edition 2024**, so use a compatible Rust toolchain.

### Installation

Clone the repository and enter the project directory:

```bash
git clone https://github.com/arq7arq/sudoku-solver.git
cd sudoku-solver
```

### Run the solver

The repository includes a sample puzzle in `board.txt`:

```bash
cargo run -- board.txt
```

You can also solve another puzzle by passing the path to its file:

```bash
cargo run -- path/to/puzzle.txt
```

> **Important:** The input filename is required. The program expects a readable puzzle file and does not currently provide a usage message when the argument is missing.

## Puzzle Format

Input files must contain **9 lines**, each with **9 comma-separated integers**:

- `1`–`9` represent known numbers.
- `0` represents an empty cell.

Example (`board.txt`):

```text
3,0,8,0,0,0,0,9,0
6,0,0,0,0,0,5,0,0
0,4,0,0,0,7,0,0,0
0,5,1,0,0,9,0,0,0
0,0,0,0,0,0,0,0,0
4,0,9,8,1,0,0,0,6
0,1,0,3,0,8,0,0,0
0,0,6,0,5,0,0,0,0
5,0,0,4,7,1,0,0,3
```

The solver prints the resulting board in a terminal-friendly layout with lines separating each 3×3 box.

## How It Works

The solver uses **depth-first search with backtracking**:

1. Find the first empty cell (`0`) on the board.
2. Try numbers `1` through `9` in that cell.
3. For each number, check that it does not already occur in the same row, column, or 3×3 box.
4. If the number is valid, place it and recursively solve the remaining cells.
5. If a choice leads to a dead end, reset the cell to `0` and try the next number.
6. If no empty cells remain, a solution has been found.

### Main Functions

| Function | Purpose |
| --- | --- |
| `read_file` | Loads a puzzle from a comma-separated text file. |
| `find_spot` | Locates the next empty cell. |
| `start_box` | Finds the starting index of a 3×3 box. |
| `is_correct` | Checks whether a number can be placed in a cell. |
| `solve` | Uses recursive backtracking to fill the board. |
| `draw` | Prints the board with 3×3 box separators. |

## Project Structure

```text
sudoku-solver/
├── src/
│   └── main.rs       # Puzzle loading, solving, and output
├── board.txt         # Example Sudoku puzzle
├── Cargo.toml        # Rust package configuration
├── Cargo.lock        # Dependency lockfile
└── README.md         # Project documentation
```

## Current Limitations

- Supports only standard 9×9 Sudoku boards.
- Expects correctly formatted input; malformed files may cause the program to panic.
- Does not explicitly validate the original clues for contradictions.
- Stops after finding the first solution, rather than finding all solutions.
- Does not currently report when a puzzle has no solution; it prints the resulting board regardless of the solver's return value.

## Contributing

Suggestions, improvements, and pull requests are welcome. Feel free to open an issue to discuss an idea or bug.

## Author

Created by [arq7arq](https://github.com/arq7arq).
