#[macro_export]
macro_rules! print_xpr {
    ($a:expr) => {
        println!("{}", $a);
    };
}

#[macro_export]
macro_rules! eprint_xpr {
    ($a:expr) => {
        eprintln!("{}", $a)
    };
}

#[macro_export]
macro_rules! debug_print_xpr {
    ($a:expr) => {
        println!("{:?}", $a);
    };
}

#[macro_export]
macro_rules! debug_eprint_xpr {
    ($a:expr) => {
        eprintln!("{:?}", $a)
    };
}
