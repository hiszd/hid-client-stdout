use hid_io_client::keyboard_capnp::keyboard::signal::volume::Command;

pub fn command_from_str(s: &str) -> Result<Command, ()> {
    match s {
        "set" => Ok(Command::Set),
        "inc" => Ok(Command::Inc),
        "dec" => Ok(Command::Dec),
        "mute" => Ok(Command::Mute),
        "unmute" => Ok(Command::UnMute),
        "togglemute" => Ok(Command::ToggleMute),
        _ => Err(()),
    }
}

pub fn str_from_command(s: &Command) -> &str {
    match s {
        Command::Set => "set",
        Command::Inc => "inc",
        Command::Dec => "dec",
        Command::Mute => "mute",
        Command::UnMute => "unmute",
        Command::ToggleMute => "togglemute",
    }
}
