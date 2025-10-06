use tests::Guess;

#[test]
fn creates_guess() {
    let value = 32;
    let guess = Guess::new(value);
    assert!(guess.value == 32);
}
