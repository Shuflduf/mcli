use std::fs;

pub fn init(name: &str, version: &str, loader: &str) {
    fs::write(
        format!("{}/mcx.toml", name),
        format!(
            r#"[server]
name = "{name}"
version = "{version}"
loader = "{loader}"
"#,
        ),
    ).expect("Error writing configuration file");
}

pub fn get_value(id: &str) -> String {
    let config = fs::read_to_string("mcx.toml").expect("Error reading configuration file");
    let config: toml::Value = toml::from_str(&config).unwrap();
    config["server"][id].to_string()
}
