use colored::*;
use simple_error::bail;
use std::{env::args, error::Error, ffi::OsStr, fs::read_dir, path::PathBuf};

/// Given two directory paths, recursively iterate through each directory together
/// and compare the files and directories in each directory. Emit the differences
/// between the two directories.
fn main() -> Result<(), Box<dyn Error>> {
    let path_one = match args().nth(1) {
        Some(path) => PathBuf::from(path),
        None => bail!("No paths provided. Usage: biff ./dir_one ./dir_two"),
    };
    let path_two = match args().nth(2) {
        Some(path) => PathBuf::from(path),
        None => bail!("Only one path provided. Usage: biff ./dir_one ./dir_two"),
    };

    if !path_one.is_dir() {
        bail!(format!("{:?} is not a directory", path_one));
    }
    if !path_two.is_dir() {
        bail!(format!("{:?} is not a directory", path_two));
    }

    compare_directories(&path_one, &path_two, 0)?;

    Ok(())
}

/// Macro for printing a line which is split into two columns
/// Each column is 50 characters wide and contains a path to
/// a file or directory which may be missing.
macro_rules! print_row {
    ($left:expr, $right:expr, $indent:expr) => {
        let indentation = " ".repeat($indent * 2);
        let left = format!(
            "{}{}",
            $left.file_name.to_string_lossy(),
            if $left.is_dir { "/" } else { "" }
        );
        let right = format!(
            "{}{}",
            $right.file_name.to_string_lossy(),
            if $right.is_dir { "/" } else { "" }
        );
        let width = 50 - $indent * 2;
        println!(
            "{}{:<width$} | {}{:<50}",
            indentation,
            if $left.exists {
                left.normal()
            } else {
                left.dimmed()
            },
            indentation,
            if $right.exists {
                right.normal()
            } else {
                right.dimmed()
            },
            width = width
        );
    };
}

fn compare_directories(
    path_one: &PathBuf,
    path_two: &PathBuf,
    indent: usize,
) -> Result<(), Box<dyn Error>> {
    let mut path_one_files = read_dir(path_one)
        .map(|files| {
            files
                .filter_map(|file| file.ok())
                .map(|file| file.path())
                .collect::<Vec<PathBuf>>()
        })
        .unwrap_or_default();
    let mut path_two_files = read_dir(path_two)
        .map(|files| {
            files
                .filter_map(|file| file.ok())
                .map(|file| file.path())
                .collect::<Vec<PathBuf>>()
        })
        .unwrap_or_default();

    path_one_files.sort();
    path_two_files.sort();

    let mut path_one_files = path_one_files.iter();
    let mut path_two_files = path_two_files.iter();

    let mut path_one_file = path_one_files.next();
    let mut path_two_file = path_two_files.next();

    loop {
        match (
            path_one_file.and_then(Entry::from_path_buf),
            path_two_file.and_then(Entry::from_path_buf),
        ) {
            (Some(one), Some(two)) => {
                if one.file_name == two.file_name {
                    print_row!(one, two, indent);
                    let path_one = path_one.join(one.file_name);
                    let path_two = path_two.join(two.file_name);
                    if path_one.is_dir() || path_two.is_dir() {
                        compare_directories(&path_one, &path_two, indent + 1)?;
                    }
                    path_one_file = path_one_files.next();
                    path_two_file = path_two_files.next();
                } else if one.file_name < two.file_name {
                    print_row!(one, one.as_nonexistent(), indent);
                    let path_one = path_one.join(one.file_name);
                    let path_two = path_two.join(one.file_name);
                    if path_one.is_dir() {
                        compare_directories(&path_one, &path_two, indent + 1)?;
                    }
                    path_one_file = path_one_files.next();
                } else {
                    print_row!(two.as_nonexistent(), two, indent);
                    let path_one = path_one.join(two.file_name);
                    let path_two = path_two.join(two.file_name);
                    if path_two.is_dir() {
                        compare_directories(&path_one, &path_two, indent + 1)?;
                    }
                    path_two_file = path_two_files.next();
                }
            }
            (Some(one), None) => {
                print_row!(one, one.as_nonexistent(), indent);
                let path_one = path_one.join(one.file_name);
                let path_two = path_two.join(one.file_name);
                if path_one.is_dir() {
                    compare_directories(&path_one, &path_two, indent + 1)?;
                }
                path_one_file = path_one_files.next();
            }
            (None, Some(two)) => {
                print_row!(two.as_nonexistent(), two, indent);
                let path_one = path_one.join(two.file_name);
                let path_two = path_two.join(two.file_name);
                if path_two.is_dir() {
                    compare_directories(&path_one, &path_two, indent + 1)?;
                }
                path_two_file = path_two_files.next();
            }
            (None, None) => break,
        }
    }

    Ok(())
}

struct Entry<'a> {
    file_name: &'a OsStr,
    exists: bool,
    is_dir: bool,
}

impl Entry<'_> {
    fn from_path_buf(path_buf: &PathBuf) -> Option<Entry> {
        path_buf.file_name().map(|file_name| Entry {
            file_name,
            exists: path_buf.exists(),
            is_dir: path_buf.is_dir(),
        })
    }
    fn as_nonexistent(&self) -> Entry {
        Entry {
            file_name: self.file_name,
            exists: false,
            is_dir: self.is_dir,
        }
    }
}
