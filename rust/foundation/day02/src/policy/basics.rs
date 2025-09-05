use crate::domain::user::User;

pub fn tier(age: u32) -> &'static str {
    if age < 18 {
        "minor"
    } else if age <= 25 {
        "yong_adult"
    } else if age <= 64 {
        "adult"
    } else {
        "senior"
    }

}

pub fn must_verify_age(age: u32, name_len: usize) -> bool {
    if age < 21 || name_len < 4 {
        true
    } else {
        false
    }
    }

    pub fn discount_percent(age: u32, name_len: usize) -> u8 {
        let max_discount: i32 = 50;
        let base: i32 = if age <= 65 {
            20
        } else {
            0
        };

        let bonus: i32 = if name_len > 10 {
            5
        } else {
            0
        };
        (base.saturating_add(bonus)).min(max_discount) as u8
    }

    pub fn avg_name_bytes_per_year(age: u32, name_len: usize) -> f64 {
        if age <= 0  {
            0.0
        } else {
            name_len as f64 / age as f64
        }
    }

    pub fn risk_score(age: u32, name_len: usize) -> u8 {
        let base = age * name_len as u32;
        (base % 256) as u8
    }
