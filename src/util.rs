//! hyprpaper model — Hyprland wallpaper daemon.

#[derive(Debug, Clone)]
pub struct Wallpaper {
    pub path: String,
    pub monitor: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub wallpapers: Vec<Wallpaper>,
}

impl Default for Model {
    fn default() -> Self { Self { wallpapers: vec![
        Wallpaper { path: "/usr/share/backgrounds/sigma.png".into(), monitor: "eDP-1".into(), enabled: true },
    ] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }
    pub fn list(&self) -> &[Wallpaper] { &self.wallpapers }
    pub fn set(&mut self, monitor: &str, path: &str) -> bool {
        if let Some(w) = self.wallpapers.iter_mut().find(|w| w.monitor == monitor) {
            w.path = path.into(); true
        } else {
            self.wallpapers.push(Wallpaper { path: path.into(), monitor: monitor.into(), enabled: true });
            true
        }
    }
    pub fn reload(&self) -> bool { true }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_reload() {
        let mut h = Model::new();
        assert!(h.set("HDMI-A-1", "/tmp/wall.png"));
        assert_eq!(h.list().len(), 2);
        assert!(h.reload());
    }
}
