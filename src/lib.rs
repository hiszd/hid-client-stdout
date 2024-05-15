pub mod util;
use util::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Messages {
    LayerChanged(u16),
    Volume(
        hid_io_client::keyboard_capnp::keyboard::signal::volume::Command,
        u16,
        Option<String>,
    ),
}

impl TryFrom<&str> for Messages {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use hid_io_client::keyboard_capnp::keyboard::signal::volume::Command;
        let pts = s.split(':').collect::<Vec<&str>>();
        if pts.len() == 2 {
            match pts[0].to_lowercase().as_str() {
                "layerchanged" => Ok(Messages::LayerChanged(pts[1].parse().unwrap())),
                "volume" => {
                    let vol_pts = pts[1]
                        .to_lowercase()
                        .as_str()
                        .split(',')
                        .map(|m| m.to_owned())
                        .collect::<Vec<String>>();
                    if vol_pts.len() == 2 {
                        let cmd: Command = command_from_str(&vol_pts[0]).unwrap();
                        let vol: u16 = vol_pts[1].parse().unwrap();
                        Ok(Messages::Volume(cmd, vol, None))
                    } else if vol_pts.len() == 3 {
                        let cmd: Command = command_from_str(&vol_pts[0]).unwrap();
                        let vol: u16 = vol_pts[1].parse().unwrap();
                        let app: String = vol_pts[2].to_owned();
                        Ok(Messages::Volume(cmd, vol, Some(app)))
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

impl<'a> From<Messages> for String {
    fn from(m: Messages) -> Self {
        match m {
            Messages::LayerChanged(l) => {
                format!("layerchanged:{}", l)
            }
            Messages::Volume(c, v, a) => format!(
                "volume:{},{},{}",
                str_from_command(c),
                v,
                match a {
                    Some(a) => a,
                    None => "".to_string(),
                }
            ),
        }
    }
}
