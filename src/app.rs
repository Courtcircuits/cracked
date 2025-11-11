use crate::challenge::Challenge;

pub struct App {
    pub challenges: Vec<Challenge>,
    pub selected_index: usize,
    pub status_message: String,
    pub should_quit: bool,
    pub scroll_offset: usize,
    pub should_download: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            challenges: Vec::new(),
            selected_index: 0,
            status_message: String::from("Loading challenges..."),
            should_quit: false,
            scroll_offset: 0,
            should_download: false,
        }
    }

    pub fn next_challenge(&mut self) {
        if !self.challenges.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.challenges.len();
        }
    }

    pub fn previous_challenge(&mut self) {
        if !self.challenges.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.challenges.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn get_selected_challenge(&self) -> Option<&Challenge> {
        self.challenges.get(self.selected_index)
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn set_challenges(&mut self, challenges: Vec<Challenge>) {
        self.challenges = challenges;
        self.selected_index = 0;
        self.scroll_offset = 0;
        if self.challenges.is_empty() {
            self.status_message = String::from("No challenges found");
        } else {
            self.status_message = format!("{} challenges found", self.challenges.len());
        }
    }

    pub fn set_status(&mut self, message: String) {
        self.status_message = message;
    }

    pub fn trigger_download(&mut self) {
        self.should_download = true;
    }

    pub fn reset_download_flag(&mut self) {
        self.should_download = false;
    }
}
