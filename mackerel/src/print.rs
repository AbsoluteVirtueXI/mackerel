// TODO add ansi_term/colored/termcolor colors to the print macro's
// maybe should also be used by log module.
pub enum Color {}

#[macro_export]
macro_rules! p {
    ($a:expr) => {
        println!("{}", $a);
    };
}

#[macro_export]
macro_rules! ep {
    ($a:expr) => {
        eprintln!("{}", $a)
    };
}

#[macro_export]
macro_rules! dp {
    ($a:expr) => {
        println!("{:?}", $a);
    };
}

#[macro_export]
macro_rules! dep {
    ($a:expr) => {
        eprintln!("{:?}", $a)
    };
}
