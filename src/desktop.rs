use std::{fmt::Display, fs, io, path::Path, str::FromStr};

use ini::Ini;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Application(Option<String>),
    Link(String),
    Directory,
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Application(_) => f.write_str("Application"),
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
            "Application" => Ok(Type::Application(None)),
            "Link" => Ok(Type::Link(String::from(""))),
            "Directory" => Ok(Type::Directory),
            _ => Err(ParseTypeError),
        }
    }
}

pub struct Desktop {
    pub name: String,
    pub chars: Vec<char>,
    pub entry_type: Type,
    pub no_display: bool,
}

impl Display for Desktop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl Desktop {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        let ini = match Ini::load_from_file(path) {
            Ok(v) => v,
            Err(_) => return None,
        };
        let desktop_entry = ini.section(Some("Desktop Entry"))?;

        let name = desktop_entry.get("Name")?.to_string();
        let chars: Vec<char> = name.chars().collect();
        let mut app_type = Type::from_str(desktop_entry.get("Type")?).ok()?;
        match app_type {
            Type::Application(_) => {
                app_type = Type::Application(desktop_entry.get("Exec").map(|val| val.to_string()));
            }
            Type::Link(_) => {
                let link = desktop_entry.get("Name")?.to_string();
                app_type = Type::Link(link);
            }
            Type::Directory => todo!(),
        }
        let no_display: bool = desktop_entry
            .get("NoDisplay")
            .unwrap_or("false")
            .parse()
            .unwrap_or(false);

        Some(Self {
            name,
            chars,
            entry_type: app_type,
            no_display,
        })
    }

    pub fn load_from_directory<P: AsRef<Path>>(path: P) -> io::Result<Vec<Self>> {
        let dir_entries: Vec<_> = fs::read_dir(path)?.filter_map(|entry| entry.ok()).collect();
        Ok(dir_entries
            .par_iter()
            .filter_map(|entry| Self::load_from_file(entry.path()))
            .filter(|app| !app.no_display)
            .collect())
    }

    pub fn for_each_in_directory<P: AsRef<Path>, OP: Fn(Self) + Sync + Send>(
        path: P,
        op: OP,
    ) -> io::Result<()> {
        let dir_entries: Vec<_> = fs::read_dir(path)?.filter_map(|entry| entry.ok()).collect();
        dir_entries
            .par_iter()
            .filter_map(|entry| Self::load_from_file(entry.path()))
            .filter(|app| !app.no_display)
            .for_each(op);
        Ok(())
    }
}
