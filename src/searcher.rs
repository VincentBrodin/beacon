use nucleo_matcher::{Config, Matcher, Utf32Str};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::application::Application;

pub struct Searcher {
    applications: Vec<Application>,
    matcher: Matcher,
}

impl Searcher {
    pub fn new(apps: Vec<Application>, config: Config) -> Self {
        return Searcher {
            applications: apps,
            matcher: Matcher::new(config),
        };
    }

    pub fn search(&self, str: &str) -> Vec<&Application> {
        let mut buf: Vec<char> = Vec::new();
        let needle = Utf32Str::new(str, &mut buf);
        let mut results: Vec<(&Application, u16)> = self
            .applications
            .par_iter()
            .filter_map(|app| {
                let mut local_matcher = self.matcher.clone();
                let haystack = Utf32Str::Unicode(&app.chars);
                match local_matcher.substring_match(haystack, needle) {
                    Some(score) => Some((app, score)),
                    None => None,
                }
            })
            .collect();
        results.sort_by(|a, b| b.1.cmp(&a.1));
        let apps: Vec<_> = results.par_iter().map(|(app, _)| *app).collect();
        return apps;
    }
}
