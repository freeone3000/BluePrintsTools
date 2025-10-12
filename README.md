# Blue Prince Tools

A collection of tools to ~~cheat~~ solve puzzles easily in the game Blue Prince.
I wrote these tools for my own use, since I wanted to solve the puzzles without
external help, but found a certain part quite tedious. As a programmer, anything
that I crafted with my hands counts as "doing it myself", therefore, I automated
it.

## Tools
### Mora Jai Solver
Running: `cargo run --package blue-prince --bin mora_jai_solver`
- A tool to solve the Mora Jai puzzles in blue prince.
  - It has helpful input as you type it in. 
### Numeric Core Solver
Running: `cargo run --package blue-prince --bin numeric_core_solver`
- A tool to solve a specific puzzle in blue prince based on numeric core calculation.
- For inscrutable reasons, this accepts input as words and will output letters. `%!` will be output if no letter matches, often because an input was mistyped.
