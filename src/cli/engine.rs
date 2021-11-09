use std::cmp::max;
use std::cmp::min;
use std::path::PathBuf;
use std::path::Path;
use std::slice::Iter;
use chrono::NaiveDateTime;
use chrono::Utc;
use smallvec::SmallVec;
use smallvec::smallvec;
use chrono::DateTime;

pub struct Engine {
    files: Vec<File>,
    filters: Vec<Box<dyn Filter>>,
    lines: Option<Vec<String>>,
}

impl Engine {
    pub fn new<P: AsRef<Path>>(files: Vec<P>, filters: Vec<Box<dyn Filter>>) -> Engine {
        let ff = files.into_iter().map(|f| File::new(f)).collect();
        Engine{files: ff, filters: filters, lines: None}
    }

    pub fn compute(&mut self) {
        let mut lines = Vec::new();

        for file in &self.files {
            for rec in &file.records {
                lines.extend(
                    rec.line_iterator()
                        .filter(|l| self.keep(l)).cloned());
            }
        }
        self.lines = Some(lines);
    }

    pub fn all_lines(&self) -> &[String] {
        match &self.lines {
            Some(ll) => return &ll,
            None => return &[] as &[String],
        };
    }

    pub fn lines(&self, start: usize, length: usize) -> &[String] {
        match &self.lines {
            Some(ll) => {
                if start >= ll.len() {
                    return &[] as &[String];
                }
                let end = min(start+length, ll.len());
                return &ll[start..end];
            },
            None => return &[] as &[String],
        };
    }

    fn keep(&self, line: &str) -> bool {
        self.filters.iter().all(|f| f.keep(line))
    }
}

struct File {
    path: PathBuf,
    records: Vec<Record>,
    filters: Vec<Box<dyn Filter>>
}

struct Record {
    timestamp: DateTime<Utc>,
    lines: SmallVec::<[String; 1]>
}

impl Record {
    fn line_iterator(&self) -> Iter<String> {
        self.lines.iter()
    }
}

impl File {
    fn new<P: AsRef<Path>>(path: P) -> File {
        let lines = File::lines_from_file(&path);
        let records = lines.into_iter()
        .map(|l| Record {
            timestamp: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc),
            lines: smallvec![l]})
        .collect();

        let p = path.as_ref();
        let mut pb = PathBuf::new();
        pb.push(p);
        File{path: pb, records, filters: Vec::new()}
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