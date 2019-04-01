extern crate dbus;
extern crate serde;
extern crate toml;

#[macro_use]
mod util;
mod bus;
mod config;
mod player;

fn main() {
    let config = config::load_config();
    let player = player::MprisPlayer::new(config.player, config.commands);

    if let Err(e) = bus::listen(player) {
        error!("Unable to create dbus connection: {}", e);
    }
}
