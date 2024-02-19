use rand::Rng;
use std::fs;

use crate::errors::Errors;

fn random_from_range(range_size: usize) -> usize {
    let mut rng = rand::thread_rng();

    rng.gen_range(0..range_size) as usize
}

pub fn select_pair(mut difficulty: Option<usize>, random: bool) -> Result<(String, String), Errors<'static>> {
    if random {
        let files = match fs::read_dir("./problems") {
            Ok(files) => files,
            Err(_) => return Err(Errors::DirectoryNotFoundError("problems directory not found make sure the dataset is properly initalized"))
        };

        difficulty = Some(random_from_range(files.count()));
    }

    if let Some(d) = difficulty {
        let content = match fs::read_to_string(format!("./problems/{d}.csv")) {
            Ok(content) => content,
            Err(_) => return Err(Errors::FileReadError("Unable to read the given file"))
        };

        let content: Vec<&str> = content.split("\n").collect();
        let idx = random_from_range(content.len()-1);

        if let Some(line) = content.get(idx) {
            let mut parts = line.split(',');
            
            let problem = parts.next().unwrap().to_string();
            let solution = parts.next().unwrap().to_string();

            return Ok((problem, solution))
        }
    }

    Err(Errors::UnknownError("HOW!"))
}

#[cfg(test)]
mod tests {
    use super::select_pair;

    #[test]
    // fn random_number_gen() {
    //     assert_eq!(5, super::random_from_range(10));
    // }
    fn random_problem_select() {
        println!("{:?}", select_pair(Some(2), false));
        println!("{:?}", select_pair(None, true));
    }
}