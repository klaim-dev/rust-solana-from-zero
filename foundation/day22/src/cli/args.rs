use crate::domain::types::SortSpec;
use std::{iter::Peekable, path::PathBuf};

use crate::cli::error::UsageError;
#[derive(Debug, PartialEq)]
pub enum Command {
    Print,
    Add{
        id: String,
        sku: String,
        name: String,
        price: String,
    },
}
#[derive(Debug, PartialEq)]
pub struct Args {
    pub cmd: Command,
    pub file: Option<PathBuf>,
    pub sort: SortSpec,
}
#[derive(Debug, PartialEq)]
pub enum ParseOutcome {
    Help,
    Args(Args),
}

fn take_value<I>(it: &mut Peekable<I>, flag: &str) -> Result<String, UsageError>
where
    I: Iterator<Item = String>,
{
    let next = it.peek();
    match next {
        None => Err(UsageError::MissingValue {
            flag: flag.to_string(),
        }),
        Some(v) if v.starts_with("--") => Err(UsageError::UnexpectedFlagValue {
            flag: flag.to_string(),
            got: v.to_string(),
        }),
        Some(_) => it.next().ok_or_else(|| UsageError::MissingValue {
            flag: flag.to_string(),
        }),
    }
}

impl Args {
    pub fn parse<I>(it: I) -> Result<ParseOutcome, UsageError>
    where
        I: IntoIterator<Item = String>,
    {
        let argv = it.into_iter().collect::<Vec<_>>();
        let has_help = argv.iter().any(|h| h == "--help" || h == "-h");
        if has_help {
            return Ok(ParseOutcome::Help);
        };

        let mut it = argv.into_iter().skip(1).peekable();
        let cmd_tok = Self::take_command(&mut it)?;

        match cmd_tok.as_str() {
            "print" => {
                let args = Self::parse_print(&mut it)?;
                Ok(ParseOutcome::Args(args))
            }
            "add" => {
                let args = Self::parse_add(&mut it)?;
                Ok(ParseOutcome::Args(args))
            }
            other => Err(UsageError::UnknownCommand {
                input: other.to_string(),
            }),
        }
    }

    fn take_command<I>(it: &mut Peekable<I>) -> Result<String, UsageError>
    where
        I: Iterator<Item = String>,
    {
        let mut cmd_tok = None;
        while let Some(tok) = it.next() {
            if !tok.starts_with('-') {
                cmd_tok = Some(tok);
                break;
            } else {
                return Err(UsageError::UnknownFlag {
                    flag: tok.to_string(),
                });
            }
        }
        let cmd_tok = cmd_tok.ok_or_else(|| UsageError::MissingCommand {
            input: "<none>".to_string(),
        })?;
        Ok(cmd_tok)
    }

    fn parse_print<I>(it: &mut Peekable<I>) -> Result<Args, UsageError>
    where
        I: Iterator<Item = String>,
    {
        let mut file = None;
        let mut sort = SortSpec::NameAsc;
        while let Some(tok) = it.next() {
            match tok.as_str() {
                "--file" => {
                    let v = take_value(it, &tok)?;
                    if v.trim().is_empty() {
                        return Err(UsageError::EmptyFilePath { input: v });
                    }
                    file = Some(PathBuf::from(v))
                }
                "--sort" => {
                    let v = take_value(it, &tok)?;
                    let spec = match v.as_str() {
                        "name" => SortSpec::NameAsc,
                        "price" => SortSpec::PriceDescNameAsc,
                        _ => return Err(UsageError::InvalidSort { input: v }),
                    };
                    sort = spec;
                }
                s if s.starts_with('-') => {
                    return Err(UsageError::UnknownFlag {
                        flag: s.to_string(),
                    });
                }
                other => {
                    return Err(UsageError::UnexpectedArg {
                        input: other.to_string(),
                    });
                }
            }
        }
        if file.is_none() {
            return Err(UsageError::MissingRequiredFlag {
                flag: "--file".to_string(),
            });
        }
        Ok(Args {
            cmd: Command::Print,
            file,
            sort,
        })
    }

    fn parse_add<I>(it: &mut Peekable<I>) -> Result<Args, UsageError>
    where
        I: Iterator<Item = String>,
    {
        let mut file = None;
        let mut sku = None;
        let mut name = None;
        let mut price = None;
        let mut id = None;
        let sort = SortSpec::NameAsc;

        while let Some(tok) = it.next() {
            match tok.as_str() {
                "--file" => {
                    let v  = take_value(it, "--file")?;
                    if v.trim().is_empty() {
                        return Err(UsageError::EmptyFilePath { input: v })
                    }
                    file = Some(PathBuf::from(v));
                },
                "--id" => {
                   let v  = take_value(it, "--id")?;
                    if v.trim().is_empty() {
                        return Err(UsageError::MissingValue {flag: "--id".to_string()})
                    }
                    id = Some(v);                    
                }
                "--sku" => {
                   let v  = take_value(it, "--sku")?;
                    if v.trim().is_empty() {
                        return Err(UsageError::MissingValue {flag: "--sku".to_string()})
                    }
                    sku = Some(v);
                },
                "--name" => {
                   let v  = take_value(it, "--name")?;
                    if v.trim().is_empty() {
                        return Err(UsageError::MissingValue { flag: "--name".to_string() })
                    }
                    name = Some(v);                    
                },
                "--price" => {
                   let v  = take_value(it, "--price")?;
                    if v.trim().is_empty() {
                        return Err(UsageError::MissingValue { flag: "--price".to_string() })
                    }
                    price = Some(v);                    
                },
                s if s.starts_with('-') => {
                    return Err(UsageError::UnknownFlag {
                        flag: s.to_string(),
                    });
                }
                other => {
                    return Err(UsageError::UnexpectedArg {
                        input: other.to_string(),
                    });
                }
            }
        }

        let file = file.ok_or_else(|| UsageError::MissingRequiredFlag {
    flag: "--file".to_string(),
})?;

let sku = sku.ok_or_else(|| UsageError::MissingRequiredFlag {
    flag: "--sku".to_string(),
})?;
let id = id.ok_or_else(|| UsageError::MissingRequiredFlag {
    flag: "--id".to_string(),
})?;

let name = name.ok_or_else(|| UsageError::MissingRequiredFlag {
    flag: "--name".to_string(),
})?;

let price = price.ok_or_else(|| UsageError::MissingRequiredFlag {
    flag: "--price".to_string(),
})?;

Ok(Args {
    cmd: Command::Add { id, sku, name, price},
    file: Some(file),
    sort: sort,
})


    }  
}
