
#[track_caller]
pub fn parse<T: std::str::FromStr + std::fmt::Display>(input: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    match input.parse() {
        Ok(value) => {
            debug!(from = %std::panic::Location::caller(), input = %input, value = %value);
            value
        },
        Err(e) => {
            error!(from = %std::panic::Location::caller(), input = %input, "{:?}", e);
            std::process::exit(1);
        },
    }
}

pub use {itertools, tracing::{info, error, debug, self}};

pub fn init_tracing() {
    tracing_subscriber::fmt::fmt()
        .with_file(true)
        .with_line_number(true)
        .pretty()
        .init();
}

#[macro_export]
macro_rules! init {
    () => {{
        $crate::init_tracing();

        let path = std::env::args().nth(1).unwrap_or_else(|| {
            format!("inputs/{}/input.txt", env!("CARGO_PKG_NAME"))
        });

        $crate::info!(path = path, "loading input");
        match ::std::fs::read_to_string(&path) {
            Ok(input) => input,
            Err(e) => {
                $crate::error!(path = path, "{e}");
                std::process::exit(1);
            },
        }
    }};
}
