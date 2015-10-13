#[macro_use]
extern crate points;

points! {
    #[points = "1"]
    fn first_fn() {

    }

    #[points = "2"]
    mod first {
        #[points = "2.1"]
        fn second_fn() {

        }
        #[points = "2.2"]
        fn third_fn() {

        }
    }

    #[points = "3"]
    fn fourth_fn() {

    }
}
