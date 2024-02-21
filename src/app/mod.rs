use std::fs;
use std::collections::HashSet;

use crate::errors::Errors;
mod problem_select;

pub enum CurrentScreen {
    Menu,
    DifficultySelect,
    Playground,
    Exiting
}

pub enum MenuSelection {
    ChooseDifficulty,
    RandomDifficulty
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub problem: Vec<Vec<char>>,
    solution: Option<String>,
    playable_pos: HashSet<(usize, usize)>,
    pub selected_row: usize,
    pub selected_col: usize,
    pub invalid_entries: HashSet<(usize, usize)>,
    pub selected_file: usize,
    pub all_files: Vec<String>,
    pub menu_selection: MenuSelection
}

impl App {
    pub fn new() -> Result<App, Errors<'static>> {
        Ok(App {
            current_screen: CurrentScreen::Menu,
            problem: vec![],
            solution: None,
            playable_pos: HashSet::new(),
            selected_col: 0,
            selected_row: 0,
            invalid_entries: HashSet::new(),
            selected_file: 0,
            all_files: Self::list_problem_files()?,
            menu_selection: MenuSelection::ChooseDifficulty
        })
    }

    pub fn toggle_menu_selection(&mut self) {
        match &self.menu_selection {
            MenuSelection::ChooseDifficulty => {self.menu_selection = MenuSelection::RandomDifficulty},
            MenuSelection::RandomDifficulty => {self.menu_selection = MenuSelection::ChooseDifficulty}
        }
    }

    pub fn set_pair(&mut self, difficulty: Option<usize>, random: bool) -> Result<(), Errors> {
        let (problem, solution) = problem_select::select_pair(difficulty, random)?;
        self.solution = Some(solution);
        self.problem = problem
            .chars()
            .collect::<Vec<char>>()
            .chunks(9)
            .map(|chunk| chunk.to_vec())
            .collect();

        for r in 0..9 {
            for c in 0..9 {
                if self.problem[r][c] == '.' {
                    self.playable_pos.insert((r, c));
                }
            }
        }

        self.current_screen = CurrentScreen::Playground;

        Ok(())
    }

    fn list_problem_files() -> Result<Vec<String>, Errors<'static>> {
        let mut out = Vec::new();

        let files = match fs::read_dir("./problems") {
            Ok(files) => files,
            Err(_) => return Err(Errors::DirectoryListError("Unable to list problems directory"))
        };

        for file in files {
            if let Ok(file) = file {
                if let Ok(file_name) = file.file_name().into_string() {
                    out.push(file_name);
                }
            }
        }

        Ok(out)
    }

    pub fn increment_difficulty(&mut self) {
        let mut new_idx = self.selected_file + 1;
        if new_idx == self.all_files.len() {
            new_idx = 0;
        }
        self.selected_file = new_idx;
    }

    pub fn decrement_difficulty(&mut self) {
        if self.selected_file == 0 {
            self.selected_file = self.all_files.len();
        }

        let new_idx = self.selected_file - 1;
        self.selected_file = new_idx;

    }

    pub fn select_difficulty(&mut self) -> Result<(), Errors> {
        let difficulty = self.all_files[self.selected_file].as_bytes()[0] as char;
        if let Some(difficulty) = difficulty.to_digit(10) {
            self.set_pair(Some(difficulty as usize), false)?;
        }

        Ok(())
    }

    pub fn move_cursor(&mut self, row: i32, column: i32) {
        match row {
            1 => {
                let new_row = self.selected_row + 1;
                if new_row == 9 {
                    self.selected_row = 0;
                    return
                }
                self.selected_row = new_row;
            },
            -1 => {
                if self.selected_row == 0 {
                    self.selected_row = 9;
                }
                self.selected_row -= 1;
            },
            _ => ()
        }
        match column {
            1 => {
                let new_col = self.selected_col + 1;
                if new_col == 9 {
                    self.selected_col = 0;
                    return
                }
                self.selected_col = new_col;
            },
            -1 => {
                if self.selected_col == 0 {
                    self.selected_col = 9;
                }
                self.selected_col -= 1;
            },
            _ => ()
        }
    }

    pub fn set_value(&mut self, value: char) {
        if self.playable_pos.contains(&(self.selected_row, self.selected_col)) {
            self.problem[self.selected_row][self.selected_col] = value;
        }
    }

    pub fn validate(&mut self) {
        for r in 0..9 {
            for c in 0..9 {
                if self.problem[r][c] != '.' && self.problem[r][c] != self.solution.as_ref().unwrap().as_bytes()[r * 9+ c] as char {
                    self.invalid_entries.insert((r, c));
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.invalid_entries.clear();
    }
}