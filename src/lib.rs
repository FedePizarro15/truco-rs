#![allow(dead_code)]
#![allow(unused)]

use std::path::PathBuf;

use owo_colors::OwoColorize;
pub mod game;
pub mod term;

// Un copy-cat de TryFrom, pero me permite usar genericos sin interferir con la implementacion estandar de TryFrom.
trait Parse<T>: Sized {
    type Error;
    fn parse(value: T) -> Result<Self, Self::Error>;
}


#[macro_export]
macro_rules! input {
    ($($arg:expr),*) => {
        {
            use std::io::{self, Write};
            $(print!("{} ", $arg);)*
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input.trim().to_string()
        }
    };
}
#[macro_export]
macro_rules! log {
    ($($arg:expr),*) => {
        {
            use std::io::{self, Write};
            print!("LOG: ");
            $(print!("{} ", $arg);)*
            io::stdout().flush().unwrap();
        }
    };
    () => {
        
    };
}

#[macro_export]
macro_rules! logln {
    ($($arg:expr),*) => {
        {
            use std::io::{self, Write};
            print!("LOG: ");
            $(print!("{} ", $arg);)*
            print!("\n");
            io::stdout().flush().unwrap();
        }
    };
    () => {
        
    };
}

pub fn check_if_folder_exists_and_create_if_not(path: &str) -> Result<(), std::io::Error> {
    let folder = std::path::Path::new(path);
    match std::fs::metadata(folder) {
        Ok(metadata) => {
            if !metadata.is_dir() {
                std::fs::create_dir(folder)?;
            }
        }
        Err(_) => {
            std::fs::create_dir(folder)?;
        }
    
    }
    Ok(())
}

pub fn delete_all_dirs_recursively() -> Result<(), std::io::Error> {
    let directories = ["players", "mazos"];
    let directories: Vec<PathBuf> = directories
        .iter()
        .filter_map(|dir| {
            let path = std::path::Path::new(dir);
            if path.exists() {
            Some(std::path::Path::new(dir).to_path_buf())
            } else {
            None
            }
        })
        .collect();
    if directories.is_empty() {
        println!("No hay directorios para borrar ✅");
        return Ok(());
    }
    for dir in directories {
        if dir.exists() {
            logln!(
                "Borrando directorio ",
                format!("{}/", dir.display().white().bold().underline()),
                "🗑️"
            );
            std::fs::remove_dir_all(dir)?;
        }
    }
    println!("{} ✅", "Directorios borrados correctamente".underline());
    Ok(())
}
