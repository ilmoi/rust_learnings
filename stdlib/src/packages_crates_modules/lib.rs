//ch7 was about modules - so lib.rs relates to ch7

//mod ch7 tells rust to use contents from another file with the same name
//pub before mod tells rust to ALSO export it further out
//taken together this is like export (import) from js
pub mod ch7;

// -----------------------------------------------------------------------------
// stuff from main
use rustbook::ch7;
use rustbook::ch7::testmod;

fn main() {
    println!("Hello, world!");

    ch7::test();
    ch7::testmod::deeptest();
    testmod::deeptest();
}