#[allow(dead_code)]
enum Messages {
    LayerChanged(u16),
    Volume(
        hid_io_client::keyboard_capnp::keyboard::signal::volume::Command,
        u16,
        Option<&'static str>,
    ),
}

impl TryFrom<&str> for Messages {
    fn try_from(s: &str) -> Self {
        let pts = s.split(':').collect::<Vec<&str>>();
        if pts.len() == 2 {
            match pts[0].to_lowercase().as_str() {
                "layerchanged" => Messages::LayerChanged(pts[1].parse().unwrap()),
                "volume" => {
                    let vol_pts = pts[1].split(',').collect::<Vec<&str>>();
                    if vol_pts.len() == 2 {}
                }
            }
        } else {
            Err("Invalid message")
        }
    }
}
