#[macro_use]
extern crate points;

points! {
    #[points = {"1"}]
    test first {

    }

    #[points = {"2"}]
    suite first {
        #[points = {"2.1"}]
        test second {

        }
        #[points = {"2.2"}]
        test third {

        }
    }

    #[points = {"3", "4"}]
    suite fourth {
        #[points = {"3.1", "4.1"}]
        test fifth {

        }
    }
}
