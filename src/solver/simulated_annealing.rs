use rand::Rng;
use std::f64::consts;
use std::collections::HashSet;

pub fn solve(problem: &mut Vec<Vec<char>>) {
    simulated_annealing(problem, 0.99);
    println!("{:?}", calculate_energy(problem));
}

fn simulated_annealing(problem: &mut Vec<Vec<char>>, cooling_rate: f64) -> bool {
    let available = available_entries(problem);
    fill_random(problem);
    let mut curr_energy = calculate_energy(problem);

    let mut temp = 10000.0;
    let final_temp = 0.0001;

    while temp > final_temp {
        // println!("{:?}, {:?}", curr_energy, temp);
        let new_problem = swap_neighbors(problem, &available);
        let new_energy = calculate_energy(&new_problem);

        let energy_diff = curr_energy - new_energy;
        if energy_diff <= 0 {
            *problem = new_problem;
            curr_energy = new_energy;
        } else {
            let probability = consts::E.powf(-1.0 * (new_energy as f64)/temp);
            if random_uniform() > probability {
                *problem = new_problem;
                curr_energy = new_energy;
            }
        }

        temp = temp * cooling_rate;
    }

    true
}

fn swap_neighbors(problem: &Vec<Vec<char>>, map: &HashSet<(usize, usize)>) -> Vec<Vec<char>> {
    let (n_row_1, n_col_1, n_row_2, n_col_2) = random_neighbor_select(map);
    let mut neighbor = problem.clone();

    let temp = neighbor[n_row_1][n_col_1];
    neighbor[n_row_1][n_col_1] = neighbor[n_row_2][n_col_2];
    neighbor[n_row_2][n_col_2] = temp;

    neighbor
}

fn calculate_energy(problem: &Vec<Vec<char>>) -> i32 {
    let mut energy = 0;

    for r in 0..9 {
        let mut unique = HashSet::new();
        for c in 0..9 {
            if problem[r][c] == '.' {
                energy += 1;
                continue;
            }
            if unique.contains(&problem[r][c]) {
                energy += 1;
            } else {
                unique.insert(problem[r][c]);
            }
        }
    }

    for c in 0..9 {
        let mut unique = HashSet::new();
        for r in 0..9 {
            if problem[r][c] == '.' {
                energy += 1;
                continue;
            }
            if unique.contains(&problem[r][c]) {
                energy += 1;
            } else {
                unique.insert(problem[r][c]);
            }
        }
    }

    energy
}

fn random_uniform() -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen()
}

fn random_from_range(min: usize, max: usize) -> usize {
    let mut rng =  rand::thread_rng();

    rng.gen_range(min..max)
}

fn random_neighbor_select(map: &HashSet<(usize, usize)>) -> (usize, usize, usize, usize) {
    let mut flag = true;
    let (mut cell_row_1, mut cell_col_1, mut cell_row_2, mut cell_col_2) = (0, 0, 0, 0);

    while flag {
        flag = false;

        let box_row = random_from_range(0, 3);
        let box_col = random_from_range(0, 3);

        cell_row_1 = random_from_range(box_row*3, box_row*3+3);
        cell_col_1 = random_from_range(box_col*3, box_col*3+3);
        cell_row_2 = random_from_range(box_row*3, box_row*3+3);
        cell_col_2 = random_from_range(box_col*3, box_col*3+3);

        if !map.contains(&(cell_row_1, cell_col_1)) || map.contains(&(cell_row_2, cell_col_2)) || (cell_row_1 == cell_row_2 && cell_col_1 == cell_col_2) {
            flag = true;
        }
    }

    (cell_row_1, cell_col_1, cell_row_2, cell_col_2)
}

fn possible_sol_box(r: usize, c: usize, n: char, grid: &Vec<Vec<char>>) -> bool {
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

fn fill_random(problem: &mut Vec<Vec<char>>) {
    for r in 0..9 {
        for c in 0..9 {
            if problem[r][c] == '.' {
                for i in 1..=9 {
                    let i = std::char::from_digit(i as u32, 10).expect("");
                    if possible_sol_box(r, c, i, problem) {
                        problem[r][c] = i;
                    }
                }
            }
        }
    }
}

fn available_entries(problem: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut res = HashSet::new();
    for r in 0..9 {
        for c in 0..9 {
            if problem[r][c] == '.' {
                res.insert((r, c));
            }
        }
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entries() {
        let problem = "65.982............93....4.8........4......12..8..3..9.7.64..2......1......18.7..6";
        let mut problem = problem
            .chars()
            .collect::<Vec<char>>()
            .chunks(9)
            .map(|chunk| chunk.to_vec())
            .collect();

        fill_random(&mut problem);

        for line in problem {
            println!("{:?}", line);
        }
    }

    #[test]
    fn test_solve() {
        let problem = "65.982............93....4.8........4......12..8..3..9.7.64..2......1......18.7..6";
        let mut problem = problem
            .chars()
            .collect::<Vec<char>>()
            .chunks(9)
            .map(|chunk| chunk.to_vec())
            .collect();

        solve(&mut problem)
    }
}
