mod problem_select;

pub enum CurrentScreen {
    Menu,
    Playground,
    Exiting
}

pub enum MenuSelection {
    ChooseDifficulty,
    RandomDifficulty
}

pub struct App<'a> {
    pub current_screen: CurrentScreen,
    problem: &'a str,
    solution: &'a str,
    pub menu_selection: MenuSelection
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            current_screen: CurrentScreen::Menu,
            problem: "",
            solution: "",
            menu_selection: MenuSelection::ChooseDifficulty
        }
    }

    pub fn toggle_menu_selection(&mut self) {
        match &self.menu_selection {
            MenuSelection::ChooseDifficulty => {self.menu_selection = MenuSelection::RandomDifficulty},
            MenuSelection::RandomDifficulty => {self.menu_selection = MenuSelection::ChooseDifficulty}
        }
    }
}