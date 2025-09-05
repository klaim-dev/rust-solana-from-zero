use std::fmt::format;

use crate::domain::user::User;
use crate::policy;
use crate::policy::basics::{self, discount_percent, tier};

// policy/engine.rs
pub struct PolicyReport {
    pub tier: &'static str,
    pub discount_percent: u8,
    pub must_verify_age: bool,
    pub risk_score: u8,
    pub summary: String,
}

pub fn build_report(u: &User) -> PolicyReport {

    
       let tier = basics::tier(u.age);
       let  discount_percent = basics::discount_percent(u.age, u.name.len());
       let must_verify_age = basics::must_verify_age(u.age, u.name.len());
       let  risk_score = basics::risk_score(u.age, u.name.len());




    let policy = PolicyReport{
        tier: tier,
        discount_percent: discount_percent,
        must_verify_age: must_verify_age,
        risk_score: risk_score,
        summary: format!("tire: {}, discount_percent: {}, must_verify_age : {}, risk_score: {}", tier, discount_percent, must_verify_age, risk_score ),
    };


    policy


}
