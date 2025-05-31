use std::path::{Path, PathBuf};

/// Разрешить относительный путь на основе текущего каталога
pub fn resolve_path(current_dir: &PathBuf, input: &str) -> PathBuf {
    let input_path = Path::new(input);
    if input_path.is_absolute() {
        input_path.to_path_buf()
    } else {
        current_dir.join(input_path)
    }
}
