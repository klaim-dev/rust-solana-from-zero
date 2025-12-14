use crate::{domain::Action, domain::Event, errors::CsvError};

pub fn parse_line(line: &str) -> Result<Event, CsvError> {
    let parts = line.split(',').collect::<Vec<&str>>();
    let arryty_len = parts.len();
    if arryty_len != 3 {
        return Err(CsvError::WrongArity {
            expected: 3,
            got: arryty_len,
        });
    }

    let raw_user_id = parts[0];
    let user_id = raw_user_id
        .parse::<u64>()
        .map_err(|_| CsvError::InvalidUserId(raw_user_id.to_string()))?;
    if user_id == 0 {
        return Err(CsvError::UserIdZero);
    }

    let raw_action = parts[1];
    let action = match raw_action {
        "click" => Action::Click,
        "purchase" => Action::Purchase,
        other => return Err(CsvError::InvalidAction(other.to_string())),
    };

    let raw_value = parts[2];
    let value = raw_value
        .parse::<u64>()
        .map_err(|_| CsvError::InvalidValue(raw_value.to_string()))?;

    Ok(Event {
        user_id,
        action,
        value,
    })
}
