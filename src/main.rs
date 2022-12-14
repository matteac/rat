use colored::Colorize;
use path_clean::PathClean;
use std::{
    env, fs, io,
    ops::Index,
    path::{Path, PathBuf},
    process::exit,
};

fn main() {
    let arg = match env::args().nth(1) {
        Some(s) => s,
        None => {
            println!("rat: Argument required");
            exit(1)
        }
    };

    if arg == "--version" {
        println!("rat v0.1");
        exit(0)
    }

    let file_content: String = match fs::read_to_string(&arg) {
        Ok(s) => s,
        Err(_) => {
            println!("\n{}: {}: No such file or directory", "rat".bright_green(), arg.bright_red());
            exit(2)
        }
    };
    let file_lines: Vec<&str> = file_content.split("\n").collect();

    let abs_path = absolute_path(&arg).unwrap();
    println!("\n\tFile: {}\n", abs_path.to_string_lossy().bold());
    for index in 0..file_lines.len() {
        let i = index + 1;
        println!(
            "  {}\t{}",
            i.to_string().bright_yellow(),
            file_lines.index(index)
        );
    }
}

pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }
    .clean();

    Ok(absolute_path)
}
