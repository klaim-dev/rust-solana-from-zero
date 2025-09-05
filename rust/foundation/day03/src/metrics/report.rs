use crate::metrics::basics::{avg_name_bytes_per_year, discount_percent, must_verify_age, risk_score, tier};
use crate::domain::user::User;


pub struct UserMetrics {
    pub tier: &'static str,
    pub must_verify_age: bool,
    pub discount_percent: u8,
    pub risk_score: u8,
    pub avg_name_bytes_per_year: f64,
}

pub fn build_metrics(u: &User) -> UserMetrics {
    let tier = tier(u.age);
    let must_verify_age = must_verify_age(u.age, u.name.len());
    let discount_percent = discount_percent(u.age, u.name.len());
    let risk_score = risk_score(u.age, u.name.len());
    let avg_name_bytes_per_year = avg_name_bytes_per_year(u.age, u.name.len());

    let user_metrics = UserMetrics{
        tier,
        must_verify_age,
        discount_percent,
        risk_score,
        avg_name_bytes_per_year,
    };
    user_metrics
 
}

pub fn format_summary(m: &UserMetrics, u: &User) -> String {
    format!("User : {} ({}), tier: {}, must_verify_age: {}, discount_percent: {}, risk_score: {}, avg_name_bytes_per_year: {}",u.name, u.age, m.tier, m.must_verify_age, m.discount_percent, m.risk_score, m.avg_name_bytes_per_year)

}













#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn check_build_tire() {
    let user = User { name: "Alice".to_string(), age: 25 };
    let result = build_metrics(&user);
    assert_eq!(result.tier, "adult");
    }
    #[test]
    fn check_build_avg_happy() {
        let user = User { name: "Alice".to_string(), age: 25 };
        let result = build_metrics(&user);
        assert!((result.avg_name_bytes_per_year -0.2).abs() <1e-9);
    }

    #[test]
    fn check_build_avg_edge() {
        let user = User { name: "Alice".to_string(), age: 0 };
        let result = build_metrics(&user);
        assert!((result.avg_name_bytes_per_year -0.0).abs() <1e-9);
    }

    #[test]
    fn summary_happy() {
        let user = User { name: "Alice".to_string(), age: 25 };
        let result = build_metrics(&user);
        let summary = format_summary(&result, &user);
        assert!(summary.contains("Alice"));
        assert!(summary.contains("tier: adult"));
        assert!(summary.contains("(25)"));

    }
    #[test]
fn summary_zero_age() {
    let user = User { name: "Neo".to_string(), age: 0 };
    let result = build_metrics(&user);
    let summary = format_summary(&result, &user);

    assert!(summary.contains("Neo"));
    assert!(summary.contains("(0)"));
}

#[test]
fn summary_empty_name() {
    let user = User { name: "".to_string(), age: 30 };
    let result = build_metrics(&user);
    let summary = format_summary(&result, &user);

    assert!(summary.contains("tier: adult"));
    assert!(summary.contains("(30)"));
}





    
}