use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::domain::store::Store;
use crate::persist::error::PersistError;
use crate::persist::format::{parse, serialize};

pub fn load(path: &Path) -> Result<Store, PersistError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Store::new()),
        Err(err) => {
            return Err(PersistError::Io {
                path: path.to_path_buf(),
                source: err,
            });
        }
    };
    let store = parse(&text)?;
    Ok(store)
}

pub fn save_atomic(path: &Path, store: &Store) -> Result<(), PersistError> {
    let content = serialize(store);

    let tmp_path: PathBuf = path.with_extension("tmp");

    {
        let mut f = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&tmp_path)
            .map_err(|e| PersistError::Io {
                path: tmp_path.clone(),
                source: e,
            })?;

        f.write_all(content.as_bytes())
            .map_err(|e| PersistError::Io {
                path: tmp_path.clone(),
                source: e,
            })?;

        f.flush().map_err(|e| PersistError::Io {
            path: tmp_path.clone(),
            source: e,
        })?;

        f.sync_all().map_err(|e| PersistError::Io {
            path: tmp_path.clone(),
            source: e,
        })?;
    }

    fs::rename(&tmp_path, path).map_err(|e| PersistError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;

    Ok(())
}
