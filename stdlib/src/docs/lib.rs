// -----------------------------------------------------------------------------
// documentation
// fns need to come from this lib file (so defined here or imported into here)
// fns also need to be PUB to be visible in the docs - private aren't
// anything in main is ignored

//! comment on rustbook itself

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = rustbook::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// This wont generate docs even though I wrote them:(
fn less_one(x: i32) -> i32 {
    x + 1
}