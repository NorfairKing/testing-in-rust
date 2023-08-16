fn main() {
    println!("Hello World");
}

#[cfg(test)]
mod tests {
    // This doesn't work:
    // #[test]
    // let bool_example: bool = true;

    // This doesn't work:
    // #[test]
    // fn return_bool() -> bool {
    //     return true;
    // }

    use std::{env, thread::sleep, time::Duration};

    #[test]
    fn assert_eq() {
        assert_eq!("foo", "bar");
    }

    #[test]
    fn success_1() {
        ()
    }

    #[test]
    fn success_2() {
        ()
    }

    #[test]
    fn success_3() {
        ()
    }

    #[test]
    fn sleep_test() {
        sleep(Duration::from_secs(5));
    }

    #[test]
    fn success_4() {
        ()
    }

    #[test]
    fn fail_5() {
        assert!(false)
    }

    #[test]
    fn args() {
        let empty: Vec<String> = vec![];
        assert_eq!(env::args().collect::<Vec<String>>(), empty)
    }

    #[test]
    fn deep_eq() {
        assert_eq!(
            Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
                Some(Some(Some(Some(Some("foo")))))
            ))))))))))),
            Some(None)
        );
    }
}
