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

#[derive(Default)]
struct Flags {
    id: Option<u64>,
    customer: Option<String>,
    sku: Option<String>,
    qty: Option<u32>,
}

impl Flags {
    fn validate_for_command(&self, cmd: &str) -> Result<(), UsageError> {
        match cmd {
            "add-order" => {
                reject_unexpected_flag(cmd, "--sku", self.sku.is_some())?;
                reject_unexpected_flag(cmd, "--qty", self.qty.is_some())?;
            }
            "add-item" => {
                reject_unexpected_flag(cmd, "--customer", self.customer.is_some())?;
            }
            "remove-item" => {
                reject_unexpected_flag(cmd, "--customer", self.customer.is_some())?;
                reject_unexpected_flag(cmd, "--qty", self.qty.is_some())?;
            }
            "show" | "total" => {
                reject_unexpected_flag(cmd, "--customer", self.customer.is_some())?;
                reject_unexpected_flag(cmd, "--sku", self.sku.is_some())?;
                reject_unexpected_flag(cmd, "--qty", self.qty.is_some())?;
            }
            "list" => {
                reject_unexpected_flag(cmd, "--id", self.id.is_some())?;
                reject_unexpected_flag(cmd, "--sku", self.sku.is_some())?;
                reject_unexpected_flag(cmd, "--qty", self.qty.is_some())?;
            }
            _ => {}
        }
        Ok(())
    }
}

pub fn parse_args<I>(args: I) -> Result<CliCommand, UsageError>
where
    I: IntoIterator<Item = String>,
{
    let mut it = args.into_iter();
    let cmd = next_command(&mut it)?;

    if is_help_command(&cmd) {
        return Err(UsageError::HelpRequested);
    }
    if cmd.starts_with('-') {
        return Err(UsageError::UnknownFlag(cmd));
    }

    let (file, flags) = parse_flags(&mut it)?;
    flags.validate_for_command(&cmd)?;

    let action = build_action(&cmd, flags)?;
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
                id: parse_order_id("--id", id)?,
                customer,
            },
            CliAction::AddItem { id, sku, qty } => AppCommand::AddItem {
                id: parse_order_id("--id", id)?,
                sku: parse_sku("--sku", sku)?,
                qty: parse_qty("--qty", qty)?,
            },
            CliAction::RemoveItem { id, sku } => AppCommand::RemoveItem {
                id: parse_order_id("--id", id)?,
                sku: parse_sku("--sku", sku)?,
            },
            CliAction::Show { id } => AppCommand::Show {
                id: parse_order_id("--id", id)?,
            },
            CliAction::List { customer } => AppCommand::List { customer },
            CliAction::Total { id } => AppCommand::Total {
                id: parse_order_id("--id", id)?,
            },
        };

        Ok((cmd, self.file))
    }
}

fn next_command<I>(it: &mut I) -> Result<String, UsageError>
where
    I: Iterator<Item = String>,
{
    it.next().ok_or(UsageError::MissingCommand)
}

fn parse_flags<I>(it: &mut I) -> Result<(Option<PathBuf>, Flags), UsageError>
where
    I: Iterator<Item = String>,
{
    let mut flags = Flags::default();
    let mut file: Option<PathBuf> = None;

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--id" => {
                let value = take_value(it, "--id")?;
                flags.id = Some(parse_u64("--id", &value)?);
            }
            "--customer" => {
                flags.customer = Some(take_value(it, "--customer")?);
            }
            "--sku" => {
                flags.sku = Some(take_value(it, "--sku")?);
            }
            "--qty" => {
                let value = take_value(it, "--qty")?;
                flags.qty = Some(parse_u32("--qty", &value)?);
            }
            "--file" => {
                let value = take_value(it, "--file")?;
                file = Some(PathBuf::from(value));
            }
            "--help" | "-h" => return Err(UsageError::HelpRequested),
            _ => return Err(UsageError::UnknownFlag(arg)),
        }
    }

    Ok((file, flags))
}

fn build_action(cmd: &str, flags: Flags) -> Result<CliAction, UsageError> {
    let Flags {
        id,
        customer,
        sku,
        qty,
    } = flags;

    match cmd {
        "add-order" => Ok(CliAction::AddOrder {
            id: required(id, "--id")?,
            customer: required(customer, "--customer")?,
        }),
        "add-item" => Ok(CliAction::AddItem {
            id: required(id, "--id")?,
            sku: required(sku, "--sku")?,
            qty: required(qty, "--qty")?,
        }),
        "remove-item" => Ok(CliAction::RemoveItem {
            id: required(id, "--id")?,
            sku: required(sku, "--sku")?,
        }),
        "show" => Ok(CliAction::Show {
            id: required(id, "--id")?,
        }),
        "list" => Ok(CliAction::List { customer }),
        "total" => Ok(CliAction::Total {
            id: required(id, "--id")?,
        }),
        _ => Err(UsageError::UnknownCommand(cmd.to_string())),
    }
}

fn is_help_command(cmd: &str) -> bool {
    matches!(cmd, "--help" | "-h" | "help")
}

fn take_value<I>(it: &mut I, flag: &'static str) -> Result<String, UsageError>
where
    I: Iterator<Item = String>,
{
    it.next()
        .ok_or_else(|| UsageError::MissingFlagValue(flag.to_string()))
}

fn required<T>(value: Option<T>, flag: &'static str) -> Result<T, UsageError> {
    value.ok_or(UsageError::MissingRequiredFlag(flag))
}

fn parse_u64(flag: &'static str, input: &str) -> Result<u64, UsageError> {
    input.parse::<u64>().map_err(|_| UsageError::InvalidInt {
        flag,
        input: input.to_string(),
    })
}

fn parse_u32(flag: &'static str, input: &str) -> Result<u32, UsageError> {
    input.parse::<u32>().map_err(|_| UsageError::InvalidInt {
        flag,
        input: input.to_string(),
    })
}

fn parse_order_id(flag: &'static str, input: u64) -> Result<OrderId, UsageError> {
    OrderId::new(input).map_err(|e| invalid_value(flag, input.to_string(), e))
}

fn parse_sku(flag: &'static str, input: String) -> Result<Sku, UsageError> {
    let input_for_error = input.clone();
    Sku::new(input).map_err(|e| invalid_value(flag, input_for_error, e))
}

fn parse_qty(flag: &'static str, input: u32) -> Result<Qty, UsageError> {
    Qty::new(input).map_err(|e| invalid_value(flag, input.to_string(), e))
}

fn reject_unexpected_flag(cmd: &str, flag: &'static str, used: bool) -> Result<(), UsageError> {
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
