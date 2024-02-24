pub mod backtrack;
pub mod constraint_backtrack;

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
