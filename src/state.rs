use std::sync::Mutex;

pub type Nagrs = nagrs::Nagrs<String>;

pub struct State {
    pub nagrs: Mutex<Nagrs>,
}

impl State {
    pub fn new() -> State {
        State {
            nagrs: Mutex::new(Nagrs::new(
                "./docker/var/rw/nagios.cmd".to_string(),
                "./docker/var/status.dat".to_string(),
                10,
            )),
        }
    }
}
