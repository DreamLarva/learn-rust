macro_rules! X {
    () => {
        Y!()
    };
}
fn a() {
    macro_rules! Y {
        () => {
            "Hi!"
        };
    }
    assert_eq!(X!(), "Hi!");
    {
        assert_eq!(X!(), "Hi!");
        macro_rules! Y {
            () => {
                "Bye!"
            };
        }
        assert_eq!(X!(), "Bye!");
    }
    assert_eq!(X!(), "Hi!");
}

fn b() {
    macro_rules! Y {
        () => {
            "One more"
        };
    }
    assert_eq!(X!(), "One more");
}
