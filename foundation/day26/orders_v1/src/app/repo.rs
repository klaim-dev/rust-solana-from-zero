use crate::app::error::AppError;
use crate::domain::store::Store;

pub trait StoreRepo {
    fn load(&self) -> Result<Store, AppError>;
    fn save(&self, store: Store) -> Result<(), AppError>;
}
