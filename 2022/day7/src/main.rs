use std::io::{BufRead, stdin};

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let root_fs = parse_input()?;

    println!("Part 1: {}", part1(&root_fs));
    println!("Part 2: {}", part2(&root_fs));

    Ok(())
}

fn parse_input() -> Result<Directory> {
    let mut root = Directory::new("");
    let mut working_dir = Vec::new();

    let mut lines = stdin().lock().lines().peekable();
    'outer: loop {
        let Some(line) = lines.next() else {
            break;
        };
        let line = line?;

        let tokens: Vec<_> = line.split_ascii_whitespace().collect();
        match *tokens.as_slice() {
            ["$", "cd", dir] => match dir {
                "/" => {
                    working_dir.clear();
                }
                ".." => {
                    working_dir.pop();
                }
                _ => {
                    working_dir.push(dir.to_owned());
                }
            },
            ["$", "ls"] => 'files: loop {
                if let Some(next_line) = lines.peek() {
                    let next_line: &String = match next_line {
                        Ok(next_line) => next_line,
                        Err(e) => bail!("{e}"),
                    };
                    if next_line.starts_with("$") {
                        break 'files;
                    }

                    let line = lines.next().unwrap();
                    let line = line?;
                    let tokens: Vec<_> = line.split_ascii_whitespace().collect();

                    if tokens[0] == "dir" {
                        let dir = Directory::new(tokens[1]);
                        root.add_directory(&working_dir, dir);
                    } else {
                        let file = File {
                            size: tokens[0].parse()?,
                        };
                        root.add_file(&working_dir, file);
                    };
                } else {
                    break 'outer;
                };
            },
            _ => bail!("unrecognized command"),
        }
    }

    Ok(root)
}

fn part1(root_fs: &Directory) -> usize {
    root_fs
        .subdir_sizes()
        .into_iter()
        .filter(|&size| size <= 100_000)
        .sum()
}

fn part2(root_fs: &Directory) -> usize {
    let total = 70_000_000;
    let used = root_fs.size();
    let free = total - used;
    let target = 30_000_000;
    root_fs
        .subdir_sizes()
        .into_iter()
        .filter(|&size| size + free >= target)
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

#[derive(Debug)]
struct File {
    // Name can be ignored
    size: usize,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    fn find_mut(&mut self, path: &[String]) -> &mut Directory {
        match path {
            [] => self,
            [head, rest @ ..] => self
                .directories
                .iter_mut()
                .find(|d| d.name == *head)
                .unwrap()
                .find_mut(rest),
        }
    }

    fn add_file(&mut self, path: &[String], file: File) {
        self.find_mut(path).files.push(file);
    }

    fn add_directory(&mut self, path: &[String], directory: Directory) {
        self.find_mut(path).directories.push(directory);
    }

    fn size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum::<usize>()
            + self.directories.iter().map(|d| d.size()).sum::<usize>()
    }

    fn subdir_sizes(&self) -> Vec<usize> {
        let mut sizes = vec![self.size()];
        for d in &self.directories {
            sizes.extend(d.subdir_sizes());
        }
        sizes
    }
}
