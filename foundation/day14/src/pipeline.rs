use crate::domain::{Action, Event, Report};
use crate::errors::CsvError;
use crate::parse::parse_line;
use std::collections::HashSet;

#[derive(Default)]
struct Acc {
    total_events: usize,
    total_clicks: u64,
    total_revenue_cents: u64,
    users: HashSet<u64>,
}

pub fn build_report_strict(lines: &[String]) -> Result<Report, CsvError> {
    lines
        .iter()
        .try_fold(Acc::default(), |mut acc, line| {
            let ev = parse_line(line.as_str())?;
            acc.total_events += 1;
            acc.users.insert(ev.user_id);
            match ev.action {
                Action::Click => acc.total_clicks += ev.value,
                Action::Purchase => acc.total_revenue_cents += ev.value,
            };
            Ok(acc)
        })
        .map(|acc| Report {
            total_events: acc.total_events,
            total_clicks: acc.total_clicks,
            total_revenue_cents: acc.total_revenue_cents,
            unique_users: acc.users.len(),
        })
}

pub fn build_report_tolerant(lines: &[String]) -> Report {
    let acc = lines
        .iter()
        .filter_map(|l| parse_line(l.as_str()).ok())
        .fold(Acc::default(), |mut acc, ev| {
            acc.total_events += 1;
            acc.users.insert(ev.user_id);
            match ev.action {
                Action::Click => acc.total_clicks += ev.value,
                Action::Purchase => acc.total_revenue_cents += ev.value,
            };
            acc
        });
    Report {
        total_events: acc.total_events,
        total_clicks: acc.total_clicks,
        total_revenue_cents: acc.total_revenue_cents,
        unique_users: acc.users.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_fails_on_user_id_zero() {
        let lines = vec![
            "1,click,3".to_string(),
            "0,click,1".to_string(),
            "2,purchase,10".to_string(),
        ];

        assert_eq!(build_report_strict(&lines), Err(CsvError::UserIdZero));
    }

    #[test]
    fn tolerant_skips_invalid_lines_and_counts_only_valid() {
        let lines = vec![
            "1,click,3".to_string(),
            "0,click,1".to_string(), // invalid (UserIdZero)
            "2,purchase,10".to_string(),
        ];

        let report = build_report_tolerant(&lines);
        assert_eq!(report.total_events, 2);
        assert_eq!(report.unique_users, 2);
    }

    #[test]
    fn strict_fails_on_invalid_action() {
        let lines = vec!["1,boom,3".to_string()];

        assert!(matches!(
            build_report_strict(&lines),
            Err(CsvError::InvalidAction(_))
        ));
    }

    #[test]
    fn strict_builds_correct_report_on_valid_input() {
        let lines = vec![
            "1,click,3".to_string(),
            "2,purchase,10".to_string(),
            "1,click,2".to_string(),
        ];

        let r = build_report_strict(&lines).unwrap();
        assert_eq!(r.total_events, 3);
        assert_eq!(r.total_clicks, 5);
        assert_eq!(r.total_revenue_cents, 10);
        assert_eq!(r.unique_users, 2);
    }
}
