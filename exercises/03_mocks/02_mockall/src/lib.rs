use mockall::automock;

pub fn square<L>(x: i32, logger: L) -> i32
where
    L: Logger,
{
    let y = x * x;
    logger.log(&format!("{}^2 == {}", x, y));
    y
}

#[automock]
pub trait Logger {
    fn log(&self, msg: &str);
}

pub struct PrintlnLogger;

impl Logger for PrintlnLogger {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use crate::MockLogger;

    use super::square;
    use googletest::assert_that;
    use googletest::matchers::eq;

    #[test]
    fn square_works() {
        let mut mock_logger = MockLogger::new();
        mock_logger.expect_log().returning(|_| ());
        assert_eq!(square(2, mock_logger), 4);
    }
}
