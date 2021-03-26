fn main() {
    std::process::exit(option_env!("TEST").is_none() as i32);
}
