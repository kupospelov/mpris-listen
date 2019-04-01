macro_rules! error {
    ($($args:expr),+) => ({
        eprintln!($($args),+);
        std::process::exit(1);
    })
}
