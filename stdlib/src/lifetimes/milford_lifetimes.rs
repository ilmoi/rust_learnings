fn main() {
    /*
    https://www.youtube.com/watch?v=1QoT9fmPYr8&list=PLLqEtX6ql2EyPAZ1M2_C0GgVd4A-_L4_5&index=14
    lifetimes are about ensuring that a memory doesn't get cleaned up BEFORE A REF CAN USE IT
     */

    //lifetime subtyping - tell the compiler that 'b will live at least as long as 'a
    fn some_fn<'a, 'b:'a>(param_a: &'a i32, param_b: &'b i32) -> &'a i32;

    //but a better way is to get rid of 'b alltogether
    fn some_fn2<'a>(param_a: &'a i32, param_b: &'a i32) -> &'a i32;

    /*
    Reminder of 3 lifetime rules:
    1. Each &param gets its own lifetime
    2. If there is only one &input, then the &output gets that lifetime
    3. If there is a &self or &mut self input, then the &output gets that lifetime
     */

    //note that lifetimes on structs work just the same
    struct MyStruct<'a, 'b> {
        x: String,
        y: &'a String,
        z: &'b String,
    }

}