use colored::*;
use std::env;
use std::fs;
use std::path;
use std::str::FromStr;
use clap::{Arg, App};

fn get_dir_count(p: &path::Path) -> usize {
    let mut result = 0;
    if let Ok(children) = fs::read_dir(p) {
        for child in children {
            if let Ok(child) = child {
                if child.metadata().unwrap().is_dir() {
                    result += 1;
                }
            }
        }
    }
    result
}

// Some helper function to get the proper characters depending on 
// whether ascii should be used or not.

fn i_char(use_ascii: bool) -> String {
    match use_ascii {
        false => "│".to_string(),
        true => "|".to_string()
    }
}

fn t_char(use_ascii: bool) -> String {
    match use_ascii {
        false => "├────".to_string(),
        true =>  "+---".to_string()
    }
}

fn l_char(use_ascii: bool) -> String {
    match use_ascii {
        false => "└────".to_string(),
        true => "\\---".to_string()
    }
}


fn print_row(depth: u32, open_depths: &mut Vec<u32>, color_depth_map_vec: &Vec<Color>, item: &fs::DirEntry, is_last_item: bool, use_ascii: bool) {
    for i in 0..depth {
        if open_depths.contains(&i) {
            let adjusted_depth = i % color_depth_map_vec.len() as u32;
            let branch_color = color_depth_map_vec[adjusted_depth as usize];
            print!("{}     ", i_char(use_ascii).color(branch_color));
        } else {
            print!("      ");
        }
    }
    let adjusted_depth = depth % color_depth_map_vec.len() as u32;
    let branch_color = color_depth_map_vec[adjusted_depth as usize];
    let file_name = item.file_name().into_string().unwrap();
    let file_color = match item.metadata().unwrap().is_dir() {
        true => Color::Blue,
        false => Color::White,
    };
    if !is_last_item {
        println!("{} {}", t_char(use_ascii).color(branch_color), file_name.color(file_color)
    );
    } else {
        println!("{} {}", l_char(use_ascii).color(branch_color), file_name.color(file_color));
        open_depths.pop();
    }
}


fn recurse_children(
    p: &path::Path,
    depth: u32,
    color_depth_map: &Vec<Color>,
    open_depths: &mut Vec<u32>,
    show_files: bool,
    use_ascii: bool
) {
    if let Ok(children) = fs::read_dir(p) {
        let mut entry_counter = 1;
        let total_entries = match show_files {
            true => fs::read_dir(p).unwrap().count(),
            false => get_dir_count(p)
        };
        if !show_files {
            // println!("found {} directories", total_entries);
        }
        // Don't do anything with empty directories:
        if total_entries > 0 {
            open_depths.push(depth);
            // println!("Opened depth {}", depth);
            for child in children {
                if let Ok(child) = child {
                    if child.metadata().unwrap().is_dir() || show_files {
                        let is_last_item = entry_counter == total_entries;
                        print_row(depth, open_depths, color_depth_map, &child, is_last_item, use_ascii);
                        // Print all the children of children that are directories
                        // (this is where the recursion happens)
                        if child.metadata().unwrap().is_dir() {
                            recurse_children(
                                child.path().as_path(),
                                depth + 1,
                                color_depth_map,
                                open_depths,
                                show_files,
                                use_ascii
                            );
                        }
                        entry_counter += 1;
                    }
                }
            }
        }
    }
}


fn main() {

    let matches = App::new("rtr")
        .version("0.1.0")
        .author("Teun van Wezel")
        .about("Rust'ed tree")
        .arg(Arg::with_name("Path"))
        .arg(Arg::with_name("Show files")
                .short("f")
                .long("show-files")
                .takes_value(false)
                .help("Show files inside folders."))
        .arg(Arg::with_name("Use ASCII")
                .short("a")
                .long("ascii")
                .takes_value(false)
                .help("Use ASCII characters."))
        .get_matches();

    // Vector to map depths to colors in the tree.
    let color_depth_map: Vec<Color> = vec![
        Color::Cyan,
        Color::Magenta,
        Color::Red,
        Color::Yellow,
        Color::Green,
    ];

    let current_dir = env::current_dir()
                .expect("Expected to be able to find the current directory, but failed.");

    let dir = match matches.value_of("Path") {
        Some(path) => match path {
            "." => current_dir,
            _ => path::PathBuf::from_str(&path)
                .expect(&format!("Expected a valid path, got {}", path))
        },
        None => current_dir
    };

    if dir.exists() {
        if dir.metadata().unwrap().is_dir() {

            let show_files: bool = match matches.occurrences_of("Show files") {
                1 => true,
                _ => false
            };
            let use_ascii: bool = match matches.occurrences_of("Use ASCII") {
                1 => true,
                _ => false
            };

            let root = dir.file_name().unwrap().to_str().unwrap();
            println!("Showing tree of directory '{}':\n", root);
            println!("{}", root);
            // Use a Vec<u32> as a stack to keep track of "open depths".
            let mut open_depths: Vec<u32> = Vec::new();
            recurse_children(dir.as_path(), 0, &color_depth_map, &mut open_depths, show_files, use_ascii);
        }
    } else {
        panic!("Directory {:?} doesn't exist.", dir);
    }
}
