# mpris-listen
mpris-listen is a general purpose user service which implements a subset of the MPRIS2 interface and delegates the processing to external programs.

## Usage
The service needs a configuration file to run. The file describes parameters and commands to be executed:

```toml
[player]
name = "mpris_mpc"

[commands]
previous = "mpc prev"
playpause = "mpc toggle"
next = "mpc next"
```

Run `man 5 mpris-listen` for the full list of options.

## Notes
The service does emit any signals, so clients which rely on them may show incorrect or outdated information.
