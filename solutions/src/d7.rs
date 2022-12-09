use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
};

use regex::Regex;

struct Files {
    directories: HashMap<PathBuf, usize>,
}
impl Files {
    fn new() -> Self {
        Self {
            directories: Default::default(),
        }
    }
    fn add(&mut self, path: &Path, file_size: usize) {
        for ancestor in path.ancestors() {
            let prev_size = *self.directories.entry(ancestor.into()).or_insert(0);
            self.directories
                .insert(ancestor.into(), prev_size + file_size);
        }
    }
    fn get_directory_size(&self, path: &Path) -> usize {
        *self.directories.get(path).unwrap_or(&0)
    }
}

fn parse_files(input: &str) -> Result<Files, Box<dyn Error>> {
    let mut files = Files::new();
    let mut current_path = PathBuf::new();

    let regex_cd = Regex::new(r"^\$ cd (.+)$")?;
    let regex_file = Regex::new(r"^(\d+) (.+)$")?;
    for line in input.lines() {
        if let Some(caps) = regex_cd.captures(line) {
            let target = &caps[1];
            match target {
                "/" => {
                    current_path = PathBuf::new();
                }
                ".." => {
                    current_path = current_path.parent().ok_or("todo")?.into();
                }
                dir => {
                    current_path = current_path.join(dir);
                }
            }
        } else if let Some(caps) = regex_file.captures(line) {
            let file_size = caps[1].parse::<usize>()?;
            files.add(&current_path, file_size);
        }
    }

    Ok(files)
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let files = parse_files(input)?;
    let mut total = 0;
    for size in files.directories.values() {
        if *size <= 100000 {
            total += size;
        }
    }
    Ok(total.to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    let files = parse_files(input)?;
    let total_space: i32 = 70000000;
    let needed_space: i32 = 30000000;
    let used_space = files.get_directory_size(&PathBuf::new()) as i32;
    let space_needed_to_delete = used_space - (total_space - needed_space);
    let value = files
        .directories
        .values()
        .filter(|size| **size as i32 >= space_needed_to_delete)
        .min()
        .unwrap_or(&0);
    Ok(value.to_string())
}
