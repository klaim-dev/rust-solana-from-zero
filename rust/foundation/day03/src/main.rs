mod metrics;
mod domain;
use domain::user::User;
fn main() {
let user = User {
    name: "Alice".to_string(),
    age: 0,
};

let metrics = metrics::report::build_metrics(&user);
let format = metrics::report::format_summary(&metrics, &user);
println!("{}", format);

}



