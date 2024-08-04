#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MainMenuOptions {
    Campaign,
    FastMatch,
    Multiplayer,
    Options,
    Test,
    Reset,
}

impl MainMenuOptions {
    fn as_str(&self) -> &str {
        match self {
            MainMenuOptions::Campaign => "Campaña",
            MainMenuOptions::FastMatch => "Partida rapida",
            MainMenuOptions::Multiplayer => "Multijugador",
            MainMenuOptions::Options => "Opciones",
            MainMenuOptions::Test => "Test",
            MainMenuOptions::Reset => "Reset",
        }
    }
}

pub struct MainMenuState<'a> {
    pub main_menu_options: Vec<MainMenuOptions>,
    tabs_titles: Vec<&'a str>,
    pub option_selected: usize,
    pub tab_selected: usize,
}

impl<'a> MainMenuState<'a> {
    pub fn new() -> Self {
        Self {
            main_menu_options: vec![
                MainMenuOptions::Campaign,
                MainMenuOptions::FastMatch,
                MainMenuOptions::Multiplayer,
                // MainMenuOptions::Options,
                // MainMenuOptions::Test,
                // MainMenuOptions::Reset,
            ],
            tabs_titles: vec!["Menu principal", "Opciones"],
            option_selected: 0,
            tab_selected: 0,
        }
    }

    pub fn next_tab(&mut self) {
        self.tab_selected = (self.tab_selected + 1) % self.tabs_titles.len();
    }

    pub fn previous_tab(&mut self) {
        self.tab_selected =
            (self.tab_selected + self.tabs_titles.len() - 1) % self.tabs_titles.len();
    }

    fn next_mm_option(&mut self) {
        self.option_selected = (self.option_selected + 1) % self.main_menu_options.len();
    }

    fn previous_mm_option(&mut self) {
        self.option_selected = (self.option_selected + self.main_menu_options.len() - 1)
            % self.main_menu_options.len();
    }

    pub fn selected_mm_option(&self) -> MainMenuOptions {
        self.main_menu_options[self.option_selected]
    }

    pub fn next(&mut self) {
        match self.tab_selected {
            0 => self.next_mm_option(),
            1 => {}
            _ => {}
        }
    }

    pub fn previous(&mut self) {
        match self.tab_selected {
            0 => self.previous_mm_option(),
            1 => {}
            _ => {}
        }
    }

    pub fn tabs_titles_iter(&self) -> impl Iterator<Item = &&str> {
        self.tabs_titles.iter()
    }

    pub fn main_menu_options_iter(&self) -> impl Iterator<Item = &MainMenuOptions> {
        self.main_menu_options
            .iter()
    }
}