# Brasp Documentation

## Overview

**Brasp** is a Rust library designed to handle configuration parsing for command-line applications. It offers a way to define configuration options, parse command-line arguments, validate input values, and manage defaults from environment variables.

## Features

- Define options of types: string, number, and boolean.
- Supports both single and multiple values for options.
- Automatic validation of input values.
- Ability to set defaults from environment variables.
- Supports short and long command-line options.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
brasp = { path = "path/to/brasp" }
```

## Usage

### Basic Setup

Import the required modules and create an instance of `Brasp`.

```rust
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

    // Define options and flags here
}
```

### Defining Options

Options can be defined using the available methods:

- `opt` for string options.
- `num` for numeric options.
- `flag` for boolean options.

Each option can have a short name, default value, description, and validations.

```rust
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
```

### Parsing Command-Line Arguments

To parse the command-line arguments, use the `parse_raw` method. This will return an `OptionsResult` containing the parsed values and positional arguments.

```rust
let parsed_values = brasp.parse_raw(args[1..].to_vec());

if let Some(config) = parsed_values.values.get("config") {
    println!("Config value: {}", config);
}

if let Some(verbose) = parsed_values.values.get("verbose") {
    println!("Verbose mode: {}", verbose);
} else {
    println!("Verbose mode is off");
}
```

### Validating Options

Use the `validate` method to validate the parsed values against the defined options.

```rust
if let Err(e) = brasp.validate(&parsed_values.values) {
    eprintln!("Validation error: {}", e);
}
```

### Setting Defaults from Environment Variables

You can populate default values from environment variables if they are set.

```rust
brasp.set_defaults_from_env();
```

### Full Example

Here's a complete example of a command-line application using Brasp:

```rust
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
```

## Advanced Features

### Multiple Values

You can define options that accept multiple values.

```rust
brasp.opt_list(HashMap::from([(
    "include".to_string(),
    ConfigOptionBase {
        config_type: "string".to_string(),
        short: Some("I".to_string()),
        default: None,
        description: Some("Directories to include".to_string()),
        validate: Some(Validator::None),
        multiple: true,
    },
)]));
```

### Validation with Regex and Ranges

You can enable validation using regex for strings or range checks for numbers.

```rust
use regex::Regex;

brasp.opt(HashMap::from([(
    "pattern".to_string(),
    ConfigOptionBase {
        config_type: "string".to_string(),
        short: Some("p".to_string()),
        default: None,
        description: Some("Pattern to match".to_string()),
        validate: Some(Validator::Regex("^[a-z]+$".to_string())),
        multiple: false,
    },
)]));

brasp.num(HashMap::from([(
    "level".to_string(),
    ConfigOptionBase {
        config_type: "number".to_string(),
        short: Some("l".to_string()),
        default: Some(ValidValue::Number(3)),
        description: Some("Level value".to_string()),
        validate: Some(Validator::NumberRange(1, 5)),
        multiple: false,
    },
)]));
```

### Environment Variables

To set default value from environment variables, define the prefix and call `set_defaults_from_env`.

```rust
env::set_var("MYAPP_CONFIG", "default_config.yaml");

let mut brasp = Brasp {
    config_set: HashMap::new(),
    short_options: HashMap::new(),
    options: BraspOptions {
        allow_positionals: true,
        env_prefix: Some("MYAPP".to_string()),
        usage: None,
    },
};

// Define options...

brasp.set_defaults_from_env();
```

### Setting Usage Information

You can define custom usage information that will be displayed when needed.

```rust
brasp.options.usage = Some("Usage: myapp [options]".to_string());

if let Some(usage) = brasp.options.usage.clone() {
    println!("{}", usage);
}

// Or handling help flag
```

## Contribution

Open a pull request, submit your PR and I will review it. Feel free to contribute! :)