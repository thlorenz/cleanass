#[macro_export]
#[cfg(feature = "strict")]
macro_rules! assert {
    // Cleanup versions of standard assertions
    ($cond:expr, $cleanup:stmt $(,)?) => {
        if !$cond {
            $cleanup;
        }
        ::core::assert!($cond)
    };
    ($cond:expr, $cleanup:stmt, $($arg:tt)+) => {
        if !$cond {
            $cleanup;
        }
        ::core::assert!($cond, $($arg)+)
    };
}

#[macro_export]
#[cfg(not(feature = "strict"))]
macro_rules! assert {
    // Cleanup versions of standard assertions
    ($cond:expr, $cleanup:stmt $(,)?) => {
        if !$cond {
            $cleanup;
        }
        ::core::assert!($cond)
    };
    ($cond:expr, $cleanup:stmt, $($arg:tt)+) => {
        if !$cond {
            $cleanup;
        }
        ::core::assert!($cond, $($arg)+)
    };
    // Standard Assertions
    ($cond:expr $(,)?) => {
        if !$cond {
            ::core::assert!($cond)
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            ::core::assert!($cond, $($arg)+)
        }
    };
}

#[cfg(feature = "strict")]
#[macro_export]
macro_rules! assert_eq {
    // Cleanup versions of standard assertions
    ($left:expr, $right:expr, $cleanup:stmt $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $cleanup;
                }
            }
        }
        ::core::assert_eq!($left, $right)
    };
    ($left:expr, $right:expr, $cleanup:stmt, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $cleanup;
                }
            }
        }
        ::core::assert_eq!($left, $right, $($arg)+)
    };
}

#[cfg(not(feature = "strict"))]
#[macro_export]
macro_rules! assert_eq {
    // Cleanup versions of standard assertions
    ($left:expr, $right:expr, $cleanup:stmt $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $cleanup;
                }
            }
        }
        ::core::assert_eq!($left, $right)
    };
    ($left:expr, $right:expr, $cleanup:stmt, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $cleanup;
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

#[cfg(feature = "strict")]
#[macro_export]
macro_rules! assert_ne {
    // Cleanup versions of standard assertions
    ($left:expr, $right:expr, $cleanup:stmt $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    $cleanup;
                }
            }
        }
        ::core::assert_ne!($left, $right)
    };
    ($left:expr, $right:expr, $cleanup:stmt, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    $cleanup;
                }
            }
        }
        ::core::assert_ne!($left, $right, $($arg)+)
    };
}

#[cfg(not(feature = "strict"))]
#[macro_export]
macro_rules! assert_ne {
    // Cleanup versions of standard assertions
    ($left:expr, $right:expr, $cleanup:stmt $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    $cleanup;
                }
            }
        }
        ::core::assert_ne!($left, $right)
    };
    ($left:expr, $right:expr, $cleanup:stmt, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    $cleanup;
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
