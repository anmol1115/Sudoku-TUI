use std::fs;

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
    problem: Option<String>,
    solution: Option<String>,
    pub selected_file: usize,
    pub all_files: Vec<String>,
    pub menu_selection: MenuSelection
}

impl App {
    pub fn new() -> Result<App, Errors<'static>> {
        Ok(App {
            current_screen: CurrentScreen::Menu,
            problem: None,
            solution: None,
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
        self.problem = Some(problem);
        self.solution = Some(solution);

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
}