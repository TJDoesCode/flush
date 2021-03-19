use std::sync::Mutex;

pub mod commands;

#[derive(Debug, Default)]
pub struct ProgState {
    pub kill_count: i8,
    pub child_running: bool,
}

impl ProgState {
    pub fn new() -> Mutex<ProgState> {
        Mutex::new(ProgState::default())
    }

    pub fn increase_check(&mut self, threshold: i8) -> bool {
        self.kill_count += 1;
        println!("{}", self.kill_count);

        if self.kill_count > threshold {
            return true;
        }
        false
    }

    pub fn set_child_running(&mut self, new: bool) -> bool {
        self.child_running = new;

        self.child_running
    }
}
