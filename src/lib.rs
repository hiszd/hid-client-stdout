pub mod util;
use std::fmt::Display;

use hid_io_client::capnp::traits::FromStructBuilder;
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

#[derive(Debug, Clone)]
pub struct Node {
    pub r#type: String,
    pub name: String,
    pub serial: String,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.r#type, self.name, self.serial)
    }
}

impl TryFrom<String> for Node {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let pts = s.split(':').collect::<Vec<&str>>();
        if pts.len() == 3 {
            Ok(Node {
                r#type: pts[0].to_string(),
                name: pts[1].to_string(),
                serial: pts[2].to_string(),
            })
        } else {
            Err(())
        }
    }
}

pub fn stdout_from_node(node: hid_io_core::common_capnp::destination::Reader<'_>) -> String {
    let name_parts = node
        .get_name()
        .unwrap_or("")
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    format!(
        "{}:{}:{}",
        node.get_type().unwrap(),
        name_parts[2],
        node.get_serial().unwrap_or(""),
    )
}
