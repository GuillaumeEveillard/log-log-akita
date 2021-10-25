use std::path::PathBuf;

struct Engine {
    files: Vec<File>,
    filters: Vec<Box<dyn Filter>>
}

struct File {
    path: PathBuf,
    lines: Vec<String>,
    filters: Vec<Box<dyn Filter>>
}

trait Filter {
    fn keep(&self, line: &str) -> bool;
}

struct PatternFilter {
    mode: FilterMode,
    pattern: String
}

impl PatternFilter {
    fn new(mode: FilterMode, pattern: &str) {
        PatternFilter(mode: mode, pattern: pattern)
    }
}

enum FilterMode {Includes, Excludes}

impl Filter for PatternFilter {
    fn keep(&self, line: &str) -> bool {
        return match &self.mode {
            FilterMode::Includes => { line.contains(&self.pattern) },
            FilterMode::Excludes => { !line.contains(&self.pattern) },
        };
    }
}