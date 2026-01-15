use day24::app::build::build_app;
use day24::config::models::Config;
use day24::domain::record::{Name, PriceCents, Record, Sku};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = match std::env::var("CONFIG_PATH") {
        Ok(value) => PathBuf::from(value),
        Err(_) => PathBuf::from("./config.toml"),
    };

    let cfg = Config::load(&path)?;
    let state = build_app(cfg)?;

    let records = vec![
        Record::new(
            Sku::new("SKU-001".to_string())?,
            Name::new("  Spacey  Name  ".to_string())?,
            PriceCents::new(1200)?,
        ),
        Record::new(
            Sku::new("SKU-002".to_string())?,
            Name::new("Another   Name".to_string())?,
            PriceCents::new(500)?,
        ),
    ];

    let _processed = state.pipeline.run_all(records)?;
    Ok(())
}
