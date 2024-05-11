trait Logger {
    fn log(&self, msg: &str);
}

pub fn square<L>(x: i32, logger: &L) -> i32 where L: Logger{
    let y = x * x;
    logger.log(&format!("{}^2 == {}", x, y));
    y
}

pub struct PrintlnLogger;

impl Logger for PrintlnLogger {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::square;
    use super::PrintlnLogger;
    use googletest::assert_that;
    use googletest::matchers::eq;

    #[test]
    fn square_works() {
        assert_eq!(square(2, &PrintlnLogger), 4);
    }
}
