use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn list_dir(current_dir: &PathBuf) {
    match fs::read_dir(current_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy();
                if path.is_dir() {
                    println!("[DIR]  {}", name);
                } else {
                    println!("       {}", name);
                }
            }
        }
        Err(e) => println!("Ошибка чтения каталога: {}", e),
    }
}

pub fn change_dir(current_dir: &PathBuf, new_path: &str) -> Option<PathBuf> {
    let candidate = utils::resolve_path(current_dir, new_path);
    if candidate.is_dir() {
        Some(candidate)
    } else {
        None
    }
}

pub fn remove(current_dir: &PathBuf, target: &str) -> io::Result<()> {
    let path = utils::resolve_path(current_dir, target);
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

pub fn copy(current_dir: &PathBuf, src: &str, dest: &str) -> io::Result<()> {
    let src_path = utils::resolve_path(current_dir, src);
    let dest_path = utils::resolve_path(current_dir, dest);

    if src_path.is_dir() {
        // Рекурсивное копирование каталогов
        copy_dir_recursive(&src_path, &dest_path)
    } else {
        fs::copy(src_path, dest_path).map(|_| ())
    }
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    if !dest.exists() {
        fs::create_dir(&dest)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)?;
        }
    }
    Ok(())
}
