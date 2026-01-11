use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use crate::index::InventoryIndex;
use crate::persist::error::PersistError;
use crate::persist::format::deserialize;

pub fn load_from_file(path: &Path) -> Result<InventoryIndex, PersistError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // We read the whole file into a String first instead of streaming line-by-line.
    // This separates IO errors (reading the file) from parsing errors (invalid data).
    // It also simplifies the logic by having a complete text buffer for deserialization.
    let mut text = String::new();
    for line in reader.lines() {
        let line = line?;
        text.push_str(&line);
        text.push('\n');
    }

    let idx = deserialize(&text)?;
    Ok(idx)
}

pub fn save_to_file(idx: &InventoryIndex, path: &Path) -> Result<(), PersistError> {
    use std::io::Write;
    use crate::persist::format::serialize;

    let text = serialize(idx);

    // 1. Get directory and filename
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    let filename = path.file_name()
        .ok_or_else(|| PersistError::InvalidPath(path.to_string_lossy().to_string()))?;
    
    let temp_filename = format!("{}.tmp", filename.to_string_lossy());
    let temp_path = dir.join(temp_filename);

    // 2. Create temp file
    let file = File::create(&temp_path)?;
    
    // 3. Write with BufWriter
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(text.as_bytes())?;
    
    // 4. Flush and sync
    writer.flush()?;
    let file = writer.into_inner().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    file.sync_all()?;

    // 5. Atomic rename
    // 5. Atomic rename
    if let Err(e) = std::fs::rename(&temp_path, path) {
        // If rename fails, try to remove the temporary file to avoid clutter.
        // We ignore errors from remove_file since the original error `e` is more important.
        let _ = std::fs::remove_file(&temp_path);
        return Err(e.into());
    }
    
    Ok(())
}
