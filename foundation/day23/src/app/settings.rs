use crate::cli::args::Args;
use crate::config::Config;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct EffectiveSettings {
    pub data_file: PathBuf,
    pub page_size: usize,
}

pub fn resolve_effective_settings(args: &Args, cfg: &Config) -> EffectiveSettings {
    let data_file = args
        .file
        .clone()
        .unwrap_or_else(|| cfg.data_file().to_path_buf());
    let page_size = args.page_size.unwrap_or(cfg.page_size());

    EffectiveSettings {
        data_file,
        page_size,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::args::Command;
    use crate::domain::types::SortSpec;
    use crate::env::FakeEnv;
    use std::io::Write;

    #[test]
    fn resolve_without_cli_file_uses_env() {
        let env = FakeEnv::new(&[("DATA_FILE", "/tmp/data.json")]);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");

        let cfg = Config::load(&env, &path).unwrap();
        let args = Args {
            cmd: Command::Print,
            file: None,
            sort: SortSpec::NameAsc,
            page_size: None,
        };

        let effective = resolve_effective_settings(&args, &cfg);
        assert_eq!(effective.data_file, PathBuf::from("/tmp/data.json"));
        assert_eq!(effective.page_size, 50);
    }

    #[test]
    fn resolve_cli_overrides_config() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        let content = r#"
data_file = "/tmp/a"
page_size = 10
"#;
        f.write_all(content.as_bytes()).unwrap();

        let env = FakeEnv::default();
        let cfg = Config::load(&env, f.path()).unwrap();
        let args = Args {
            cmd: Command::Print,
            file: Some(PathBuf::from("/tmp/b")),
            sort: SortSpec::NameAsc,
            page_size: Some(99),
        };

        let effective = resolve_effective_settings(&args, &cfg);
        assert_eq!(effective.data_file, PathBuf::from("/tmp/b"));
        assert_eq!(effective.page_size, 99);
    }
}
