use mackerel_proc_macros::call_trace;

#[call_trace]
fn hello_world() {
    println!("hello worldç");
}

#[test]
fn it_should_add_some_code_to_hello_world() {
    // cargo test -p mackerel-proc-macros -- --nocapture
    hello_world();
}
