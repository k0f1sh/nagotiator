# Nagotiator(WIP)

Nagotiator is a frontend server for nagios.

## features

- Find hosts and services from "status.dat" file by host_name
- Can be enabled/disabled:
    - notification
    - active_checks


## Usage

```
cargo run -- -c NAGIOS_CMD_PATH -s NAGIOS_STATUS_DAT_PATH -m MAX_CACHE_SEC
```
