pub mod util;
pub mod rabbitmq;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_running_in_container() {
        assert_eq!(util::is_running_in_container(), false);
    }
}
