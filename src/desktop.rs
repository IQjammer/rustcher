use freedesktop_desktop_entry::{DesktopEntry, Iter, default_paths};
use image::DynamicImage;
use ratatui::layout::Size;
use ratatui_image::{Resize, picker::Picker, protocol::Protocol};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

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
}

fn detect_icon_theme() -> String {
    freedesktop_icons::default_theme_gtk()
        .or_else(theme_from_gtk_settings)
        .unwrap_or_else(|| "hicolor".to_string())
}

fn theme_from_gtk_settings() -> Option<String> {
    let home = std::env::var_os("HOME")?;
    let path = Path::new(&home).join(".config/gtk-3.0/settings.ini");
    let text = std::fs::read_to_string(path).ok()?;
    text.lines()
        .find_map(|l| l.strip_prefix("gtk-icon-theme-name="))
        .map(|s| s.trim().to_string())
}

fn find_icon_path(icon_name: &str, theme: &str) -> Option<PathBuf> {
    if icon_name.is_empty() {
        return None;
    }

    let path = PathBuf::from(icon_name);
    if path.is_absolute() && path.exists() {
        return Some(path);
    }

    freedesktop_icons::lookup(icon_name)
        .with_size(48)
        .with_theme(theme)
        .find()
}

fn load_icon(path: &Path, px: u32) -> Option<DynamicImage> {
    if path.extension().is_some_and(|e| e == "svg") {
        let data = std::fs::read(path).ok()?;
        let tree = resvg::usvg::Tree::from_data(&data, &resvg::usvg::Options::default()).ok()?;

        let size = tree.size();
        let scale = px as f32 / size.width().max(size.height());

        let mut pixmap = resvg::tiny_skia::Pixmap::new(px, px)?;
        resvg::render(
            &tree,
            resvg::tiny_skia::Transform::from_scale(scale, scale),
            &mut pixmap.as_mut(),
        );

        let img = image::RgbaImage::from_raw(px, px, pixmap.take())?;
        Some(DynamicImage::ImageRgba8(img))
    } else {
        image::open(path).ok()
    }
}

pub fn load_desktop_entries(picker: &Picker) -> Vec<AppEntry> {
    let mut apps = Vec::new();
    let locales: &[&str] = &[];
    let icon_size = Size::new(4, 2);
    let theme = detect_icon_theme(); // один раз, вне цикла

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

        let icon = find_icon_path(entry.icon().unwrap_or_default(), &theme)
            .and_then(|p| load_icon(&p, 64))
            .and_then(|img| picker.new_protocol(img, icon_size, Resize::Fit(None)).ok());

        apps.push(AppEntry {
            name: name.to_string(),
            exec: exec.to_string(),
            description: entry.comment(locales).unwrap_or_default().to_string(),
            icon,
        });
    }
    apps
}
