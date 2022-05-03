# Nagotiator (WIP)

Nagotiator is a frontend server for nagios.

## Features

- Find hosts and services from "status.dat" file by host_name
- Can be enabled/disabled:
    - notification
    - active_checks


## Usage

```
cargo run -- -c NAGIOS_CMD_PATH -s NAGIOS_STATUS_DAT_PATH -l LOAD_INTERVAL_SEC
```


## API

TODO