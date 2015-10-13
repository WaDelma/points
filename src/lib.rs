#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref _FILE: ::std::sync::Mutex<::std::fs::File>
        = ::std::sync::Mutex::new(::std::fs::File::create("tmc-files.txt").unwrap());
}

#[macro_export]
macro_rules! points {
    () => {};
    (#[points = $points:expr] suite $suite:ident { $($block:tt)* } $($tail:tt)*) => {
        points!(@helper $points, $suite, $($block)*);
        points!($($tail)*);
    };
    (#[points = $points:expr] test $test:ident $code:block $($tail:tt)*) => {
        #[test]
        fn $test() {
            use std::io::Write;
            let mut file = $crate::_FILE.lock().unwrap();
            write!(file, "{} = {}\n", stringify!($test), $points).unwrap();
            $code
        }
        points!($($tail)*);
    };

    (@helper $spoints:expr, $suite:ident,) => {};
    (@helper $spoints:expr, $suite:ident, #[points = $points:expr] test $test:ident $code:block) => {
        #[test]
        fn $test() {
            use std::io::Write;
            let mut file = $crate::_FILE.lock().unwrap();
            write!(file, "{} = {}\n", stringify!($suite), $spoints).unwrap();
            write!(file, "{}.{} = {}\n", stringify!($suite), stringify!($test), $points).unwrap();
            $code
        }
    };
    (@helper $spoints:expr, $suite:ident, #[points = $points:expr] test $test:ident $code:block $($tail:tt)*) => {
        #[test]
        fn $test() {
            use std::io::Write;
            let mut file = $crate::_FILE.lock().unwrap();
            write!(file, "{}.{} = {}\n", stringify!($suite), stringify!($test), $points).unwrap();
            $code
        }
        points!(@helper $spoints, $suite, $($tail)*);
    };
}
