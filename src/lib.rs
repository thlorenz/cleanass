mod panicking;

#[macro_export]
macro_rules! assert {
    // Cleanup versions of standard assertions
    ($cond:expr, $closure:expr) => {
        if !$cond {
            $closure();
        }
        ::core::assert!($cond)
    };
    ($cond:expr, $closure:expr, $($arg:tt)+) => {
        if !$cond {
            $closure();
        }
        ::core::assert!($cond, $($arg)+)
    };
    // Standard Assertions
    ($cond:expr) => {
        if !$cond {
            ::core::assert!(cond)
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            ::core::assert!(cond, $($arg)+)
        }
    };
}

#[macro_export]
macro_rules! assert_eq {
    // Cleanup versions of standard assertions
    ($left:expr, $right:expr, $closure:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $closure();
                }
            }
        }
        ::core::assert_eq!($left, $right)
    };
    ($left:expr, $right:expr, $closure:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $closure();
                }
            }
        }
        ::core::assert_eq!($left, $right, $($arg)+)
    };

    // Standard Assertions
    ($left:expr, $right:expr $(,)?) => {
        ::core::assert_eq!($left, $right)
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        ::core::assert_eq!($left, $right, $($arg)+)
    };

}

#[macro_export]
macro_rules! assert_ne {
    // Cleanup versions of standard assertions
    ($left:expr, $right:expr, $closure:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    $closure();
                }
            }
        }
        ::core::assert_ne!($left, $right)
    };
    ($left:expr, $right:expr, $closure:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    $closure();
                }
            }
        }
        ::core::assert_ne!($left, $right, $($arg)+)
    };

    // Standard Assertions
    ($left:expr, $right:expr $(,)?) => {
        ::core::assert_ne!($left, $right)
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        ::core::assert_ne!($left, $right, $($arg)+)
    };
}
