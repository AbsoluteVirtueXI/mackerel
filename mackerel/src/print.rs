// TODO add ansi_term/colored/termcolor colors to the print macro's
// TODO p! should accept a list of expression
// Insert debug printing as a "d:" or "debug:" tag
//
// maybe should also be used by log module.
pub enum Color {}

#[macro_export]
macro_rules! p {
    () => {
        println!();
    };
    (?$($xpr:expr),+ $(,)?) => {
        $( println!("{:?}", $xpr); )*
    };
    (=$($xpr:expr),+ $(,)?) => {
        $( println!("{} = {}", stringify!($xpr), $xpr); )*
    };
    // TODO: find a way to handle "?="" and "=?" in a one pattern
    (=?$($xpr:expr),+ $(,)?) => {
        $( println!("{} = {:?}", stringify!($xpr), $xpr); )*
    };
    (?=$($xpr:expr),+ $(,)?) => {
        $( println!("{} = {:?}", stringify!($xpr), $xpr); )*
    };
    ($($xpr:expr),+ $(,)?) => {
        $( println!("{}", $xpr); )*
    }
}

#[macro_export]
macro_rules! ep {
    () => {
        eprintln!();
    };
    (?$($xpr:expr),+ $(,)?) => {
        $( eprintln!("{:?}", $xpr); )*
    };
    (=$($xpr:expr),+ $(,)?) => {
        $( eprintln!("{} = {}", stringify!($xpr), $xpr); )*
    };
    (=?$($xpr:expr),+ $(,)?) => {
        $( eprintln!("{} = {:?}", stringify!($xpr), $xpr); )*
    };
    (?=$($xpr:expr),+ $(,)?) => {
        $( eprintln!("{} = {:?}", stringify!($xpr), $xpr); )*
    };
    ($($xpr:expr),+ $(,)?) => {
        $( eprintln!("{}", $xpr); )*
    }
}
