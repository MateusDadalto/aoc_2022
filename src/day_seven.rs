use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use crate::helper;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: String, size: u32) -> Self {
        File {name, size}
    }
}

type DirRefMut = Rc<RefCell<Dir>>;

#[derive(Eq)]
struct Dir {
    name: String,
    size: u32,
    file_size: u32,
    parent: Option<DirRefMut>,
    children: HashMap<String, DirRefMut>,
    files: Vec<File>,
}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent = self.parent.as_deref().map_or(String::from("None"), |p| p.borrow().name.to_string());
        write!(f, "Dir {{ name: {}, size: {}, parent: {:?}}}", self.name, self.size, parent)
    }
}

impl PartialEq for Dir {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name) && self.parent.eq(&other.parent)
    }
}

impl PartialOrd for Dir {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size.partial_cmp(&other.size)
    }
}

impl Ord for Dir {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size.cmp(&other.size)
    }
}

impl Dir {
    fn new(name: String) -> Self {
        Dir {
            name,
            size: 0,
            file_size: 0,
            parent: None,
            children: HashMap::new(),
            files: vec![],
        }
    }

    fn add_child(&mut self, child: Dir) {
        self.children
            .insert(child.name.clone(), Rc::new(RefCell::new(child)));
    }

    fn add_file(&mut self, file: File) {
        self.add_size(file.size);
        self.file_size +=  file.size;
        self.files.push(file);
    }

    fn add_size(&mut self, size: u32) {
        self.size += size;

        if let Some(parent) = self.parent.as_deref() {
            parent.borrow_mut().add_size(size);
        }
    }

    fn get_all_children(&self) -> Vec<DirRefMut> {
        let mut result = vec![];

        for child in self.children.values() {
            result.push(Rc::clone(child));
            let grandchildren = child.borrow().get_all_children();
            result.extend(grandchildren);
        }

        result
    }
}

enum Path {
    Base,
    Out,
    Dir(String),
}

impl Path {
    fn parse(path: &str) -> Self {
        match path {
            "/" => Self::Base,
            ".." => Self::Out,
            _ => Path::Dir(path.to_owned()),
        }
    }
}
enum Command {
    CD(Path),
    LS,
}

impl Command {
    fn parse_str(line: &str) -> Self {
        let parts: Vec<String> = line.split_whitespace().map(String::from).collect();

        match parts[1].as_str() {
            "cd" => Self::CD(Path::parse(parts[2].as_str())),
            "ls" => Self::LS,
            _ => panic!("Not a known command"),
        }
    }
}

pub fn solve() {
    let total_space = 70_000_000;
    let free_space_req = 30_000_000;
    let mut lines = helper::get_file_lines_iter("inputs/day_seven.txt");

    let base_dir = Rc::new(RefCell::new(Dir::new("/".to_string())));
    let mut current_dir = Rc::clone(&base_dir);

    // building all directories
    while let Some(line) = lines.next() {
        let line = line.unwrap();

        if line.starts_with('$') {
            let command = Command::parse_str(line.as_str());
            match command {
                Command::CD(p) => {
                    current_dir = move_dir(p, current_dir.borrow(), Rc::clone(&base_dir));
                }
                Command::LS => (),
            }
        } else {
            parse_info(line, Rc::clone(&current_dir));
        }
    }

    let all_children = base_dir.borrow().get_all_children();
    let space_needed = free_space_req - (total_space - base_dir.borrow().size);

    let mut big_dirs: Vec<_> = all_children.iter().filter(|dir| dir.borrow().size >= space_needed).collect();

    big_dirs.sort();

    println!("Space needed: {space_needed}");
    println!("Day 7 part 2: {:?}", big_dirs.first().unwrap().borrow().size);
}

fn move_dir(path: Path, current_dir: Ref<'_, Dir>, base_dir: DirRefMut) -> DirRefMut {
    match path {
        Path::Base => base_dir,
        Path::Out => Rc::clone(current_dir.parent.as_ref().unwrap_or(&base_dir)),
        Path::Dir(name) => Rc::clone(
            current_dir
                .children
                .get(name.as_str())
                .expect(format!("Directory should have been listed already {}", name).as_str()),
        ),
    }
}

fn parse_info(line: String, current_dir: DirRefMut) {
    let info: Vec<String> = line.split_whitespace().map(String::from).collect();

    assert_eq!(info.len(), 2);

    match info[0].as_str() {
        "dir" => {
            let mut child = Dir::new(info[1].to_string());
            child.parent = Some(Rc::clone(&current_dir));
            current_dir.deref().borrow_mut().add_child(child);
        }
        s if s.parse::<u32>().is_ok() => {
            let file_name = info[1].to_string();
            let size: u32 = s.parse().unwrap();
            current_dir.deref().borrow_mut().add_file(File::new(file_name, size));
        }
        _ => panic!("wtf did you ls into bro..."),
    }
}
