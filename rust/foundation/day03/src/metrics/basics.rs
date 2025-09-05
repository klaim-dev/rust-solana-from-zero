use crate::metrics::consts::*;


pub fn tier(age: u32) -> &'static str {
    if age < TIER_ADULT_MIN {
        "minor"
    } else if age <= TIER_SENIOR_MIN {
    "adult"
    } else {
        "senior"
    }
}

pub fn must_verify_age(age: u32, name_len: usize) -> bool {
    if age < 18 || name_len > 20 {
        true
    } else  {
        false
    }
}

pub fn discount_percent(age: u32, name_len: usize) -> u8 {
    let discount = age.saturating_add(name_len as u32);
    discount.min(DISCOUNT_CAP) as u8

}

pub fn risk_score(age: u32, name_len: usize) -> u8 {
    let risk = age.saturating_add(name_len as u32);
    risk.min(255) as u8
}

pub fn avg_name_bytes_per_year(age: u32, name_len: usize) -> f64 {
    if age == 0 {
        0.0
    } else {
        name_len as f64 / age as f64
    }
}








#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn tier_age() {
        assert_eq!(tier(17), "minor");
        assert_eq!(tier(18), "adult");
        assert_eq!(tier(65), "senior");
    }

    #[test]
    fn chek_verify() {
        assert_eq!(must_verify_age(17, 10), true);
        assert_eq!(must_verify_age(17, 20), true);
        assert_eq!(must_verify_age(21, 21), true);
        assert_eq!(must_verify_age(21, 5), false);
    }

    #[test]
    fn discount_check() {
        assert_eq!(discount_percent(20, 20), 40);
        assert_eq!(discount_percent(30, 30), 50);
        assert_eq!(discount_percent(0, 0), 0);
    }

    #[test] 
    fn risk_check() {
        assert_eq!(risk_score(100, 20), 120);
        assert_eq!(risk_score(300, 300), 255);
    }

    #[test]
    fn avg_check_edge() {
        assert!((avg_name_bytes_per_year(0, 3) - 0.0) < 1e-9);
    }

    #[test]
    fn avg_check_happy() {
        assert!((avg_name_bytes_per_year(12, 3) - 4.0) < 1e-9);
    }

    #[test]
fn discount_never_exceeds_cap() {
    let test_cases = [
        (0, 0)
        (17, 5),
        (25, 50),
        (65, 100),
        (100, 1000),
    ];

    for (age, name_len) in test_cases {
        let result = discount_percent(age, name_len);
        assert!(result <= DISCOUNT_CAP, "Got {} for age {}, name_len {}", result, age, name_len);
    }

}



    
    }
