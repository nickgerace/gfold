use std::env;

pub fn init(debug: bool) {
    if debug {
        env::set_var("RUST_LOG", "debug");
    } else if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "off");
    }
    env_logger::init();
}
