fn possible_sol(r: usize, c: usize, n: char, grid: &Vec<Vec<char>>) -> bool {
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

pub fn solve(problem: &mut Vec<Vec<char>>) -> bool {
    for r in 0..9 {
        for c in 0..9 {
            if problem[r][c] == '.' {
                for i in 1..=9 {
                    let i = std::char::from_digit(i as u32, 10).expect("");
                    if possible_sol(r, c, i, problem) {
                        problem[r][c] = i;
                        if solve(problem) {
                            return true;
                        }
                        problem[r][c] = '.';
                    }
                }
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_backtrack_solve() {
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
