mod config;
use config::ScopeConfig;
use toml_scaffold::TomlScaffold;

fn main() -> Result<(), String> {
    println!("# Generated config file for scope monitor");
    println!();
    println!("{}", ScopeConfig::default().to_scaffold().unwrap());
    Ok(())
}
