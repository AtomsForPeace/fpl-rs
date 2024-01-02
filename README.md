# FPL API Wrapper for Rust
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Overview
This is a Rust library that serves as an API wrapper for the Fantasy Premier League (FPL) API. It provides convenient methods for interacting with the FPL API, allowing you to retrieve information about players, teams, gameweeks, and more.

## Features
- Retrieve information about all FPL players.
- Get details about static gameweeks.
- Fetch static data from the FPL API.

## Usage
Here is a simple example of how to use the FPL API wrapper:
```rust
use fpl_rs::{Fpl, FplError, Players};

#[tokio::main]
async fn main() {
    // Create a new Fpl instance 
    let mut fpl = Fpl::new();

    // Retrieve information about all FPL players
    match fpl.get_all_players().await {
        Ok(players) => {
            // Process information about all FPL players
            println!("{:?}", players);
        }
        Err(err) => {
            // Handle the error
            eprintln!("Error: {}", err);
        }
    }
}
```

## Documentation
For detailed documentation on the available methods and usage, please refer to the API documentation.

## Contributing
Contributions are welcome! If you have any suggestions, bug reports, or feature requests, please open an issue or submit a pull request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
