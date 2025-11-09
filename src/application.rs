use std::{fmt::Display, path::Path, str::FromStr};

use ini::Ini;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Application,
    Link(String),
    Directory,
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Application => f.write_str("Application"),
            Type::Link(_) => f.write_str("Link"),
            Type::Directory => f.write_str("Directory"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTypeError;

impl FromStr for Type {
    type Err = ParseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Application" => Ok(Type::Application),
            "Link" => Ok(Type::Link(String::from(""))),
            "Directory" => Ok(Type::Directory),
            _ => Err(ParseTypeError),
        }
    }
}

pub struct Application {
    pub name: String,
    pub chars: Vec<char>,
    pub app_type: Type,
    pub no_display: bool,
}

impl Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl Application {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        let ini = match Ini::load_from_file(path) {
            Ok(v) => v,
            Err(_) => return None,
        };
        let desktop_entry = ini.section(Some("Desktop Entry"))?;

        let name = desktop_entry.get("Name")?.to_string();
        let chars: Vec<char> = name.chars().collect();
        let mut app_type = Type::from_str(desktop_entry.get("Type")?).ok()?;
        if app_type == Type::Link(String::from("")) {
            let link = desktop_entry.get("Name")?.to_string();
            app_type = Type::Link(link);
        }
        let no_display: bool = desktop_entry
            .get("NoDisplay")
            .unwrap_or("false")
            .parse()
            .unwrap_or(false);

        Some(Application {
            name,
            chars,
            app_type,
            no_display,
        })
    }
}
