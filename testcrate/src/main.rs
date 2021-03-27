fn main() {
    let a = if let Some(a) = option_env!("TEST") {
        a.parse::<i32>().unwrap()
    } else {
        1
    };
    let b = if let Some(b) = option_env!("TEST_LOOOONNNNNNNGGGGGGGGGGGGGGG") {
        b.parse::<i32>().unwrap()
    } else {
        1
    };

    std::process::exit(a * b);
}
