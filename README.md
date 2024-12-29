# AOC 2024
My solutions for Advent of Code 2024, 50 stars, in Rust.

# Layout
## Utils
`src/` contains library code for a few utilities I built up over the course of the month.

These utilities include:
- Input management
- A custom graph library
- Utilities for working with types of the form `HashMap<K, Vec<V>>`
- A Counter type (developed before I started using Itertools)

## Days
Each day has its own subdirectory and is its own binary. Days where Part B required major refactoring work
get their own DayNb project instead.
