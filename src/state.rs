use std::sync::Mutex;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use nagrs::nagios::NagiosStatus;

pub type Nagrs = nagrs::Nagrs<String>;

pub struct State {
    pub nagrs: Nagrs,
    pub max_cache_sec: usize,
    pub is_parsing: Mutex<bool>,
    pub cached_state: Mutex<Option<CachedState>>,
}

pub struct CachedState {
    pub nagios_status: NagiosStatus,
    pub cached_at: DateTime<Utc>,
}

impl State {
    pub fn new(command_file_path: &str, status_file_path: &str, max_cache_sec: usize) -> State {
        State {
            nagrs: Nagrs::new(command_file_path.to_string(), status_file_path.to_string()),
            max_cache_sec,
            is_parsing: Mutex::new(false),
            cached_state: Mutex::new(None),
        }
    }

    pub fn load(&self) -> Result<()> {
        self.parse_start()?;
        // if status.dat is a large file, parse() would take a lot of time.
        let parsed = self.nagrs.parse();
        self.parse_stop();
        match parsed {
            Err(error) => Err(error),
            Ok(nagios_status) => {
                let mut cached_state = self.cached_state.lock().unwrap();
                let cached_at = Utc::now();
                *cached_state = Some(CachedState {
                    nagios_status,
                    cached_at,
                });
                Ok(())
            }
        }
    }

    fn parse_start(&self) -> Result<()> {
        let mut is_parsing = self.is_parsing.lock().unwrap();
        if *is_parsing {
            return Err(anyhow!("can not start to parse"));
        }
        *is_parsing = true;

        Ok(())
    }

    fn parse_stop(&self) -> () {
        let mut is_parsing = self.is_parsing.lock().unwrap();
        *is_parsing = false;
    }
}
