
# Sudoku for Terminal

A rust based TUI for the everloved game of Sudoku developed using Ratatui.  Dataset is a subset of the famous [3milling sudoku dataset](https://www.kaggle.com/datasets/radcliffe/3-million-sudoku-puzzles-with-ratings?resource=download).




## Features

- Proper menu to choose a difficulty manually or let randomness decide for you.
- Validate your solution.
- Sudoku solver algorithms to solve the board.


## Solvers

- **Backtracking**: A _Deapth First Search_ algorithm which will recursively fill each empty cell.
- **Constraint Propogation**: Reduce the number of empty cells by pre-filling the cells where only one number can be added, then let the Backtracking algorithm do its work on a relatively filled board.


## Roadmap

- Addition of New algorithms:
    - Simulated Annealing: A _Probabilistic Optimisation_ algorithm, used to solve the board without hardcoding the rules of the game directly. Branch available, optimisation required.

- Improve on current algorithms:
    - Addition of advanced concepts like naked pairs, naked triplets, hidden pairs to Constraint Propogation algorithm.

