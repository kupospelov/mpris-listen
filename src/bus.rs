use std::rc::Rc;

use dbus::tree::{Factory, MTFn, Tree};
use dbus::{BusType, Connection};

use crate::player::MprisPlayer;

pub fn listen(p: MprisPlayer) -> Result<(), dbus::Error> {
    let con = Connection::get_private(BusType::Session)?;
    con.register_name(&format!("org.mpris.MediaPlayer2.{}", p.get_name()), 0)?;

    let tree = create_dbus_tree(Rc::new(p));
    tree.set_registered(&con, true)?;
    con.add_handler(tree);

    loop {
        con.incoming(1000).next();
    }
}

fn create_dbus_tree(p: Rc<MprisPlayer>) -> Tree<MTFn, ()> {
    macro_rules! property {
        ($t:ty, $f:expr, $name:expr, $p:ident, $flag:expr) => {{
            let $p = p.clone();
            $f.property::<$t, _>($name, ()).on_get(move |m, _| {
                m.append($flag);
                Ok(())
            })
        }};
    }

    macro_rules! method {
        ($f:expr, $name:expr, $p:ident, $cmd:expr) => {{
            let $p = p.clone();
            $f.method($name, (), move |m| {
                $cmd;
                Ok(vec![m.msg.method_return()])
            })
        }};
    }

    let f = Factory::new_fn::<()>();
    let root_interface = f
        .interface("org.mpris.MediaPlayer2", ())
        .add_p(property!(bool, f, "CanRaise", p, p.can_raise()))
        .add_p(property!(bool, f, "CanQuit", p, p.can_quit()))
        .add_p(property!(bool, f, "HasTrackList", p, p.has_track_list()))
        .add_p(property!(String, f, "Identity", p, p.get_name()))
        .add_m(method!(f, "Raise", p, p.raise()))
        .add_m(method!(f, "Quit", p, p.quit()));

    let player_interface = f
        .interface("org.mpris.MediaPlayer2.Player", ())
        .add_p(property!(bool, f, "CanSeek", p, p.can_seek()))
        .add_p(property!(bool, f, "CanControl", p, p.can_control()))
        .add_p(property!(bool, f, "CanPlay", p, p.can_play()))
        .add_p(property!(bool, f, "CanPause", p, p.can_pause()))
        .add_p(property!(bool, f, "CanGoPrevious", p, p.can_go_previous()))
        .add_p(property!(bool, f, "CanGoNext", p, p.can_go_next()))
        .add_m(method!(f, "Play", p, p.play()))
        .add_m(method!(f, "Pause", p, p.pause()))
        .add_m(method!(f, "PlayPause", p, p.playpause()))
        .add_m(method!(f, "Stop", p, p.stop()))
        .add_m(method!(f, "Previous", p, p.previous()))
        .add_m(method!(f, "Next", p, p.next()));

    f.tree(()).add(
        f.object_path("/org/mpris/MediaPlayer2", ())
            .introspectable()
            .add(root_interface)
            .add(player_interface),
    )
}
