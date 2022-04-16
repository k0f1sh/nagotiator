use std::sync::Mutex;

pub type Nagrs = nagrs::Nagrs<String>;

pub struct State {
    pub nagrs: Mutex<Nagrs>,
}

impl State {
    pub fn new(command_file_path: &str, status_file_path: &str, max_cache_sec: usize) -> State {
        State {
            nagrs: Mutex::new(Nagrs::new(
                command_file_path.to_string(),
                status_file_path.to_string(),
                max_cache_sec,
            )),
        }
    }
}
