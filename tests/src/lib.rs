pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn true_bool() -> bool {
    true
}

pub fn panics() {
    panic!("Panicking!!");
}

pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_is_true() {
        assert!(true_bool());
    }

    #[test]
    #[should_panic]
    fn panics_test() {
        panics();
    }

    #[test]
    fn create_guess() {
        let value = 32;
        let guess = Guess::new(value);
        assert!(guess.value == value);
    }
}
