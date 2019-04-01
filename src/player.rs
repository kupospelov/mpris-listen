use std::io;
use std::io::Write;
use std::process::Command;

use crate::config::{CommandConfig, PlayerConfig};

pub struct MprisPlayer {
    name: String,
    playcmd: CommandLine,
    pausecmd: CommandLine,
    playpausecmd: CommandLine,
    stopcmd: CommandLine,
    nextcmd: CommandLine,
    prevcmd: CommandLine,
}

impl MprisPlayer {
    pub fn new(p: PlayerConfig, c: CommandConfig) -> MprisPlayer {
        MprisPlayer {
            name: p.name,
            playcmd: CommandLine::new(c.play),
            pausecmd: CommandLine::new(c.pause),
            playpausecmd: CommandLine::new(c.playpause),
            stopcmd: CommandLine::new(c.stop),
            prevcmd: CommandLine::new(c.previous),
            nextcmd: CommandLine::new(c.next),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn can_raise(&self) -> bool {
        false
    }

    pub fn can_quit(&self) -> bool {
        false
    }

    pub fn has_track_list(&self) -> bool {
        false
    }

    pub fn can_seek(&self) -> bool {
        false
    }

    pub fn can_control(&self) -> bool {
        true
    }

    pub fn can_play(&self) -> bool {
        self.playcmd.is_set() || self.playpausecmd.is_set()
    }

    pub fn can_pause(&self) -> bool {
        self.pausecmd.is_set() || self.playpausecmd.is_set()
    }

    pub fn can_go_next(&self) -> bool {
        self.nextcmd.is_set()
    }

    pub fn can_go_previous(&self) -> bool {
        self.prevcmd.is_set()
    }

    pub fn raise(&self) {}

    pub fn quit(&self) {}

    pub fn play(&self) {
        self.playcmd.run();
    }

    pub fn pause(&self) {
        self.pausecmd.run();
    }

    pub fn playpause(&self) {
        self.playpausecmd.run();
    }

    pub fn stop(&self) {
        self.stopcmd.run();
    }

    pub fn next(&self) {
        self.nextcmd.run();
    }

    pub fn previous(&self) {
        self.prevcmd.run();
    }
}

struct CommandLine {
    line: Vec<String>,
}

impl CommandLine {
    fn new(s: String) -> CommandLine {
        CommandLine {
            line: s.split_whitespace().map(String::from).collect(),
        }
    }

    fn is_set(&self) -> bool {
        self.line.len() > 0
    }

    fn run(&self) {
        if self.line.len() > 0 {
            let output = Command::new(&self.line[0]).args(&self.line[1..]).output();
            match output {
                Err(e) => eprintln!("Command failed: {}", e),
                Ok(o) => {
                    let _ = io::stdout().write_all(&o.stdout);
                    let _ = io::stderr().write_all(&o.stderr);
                }
            }
        }
    }
}
