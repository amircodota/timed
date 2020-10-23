use std::{thread, time};
use timed;
use timed::TracingStats;

#[allow(dead_code)]
fn sleep() {
    thread::sleep(time::Duration::from_millis(10));
}

#[timed::timed(tracing = true)]
fn foo() {
    bar();
    sleep();
    baz();
}

#[timed::timed(tracing = true)]
fn bar() {
    sleep();
    baz();
    sleep();
}

#[timed::timed(tracing = true)]
fn baz() {
    sleep();
    foo::bar::baz::foobar();
}

pub mod foo {
    pub mod bar {
        pub mod baz {
            use timed::timed;
            #[timed(tracing = true)]
            pub fn foobar() {
                println!("Foobar");
            }
        }
    }
}

#[test]
#[timed::timed(tracing = true)]
fn test_tracing() {
    timed::init_tracing!("Test");

    println!("Running main");
    sleep();
    foo();
}

#[test]
#[timed::timed(tracing = true)]
fn test_tracing_with_stats() {
    timed::init_tracing!("TestWithStats", TracingStats::Statistics);

    println!("Running main");
    sleep();
    foo();
}
