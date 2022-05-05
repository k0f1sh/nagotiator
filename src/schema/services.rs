use std::collections::HashMap;

use nagrs::nagios::object::Service;

pub type Services = HashMap<String, Vec<Service>>;
