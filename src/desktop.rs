use freedesktop_desktop_entry::{DesktopEntry, Iter, default_paths};
use ratatui::layout::Size;
use ratatui_image::Resize;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::Protocol;
use std::{path::PathBuf, process::Command};

pub struct AppEntry {
    pub name: String,
    pub description: String,
    pub exec: String,
    pub icon: Option<Protocol>,
}

impl AppEntry {
    pub fn launch(&self) -> std::io::Result<()> {
        let clean: Vec<&str> = self
            .exec
            .split_whitespace()
            .filter(|arg| !arg.starts_with('%'))
            .collect();

        if let Some((cmd, args)) = clean.split_first() {
            Command::new(cmd).args(args).spawn()?;
        }
        Ok(())
    }

    pub fn find_icon_path(icon_name: &str) -> Option<PathBuf> {
        if icon_name.is_empty() {
            return None;
        }

        let path = PathBuf::from(icon_name);
        if path.is_absolute() && path.exists() {
            return Some(path);
        }

        freedesktop_icons::lookup(icon_name)
            .with_size(48)
            .with_theme("Papirus")
            .find()
    }
}

pub fn load_desktop_entries(picker: &Picker) -> Vec<AppEntry> {
    let mut apps = Vec::new();
    let locales: &[&str] = &[];
    let icon_size = Size::new(4, 2);

    for path in Iter::new(default_paths()) {
        let Ok(entry) = DesktopEntry::from_path(&path, None::<&[&str]>) else {
            continue;
        };

        if entry.no_display() {
            continue;
        }

        let (Some(name), Some(exec)) = (entry.name(locales), entry.exec()) else {
            continue;
        };

        if name.is_empty() || exec.is_empty() {
            continue;
        }

        let icon_name = entry.icon().unwrap_or_default();

        let protocol_icon = AppEntry::find_icon_path(icon_name)
            .and_then(|p| image::open(p).ok())
            .and_then(|img| picker.new_protocol(img, icon_size, Resize::Fit(None)).ok());

        apps.push(AppEntry {
            name: name.to_string(),
            exec: exec.to_string(),
            description: entry.comment(locales).unwrap_or_default().to_string(),
            icon: protocol_icon,
        });
    }
    apps
}
