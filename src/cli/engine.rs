use std::path::PathBuf;
use std::path::Path;

pub struct Engine {
    files: Vec<File>,
    filters: Vec<Box<dyn Filter>>
}

impl Engine {
    pub fn new<P: AsRef<Path>>(files: Vec<P>, filters: Vec<Box<dyn Filter>>) -> Engine {
        let ff = files.into_iter().map(|f| File::new(f)).collect();
        Engine{files: ff, filters: filters}
    }

    pub fn all_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        self.files.iter()
            .for_each(|f| lines.extend(f.lines.iter().filter(|l| self.keep(l)).cloned()));
        lines
    }

    fn keep(&self, line: &str) -> bool {
        self.filters.iter().all(|f| f.keep(line))
    }
}

struct File {
    path: PathBuf,
    lines: Vec<String>,
    filters: Vec<Box<dyn Filter>>
}

impl File {
    fn new<P: AsRef<Path>>(path: P) -> File {
        let lines = File::lines_from_file(&path);
        let p = path.as_ref();
        let mut pb = PathBuf::new();
        pb.push(p);
        File{path: pb, lines, filters: Vec::new()}
    }

    fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
        use std::{
            fs::File,
            io::{prelude::*, BufReader}
        };

        let file = File::open(filename).expect("no such file");
        BufReader::new(file)
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }
}

pub trait Filter : std::fmt::Debug {
    fn keep(&self, line: &str) -> bool;
}

#[derive(Debug)]
pub struct PatternFilter {
    mode: FilterMode,
    pattern: String
}

impl PatternFilter {
    pub fn new(mode: FilterMode, pattern: &str) -> PatternFilter {
        PatternFilter{mode: mode, pattern: pattern.to_owned()}
    }
}

#[derive(Debug)]
pub enum FilterMode {Includes, Excludes}

impl Filter for PatternFilter {
    fn keep(&self, line: &str) -> bool {
        return match &self.mode {
            FilterMode::Includes => { line.contains(&self.pattern) },
            FilterMode::Excludes => { !line.contains(&self.pattern) },
        };
    }
}