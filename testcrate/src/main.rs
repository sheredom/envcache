fn main() {
    let a = if let Some(a) = option_env!("TEST_A") {
        a.parse::<i32>().unwrap()
    } else {
        1
    };
    let b = if let Some(b) = option_env!("TEST_B") {
        b.parse::<i32>().unwrap()
    } else {
        1
    };

    std::process::exit(a * b);
}
