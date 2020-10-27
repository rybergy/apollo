# Apollo
An othello player / solver, written in Rust.
Originally implemented as my final project for CS 730: Introduction to Artificial Intelligence.

The paper describing the methods used in this program can be found at `paper.pdf`.


# Building

To build the project, run the bash script `build.sh` in the project root. 
This will build the project and put the executable `apollo` in the project root.


# Running

## Algorithms

Commonly among the documentation you will see references to algorithms and heuristics.
When specified on the command line, they take the following form:

<algorithm>:<heuristic>[:<depth>]

Where algorithm is one of:
 * mini         The generic minimax algorithm.
 * ab           Minimax with alpha-beta pruning.
 * ab-order     Minimax with alpha-beta pruning with move ordering.

Where heuristic is one of:
 * 0                    The zero heuristic.
 * random               The random heuristic.
 * unit                 Measures the difference between the two players' disc counts.
 * weight               Weights positions on the board and takes the total difference between the two players' weights.
 * mobility             Measures the total number of moves the player can take.
 * weight-mobility      Takes into account both weight and mobility.

Where depth is a conditionally required integer; some commands will require it and others will not.

e.g. A depth-suffixed algorithm `mini:weight:5` would use minimax with the weighted heuristic with depth 5.
e.g. A non-depth-suffixed algorithm `ab:mobility` would use alpha-beta with the mobility heuristic.


## Benchmarking

Benchmarking and result generation is performed through the `benchmark` subcommand.
There are two more subcommands under the benchmark command: `winrate` and `performance`.

All benchmarks are performed by first initializing the board with some random number of random moves.

For both `winrate` and `benchmark`:
Command line options `-l` and `-u` set the lower and upper bound, respectively, on the number of random moves.
Command line option `-n` sets the number of trials.


### Winrate

The `winrate` benchmark subcommand benchmarks the winrates of multiple algorithms playing against each other.

Usage:
apollo winrate [-d <depth>] [-l <lower>] [-u <upper>] [-n <number>] <algorithms...>

Command line option `-d` sets the depth of all algorithms only if no algorithms are specified on the command line.
Algorithms are depth-suffixed.


### Performance

The `performance` benchmark subcommand benchmarks the performance of multiple algorithms taking moves at various states throughout the game.
The performance metrics measured are average number of nodes expanded and average search time.

Usage:
apollo benchmark [-d <depth>] [-l <lower>] [-u <upper>] [-n <number>] <algorithms...>

Command line option `-d` sets the maximum depth; performance will automatically test all depth levels from 1 to `depth`.
Algorithms are not depth-suffixed.


## Other utilities

### Sim

To simulate only one game, use the `sim` subcommand. The game board is not initialized with random moves. Algorithm 1 will be the black player, moving first.

Usage:
apollo sim <algorithm1> <algorithm2>


### Play

To play a game against an AI opponent, use the `play` subcommand. The game board is not initialized with random moves.

Use W/A/S/D to move up/left/down/right, then press the spacebar to place a piece in the specified position. The cursor highlight will be blue if that position is a valid move, and red if that position is not a valid move.

The opponent will automatically make a move after the player makes a move.
