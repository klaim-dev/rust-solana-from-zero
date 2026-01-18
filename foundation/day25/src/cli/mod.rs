mod error;

use std::path::PathBuf;

use crate::app::service::AppCommand;
use crate::domain::error::DomainError;
use crate::domain::types::{OrderId, Qty, Sku};
pub use error::UsageError;

pub struct CliCommand {
    pub file: Option<PathBuf>,
    action: CliAction,
}

enum CliAction {
    AddOrder { id: u64, customer: String },
    AddItem { id: u64, sku: String, qty: u32 },
    RemoveItem { id: u64, sku: String },
    Show { id: u64 },
    List { customer: Option<String> },
    Total { id: u64 },
}

pub fn parse_args<I>(args: I) -> Result<CliCommand, UsageError>
where
    I: IntoIterator<Item = String>,
{
    let mut it = args.into_iter();
    let cmd = it.next().ok_or(UsageError::MissingCommand)?;

    if cmd == "--help" || cmd == "-h" || cmd == "help" {
        return Err(UsageError::HelpRequested);
    }

    let mut id: Option<u64> = None;
    let mut customer: Option<String> = None;
    let mut sku: Option<String> = None;
    let mut qty: Option<u32> = None;
    let mut file: Option<PathBuf> = None;

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--id" => {
                let value = next_value(&mut it, "--id")?;
                id = Some(parse_u64("--id", value)?);
            }
            "--customer" => {
                customer = Some(next_value(&mut it, "--customer")?);
            }
            "--sku" => {
                sku = Some(next_value(&mut it, "--sku")?);
            }
            "--qty" => {
                let value = next_value(&mut it, "--qty")?;
                qty = Some(parse_u32("--qty", value)?);
            }
            "--file" => {
                let value = next_value(&mut it, "--file")?;
                file = Some(PathBuf::from(value));
            }
            "--help" | "-h" => return Err(UsageError::HelpRequested),
            _ => return Err(UsageError::UnknownFlag(arg)),
        }
    }

    let action = match cmd.as_str() {
        "add-order" => {
            validate_unused_flags(&cmd, &id, &customer, &sku, &qty)?;
            CliAction::AddOrder {
                id: required(id, "--id")?,
                customer: required(customer, "--customer")?,
            }
        }
        "add-item" => {
            validate_unused_flags(&cmd, &id, &customer, &sku, &qty)?;
            CliAction::AddItem {
                id: required(id, "--id")?,
                sku: required(sku, "--sku")?,
                qty: required(qty, "--qty")?,
            }
        }
        "remove-item" => {
            validate_unused_flags(&cmd, &id, &customer, &sku, &qty)?;
            CliAction::RemoveItem {
                id: required(id, "--id")?,
                sku: required(sku, "--sku")?,
            }
        }
        "show" => {
            validate_unused_flags(&cmd, &id, &customer, &sku, &qty)?;
            CliAction::Show {
                id: required(id, "--id")?,
            }
        }
        "list" => {
            validate_unused_flags(&cmd, &id, &customer, &sku, &qty)?;
            CliAction::List { customer }
        }
        "total" => {
            validate_unused_flags(&cmd, &id, &customer, &sku, &qty)?;
            CliAction::Total {
                id: required(id, "--id")?,
            }
        }
        _ => return Err(UsageError::UnknownCommand(cmd)),
    };

    Ok(CliCommand { file, action })
}

pub fn usage() -> &'static str {
    "Usage: orders <command> [options]\n\
\n\
Commands:\n\
  add-order --id <id> --customer <name>\n\
  add-item --id <id> --sku <sku> --qty <qty>\n\
  remove-item --id <id> --sku <sku>\n\
  show --id <id>\n\
  list [--customer <name>]\n\
  total --id <id>\n\
\n\
Options:\n\
  --file <path>       override default storage file\n\
  -h, --help          show this help\n"
}

impl CliCommand {
    pub fn into_app_command(self) -> Result<(AppCommand, Option<PathBuf>), UsageError> {
        let cmd = match self.action {
            CliAction::AddOrder { id, customer } => AppCommand::AddOrder {
                id: OrderId::new(id)
                    .map_err(|e| invalid_value("--id", id.to_string(), e))?,
                customer,
            },
            CliAction::AddItem { id, sku, qty } => AppCommand::AddItem {
                id: OrderId::new(id)
                    .map_err(|e| invalid_value("--id", id.to_string(), e))?,
                sku: Sku::new(sku.clone())
                    .map_err(|e| invalid_value("--sku", sku, e))?,
                qty: Qty::new(qty)
                    .map_err(|e| invalid_value("--qty", qty.to_string(), e))?,
            },
            CliAction::RemoveItem { id, sku } => AppCommand::RemoveItem {
                id: OrderId::new(id)
                    .map_err(|e| invalid_value("--id", id.to_string(), e))?,
                sku: Sku::new(sku.clone())
                    .map_err(|e| invalid_value("--sku", sku, e))?,
            },
            CliAction::Show { id } => AppCommand::Show {
                id: OrderId::new(id)
                    .map_err(|e| invalid_value("--id", id.to_string(), e))?,
            },
            CliAction::List { customer } => AppCommand::List { customer },
            CliAction::Total { id } => AppCommand::Total {
                id: OrderId::new(id)
                    .map_err(|e| invalid_value("--id", id.to_string(), e))?,
            },
        };

        Ok((cmd, self.file))
    }
}

fn next_value<I>(it: &mut I, flag: &'static str) -> Result<String, UsageError>
where
    I: Iterator<Item = String>,
{
    it.next()
        .ok_or_else(|| UsageError::MissingFlagValue(flag.to_string()))
}

fn required<T>(value: Option<T>, flag: &'static str) -> Result<T, UsageError> {
    value.ok_or(UsageError::MissingRequiredFlag(flag))
}

fn parse_u64(flag: &'static str, input: String) -> Result<u64, UsageError> {
    input.parse::<u64>().map_err(|_| UsageError::InvalidInt {
        flag,
        input,
    })
}

fn parse_u32(flag: &'static str, input: String) -> Result<u32, UsageError> {
    input.parse::<u32>().map_err(|_| UsageError::InvalidInt {
        flag,
        input,
    })
}

fn validate_unused_flags(
    cmd: &str,
    id: &Option<u64>,
    customer: &Option<String>,
    sku: &Option<String>,
    qty: &Option<u32>,
) -> Result<(), UsageError> {
    match cmd {
        "add-order" => {
            unexpected(cmd, "--sku", sku.is_some())?;
            unexpected(cmd, "--qty", qty.is_some())?;
        }
        "add-item" => {
            unexpected(cmd, "--customer", customer.is_some())?;
        }
        "remove-item" => {
            unexpected(cmd, "--customer", customer.is_some())?;
            unexpected(cmd, "--qty", qty.is_some())?;
        }
        "show" | "total" => {
            unexpected(cmd, "--customer", customer.is_some())?;
            unexpected(cmd, "--sku", sku.is_some())?;
            unexpected(cmd, "--qty", qty.is_some())?;
        }
        "list" => {
            unexpected(cmd, "--id", id.is_some())?;
            unexpected(cmd, "--sku", sku.is_some())?;
            unexpected(cmd, "--qty", qty.is_some())?;
        }
        _ => {}
    }
    Ok(())
}

fn unexpected(cmd: &str, flag: &'static str, used: bool) -> Result<(), UsageError> {
    if used {
        return Err(UsageError::UnexpectedFlagForCommand {
            cmd: cmd.to_string(),
            flag,
        });
    }
    Ok(())
}

fn invalid_value(flag: &'static str, input: String, err: DomainError) -> UsageError {
    UsageError::InvalidValue {
        flag,
        input,
        reason: err.to_string(),
    }
}
