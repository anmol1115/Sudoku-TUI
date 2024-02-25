use std::collections::HashMap;

pub mod backtrack;
pub mod constraint_backtrack;
pub mod simulated_annealing;

pub fn possible_sol(r: usize, c: usize, n: char, grid: &Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        if grid[i][c] == n {
            return false;
        }
    }

    for i in 0..9 {
        if grid[r][i] == n {
            return false;
        }
    }

    let r0 = (r / 3) * 3;
    let c0 = (c / 3) * 3;

    for i in 0..3 {
        for j in 0..3 {
            if grid[r0 + i][c0 + j] == n {
                return false;
            }
        }
    }

    true
}

pub fn available_entries(problem: &Vec<Vec<char>>) -> HashMap<(usize, usize), Vec<char>> {
    let mut res = HashMap::new();
    for r in 0..9 {
        for c in 0..9 {
            if problem[r][c] == '.' {
                for i in 1..=9 {
                    let i = std::char::from_digit(i as u32, 10).expect("");
                    if possible_sol(r, c, i, problem) {
                        let values = res.entry((r, c)).or_insert(vec![]);
                        values.push(i);
                    }
                }
            }
        }
    }
    res
}
