/// Build

use pkg_config::{Config};

fn main() {
    Config::new().atleast_version("1.0.0").probe("libpostal").unwrap();
}
