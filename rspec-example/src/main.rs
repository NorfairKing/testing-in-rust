use std::env;
use std::{thread::sleep, time::Duration};

use rspec;
fn main() {
    println!("Hello World");
    rspec::run(&rspec::describe("foo", (), |ctx| {
        ctx.it("pure boolean", |_| true);
        ctx.it("return boolean", |_| {
            return true;
        });
        ctx.it("return unit", |_| {
            return ();
        });
        ctx.it("sleeping", |_| {
            sleep(Duration::from_secs(2));
        });
        ctx.it("diff", |_| {
            assert_eq!(5, 6);
        });
        ctx.it("printing", |_| {
            println!("Hi");
        });
        ctx.it("args", |_| {
            println!("{:?}", env::args().collect::<Vec<String>>());
        });
    }));
}
