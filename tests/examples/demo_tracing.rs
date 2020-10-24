#[timed::timed(tracing = true)]
fn foo() {
    bar();
    baz();
}

#[timed::timed(tracing = true)]
fn baz() {
    println!("Hello")
}

#[timed::timed(tracing = true)]
fn bar() {
    baz();
}

#[timed::timed(tracing = true)]
fn main() {
    timed::init_tracing!("Main", timed::TraceOptions::new()
    .with_chrome_trace(
        |x: &str| println!("{}", x)
    ).build());
    foo();
}
