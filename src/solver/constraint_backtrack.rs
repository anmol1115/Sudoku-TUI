use std::collections::HashMap;

use super::available_entries;
use crate::solver::backtrack;

pub fn solve(problem: &mut Vec<Vec<char>>) -> bool {
    solve_naked_singles(problem);
    solve_hidden_singles(problem);

    backtrack::solve(problem)
}

fn solve_naked_singles(problem: &mut Vec<Vec<char>>) {
    let mut to_remove = Vec::new();
    let map = available_entries(problem);
    for (coords, possible_entries) in map.iter() {
        if possible_entries.len() == 1 {
            problem[coords.0][coords.1] = possible_entries[0];
            to_remove.push(*coords);
        }
    }
}

fn solve_hidden_singles(problem: &mut Vec<Vec<char>>) {
    // solve for row
    let map = available_entries(problem);
    let mut to_remove = Vec::new();
    for r in 0..9 {
        let mut count = HashMap::new();
        for c in 0..9 {
            if let Some(values) = map.get(&(r, c)) {
                for value in values {
                    count.entry(value).or_insert(vec![]).push((r, c));
                }
            }
        }

        for (val, coords) in count.iter() {
            if coords.len() == 1 {
                let coord = coords[0];
                problem[coord.0][coord.1] = **val;
                to_remove.push(coord);
            }
        }
    }

    // solve for col
    let map = available_entries(problem);
    let mut to_remove = Vec::new();
    for c in 0..9 {
        let mut count = HashMap::new();
        for r in 0..9 {
            if let Some(values) = map.get(&(r, c)) {
                for value in values {
                    count.entry(value).or_insert(vec![]).push((r, c));
                }
            }
        }

        for (val, coords) in count.iter() {
            if coords.len() == 1 {
                let coord = coords[0];
                problem[coord.0][coord.1] = **val;
                to_remove.push(coord);
            }
        }
    }

    // solve for box
    let map = available_entries(problem);
    let mut to_remove = Vec::new();
    let ends = vec![(0, 3), (3, 6), (6, 9)];
    for rows in &ends {
        for cols in &ends {
            let mut count = HashMap::new();
            for r in rows.0..rows.1 {
                for c in cols.0..cols.1 {
                    if let Some(values) = map.get(&(r, c)) {
                        for value in values {
                            count.entry(value).or_insert(vec![]).push((r, c));
                        }
                    }
                }
            }
            for (val, coords) in count.iter() {
                if coords.len() == 1 {
                    let coord = coords[0];
                    problem[coord.0][coord.1] = **val;
                    to_remove.push(coord);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_constraint_backtrack_solve() {
        let problem =
            "65.982............93....4.8........4......12..8..3..9.7.64..2......1......18.7..6";
        let solution =
        "654982317178345962932761458513629874469578123287134695796453281845216739321897546";
        let mut problem = problem
            .chars()
            .collect::<Vec<char>>()
            .chunks(9)
            .map(|chunk| chunk.to_vec())
            .collect();

        solve(&mut problem);
        let mut flag = true;
        for r in 0..9 {
            for c in 0..9 {
                if problem[r][c] != solution.as_bytes()[r * 9 + c] as char {
                    flag = false;
                    break;
                }
            }
        }

        assert!(flag);
    }
}
