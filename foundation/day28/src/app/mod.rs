pub mod error;
pub mod output;
pub mod repo;
pub mod service;

use std::path::PathBuf;

use crate::app::error::AppError;
use crate::app::output::AppOutput;
use crate::app::service::{AppCommand, OrderService};
use crate::persist::repo::FileRepo;

pub fn run(path: PathBuf, cmd: AppCommand) -> Result<AppOutput, AppError> {
    let repo = FileRepo::new(path);
    let service = OrderService::new(repo);
    service.run(cmd)
}
