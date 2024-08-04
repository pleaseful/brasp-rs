use std::env;
use std::collections::HashMap;
use brasp::{Brasp, BraspOptions, ValidValue, ConfigOptionBase, Validator};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut brasp = Brasp {
        config_set: HashMap::new(),
        short_options: HashMap::new(),
        options: BraspOptions {
            allow_positionals: true,
            env_prefix: Some("MYAPP".to_string()),
            usage: None,
        },
    };

    brasp.opt(HashMap::from([(
        "config".to_string(),
        ConfigOptionBase {
            config_type: "string".to_string(),
            short: Some("c".to_string()),
            default: None,
            description: Some("Configuration file path".to_string()),
            validate: Some(Validator::None),
            multiple: false,
        },
    )]));

    brasp.flag(HashMap::from([(
        "verbose".to_string(),
        ConfigOptionBase {
            config_type: "boolean".to_string(),
            short: Some("v".to_string()),
            default: Some(ValidValue::Boolean(false)),
            description: Some("Enable verbose output".to_string()),
            validate: Some(Validator::None),
            multiple: false,
        },
    )]));

    let parsed_values = brasp.parse_raw(args[1..].to_vec());
    
    if let Some(config) = parsed_values.values.get("config") {
        println!("Config value: {}", config);
    }

    if let Some(verbose) = parsed_values.values.get("verbose") {
        println!("Verbose mode: {}", verbose);
    } else {
        println!("Verbose mode is off");
    }

    if let Some(usage) = brasp.options.usage.clone() {
        println!("{}", usage);
    }
}