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

pub fn str_from_command(s: Command) -> String {
    match s {
        Command::Set => "set".to_string(),
        Command::Inc => "inc".to_string(),
        Command::Dec => "dec".to_string(),
        Command::Mute => "mute".to_string(),
        Command::UnMute => "unmute".to_string(),
        Command::ToggleMute => "togglemute".to_string(),
    }
}
