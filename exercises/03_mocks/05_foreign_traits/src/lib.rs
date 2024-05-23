use std::str::FromStr;

mockall::mock! {
    // TODO!
    Parsed {}

    impl FromStr for Parsed {
        type Err = Box<dyn std::error::Error>;
        fn from_str(s: &str) -> Result<Self, <MockParsed as FromStr>::Err>;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implements() {
        static_assertions::assert_impl_one!(MockParsed: FromStr);
    }
}
