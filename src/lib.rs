#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref _FILE: ::std::sync::Mutex<::std::fs::File>
        = ::std::sync::Mutex::new(::std::fs::File::create("tmc-points.txt").unwrap());
}

#[macro_export]
macro_rules! points {
    () => {};
    (#[points = {$($points:expr),* $(,)*}] suite $suite:ident { $($block:tt)* } $($tail:tt)*) => {
        points!(@helper {$($points,)*}, $suite, $($block)*);
        points!($($tail)*);
    };
    (#[points = {$($points:expr),* $(,)*}] test $test:ident $code:block $($tail:tt)*) => {
        #[test]
        fn $test() {
            use std::io::Write;
            let mut file = $crate::_FILE.lock().unwrap();
            let test = stringify!($test);
            $(
                write!(file, "{} = {}\n", test, $points).unwrap();
            )*
            $code
        }
        points!($($tail)*);
    };

    (@helper {$($spoints:expr,)*}, $suite:ident,) => {};
    (@helper {$($spoints:expr,)*}, $suite:ident, #[points = {$($points:expr),* $(,)*}] test $test:ident $code:block) => {
        #[test]
        fn $test() {
            use std::io::Write;
            let mut file = $crate::_FILE.lock().unwrap();
            let suite = stringify!($suite);
            let test = stringify!($test);
            let mut spoints = String::new();
            $(
                spoints.push_str($spoints);
                spoints.push_str(" ");
            )*
            let mut points = String::new();
            $(
                points.push_str($points);
                points.push_str(" ");
            )*
            write!(file, "{} = {}\n", suite, &spoints[..spoints.len()-1]).unwrap();
            write!(file, "{}.{} = {}\n", suite, test, &points[..points.len()-1]).unwrap();
            $code
        }
    };
    (@helper {$($spoints:expr,)*}, $suite:ident, #[points = {$($points:expr),* $(,)*}] test $test:ident $code:block $($tail:tt)*) => {
        #[test]
        fn $test() {
            use std::io::Write;
            let mut file = $crate::_FILE.lock().unwrap();
            let suite = stringify!($suite);
            let test = stringify!($test);
            let mut points = String::new();
            $(
                points.push_str($points);
                points.push_str(" ");
            )*
            write!(file, "{}.{} = {}\n", suite, test, &points[..points.len()-1]).unwrap();
            $code
        }
        points!(@helper {$($spoints,)*}, $suite, $($tail)*);
    };
}
