#[macro_export]
macro_rules! assert_panic {
    ($msg:expr ; $($arg:tt)*) => {
        {
            let may_panic = || { $($arg)* }; // create a closure
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(may_panic));
            assert!(result.is_err(), "{}", $msg);
        }
    };
    ($($arg:tt)*) => {
        {
            let may_panic = || { $($arg)* }; // create a closure
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(may_panic));
            assert!(result.is_err(), "Didn't panic");
        }
    };
}
