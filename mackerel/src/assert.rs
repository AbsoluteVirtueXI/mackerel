// TODO: Debug and Test

use std::fmt;

#[derive(Debug)]
pub enum AssertKind {
    Lt,
    Le,
    Ge,
    Gt,
}

#[macro_export]
macro_rules! assert_lt {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val < *right_val) {
                    let kind = $crate::assert::AssertKind::Lt;
                    $crate::assert::assert_failed(kind, &*left_val, &*right_val, std::option::Option::None);
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val < *right_val) {
                    let kind = $crate::assert::AssertKind::Lt;
                    $crate::assert::assert_failed(kind, &*left_val, &*right_val, std::option::Option::Some(std::format_args!($($arg)+)));
                }
            }
        }
    };
}

#[macro_export]
macro_rules! assert_le {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val <= *right_val) {
                    let kind = $crate::assert::AssertKind::Le;
                    $crate::assert::assert_failed(kind, &*left_val, &*right_val, std::option::Option::None);
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val <= *right_val) {
                    let kind = $crate::assert::AssertKind::Le;
                    $crate::assert::assert_failed(kind, &*left_val, &*right_val, std::option::Option::Some(std::format_args!($($arg)+)));
                }
            }
        }
    };
}

#[macro_export]
macro_rules! assert_ge {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val >= *right_val) {
                    let kind = $crate::assert::AssertKind::Ge;
                    $crate::assert::assert_failed(kind, &*left_val, &*right_val, std::option::Option::None);
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val >= *right_val) {
                    let kind = $crate::assert::AssertKind::Ge;
                    $crate::assert::assert_failed(kind, &*left_val, &*right_val, std::option::Option::Some(std::format_args!($($arg)+)));
                }
            }
        }
    };
}

#[macro_export]
macro_rules! assert_gt {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val > *right_val) {
                    let kind = $crate::assert::AssertKind::Gt;
                    $crate::assert::assert_failed(kind, &*left_val, &*right_val, std::option::Option::None);
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val > *right_val) {
                    let kind = $crate::assert::AssertKind::Gt;
                    $crate::assert::assert_failed(kind, &*left_val, &*right_val, std::option::Option::Some(std::format_args!($($arg)+)));
                }
            }
        }
    };
}

/// Internal function for `assert_lt!`, `assert_le!`, `assert_ge!` and `assert_gt!` macros
#[cold]
#[track_caller]
#[doc(hidden)]
pub fn assert_failed<T, U>(
    kind: AssertKind,
    left: &T,
    right: &U,
    args: Option<fmt::Arguments<'_>>,
) -> !
where
    T: fmt::Debug + ?Sized,
    U: fmt::Debug + ?Sized,
{
    assert_failed_inner(kind, &left, &right, args)
}

/// Non-generic version of the above functions, to avoid code bloat. => TODO match above
#[track_caller]
fn assert_failed_inner(
    kind: AssertKind,
    left: &dyn fmt::Debug,
    right: &dyn fmt::Debug,
    args: Option<fmt::Arguments<'_>>,
) -> ! {
    let op = match kind {
        AssertKind::Lt => "<",
        AssertKind::Le => "<=",
        AssertKind::Ge => ">=",
        AssertKind::Gt => ">",
    };

    match args {
        Some(args) => panic!(
            r#"assertion failed: `(left {} right)`
  left: `{:?}`,
 right: `{:?}`: {}"#,
            op, left, right, args
        ),
        None => panic!(
            r#"assertion failed: `(left {} right)`
  left: `{:?}`,
 right: `{:?}`"#,
            op, left, right,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
