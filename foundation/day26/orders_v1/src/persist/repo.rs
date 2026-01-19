use std::path::PathBuf;

use crate::app::error::AppError;
use crate::app::repo::StoreRepo;
use crate::domain::store::Store;
use crate::persist::error::PersistError;
use crate::persist::fs;

pub struct FileRepo {
    path: PathBuf,
}

impl FileRepo {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl StoreRepo for FileRepo {
    fn load(&self) -> Result<Store, AppError> {
        let store = fs::load(&self.path)?;
        Ok(store)
    }

    fn save(&self, store: Store) -> Result<(), AppError> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| PersistError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
        fs::save_atomic(&self.path, &store)?;
        Ok(())
    }
}
