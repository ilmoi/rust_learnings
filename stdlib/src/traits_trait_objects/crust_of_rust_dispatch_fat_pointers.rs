pub fn main() {
    /*
    https://www.youtube.com/watch?v=xcygqF5LVmM

    Static dispatch / monomorphization
    - "static dispatch" = when compiler knows what method we're calling at compile time
    - "monomorphization" = have a fn using generics + trait bounds > rust generates a nongeneric implementation for each concrete type found in our code
    - happens not just for fns, but eg for structs like HashMaps too
    - is why it's hard to distro rust libs as binaries - because we don't know what type it will be called with by the consumer, and so can't pregen the type
    - is also why you can't do dynamic libraries that use generics - rust doesn't know when compiling a DLL what the generic will be and can't monoporphize it
    - upside: faster code at runtime
    - downside: larger binary, slower compile time
    */

    // note how we need both PatialOrd and Copy traits here implemented on T for the fn to work
    // we call these "trait parameters"
    // there are 3 ways to add them: 1)&impl, 2)like below, and 3)using "where"
    // more: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#traits-as-parameters
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { //ALL components of the list must be of type T
        let mut largest = list[0];
        for &i in list {
            if i > largest {
                largest = i;
            }
        }
        largest
    }

    //under the hood the largest<T> fn gets monomorphized into largest_i32 and largest_str
    let v_int = vec![1,2,3];
    let v_str = vec!["a","b","c"];
    println!("largest int is {}", largest(&v_int));
    println!("largest str is {}", largest(&v_str));

    /*
    Dynamic dispatch / trait objects
    - "dynamic dispatch" = when compiler doesn't know what method will be called until runtime. Trait objects force rust to use dynamic dispatch
    - "trait objects" = pointer to both an instance of a type implementing our specified trait, and a table used to look up trait methods on that type at runtime
    - similar to "duck typing" in dynamic libraries, except once we get past compile time we know for sure that a fn has the desired method/trait
    - you can convert something to a trait object, but not back (w/o unsafe). we call this "trait erasure"
    - requires "object-safe" traits, which means 1)they dont return self, 2)they dont use any generics in them, 3)they start with self
    - upside: smaller binary, faster compile time
    - downside: slower code at runtime, no code inlining
    */

    // we specify a trait object by using a pointer (such as Box<> or &) followed by dyn followed by the trait we care about. YOU CAN USE EITHER & OR BOX<>!!!
    // NOTE: a generic type param can only be sub'ed for one concrete type at a time - whereas trait objects allow for multiple concrete types to fill in at runtime, as long as they implement the trait
    // ^THIS IS THE BIGGEST DIFF BETWEEN THE 2. TRAIT OBJ LET YOU WRITE CODE W/O WORRYING ABOUT THE TYPE YOU'LL OPERATE IT ON.
    fn largest_dyn<'a>(list: &'a [Box<dyn AsInt + 'a>]) -> Box<dyn AsInt + 'a> {
        let mut largest = list[0].box_clone();
        for i in list {
            if i.as_int() > largest.as_int() {
                largest = i.box_clone();
            }
        }
        largest
    }
    // for full example see static_to_dynamic_dispatch_example

    //to summarize, difference in syntax is:
    // fn largest<T: Copy>(list: &[T]) -> T
    // fn largest(list: &[Box<dyn Copy>]) -> Box<dyn Copy>

    /*
    Can't have a trait object with 2 traits - because each trait has a separate vtable and so we'd need 2 vtables
    Instead you get around it like below:
    You create a custom trait which in it combines 2 traits and has no methods of its own
    And so now you have one single vtable for SomeTrait, with 2 methods for each concrete type
     */
    trait SomeTrait: FirstTrait + SecondTrait {}

    /*
    This is diff for marker traits (traits w/o methods) - Send, Sync, Copy, Sized
    We can have a trait object that has a trait + a marker trait referenced like so: Box<dyn FirstTrait + Send>
    The reason we can add a 2nd marker trait is because marker traits don't have any methods and so don't have a vtable that would take up pointer space
     */
    fn largest_dyn2(list: &[Box<dyn PartialOrd<i32> + Send>]) {};
    //see "trait_obj_w_second_marker_trait.rs" for a working example

    /*
    If our trait object has an associated type - we have to specify that type when we specify the trait in Box<>
    That's because otherwise there is nowhere to specify that type - we can't put it in the vtable
    We'd specify like so:
     */
    fn some_fn(x: Box<dyn SomeTrait<TypeName = ()>>) {};

    /*
    Normally all fns on a trait obj have to be trait-safe, which has 2 requirements:
    1. must reference self
    2. can't contain any generics
    3. can't return self

    Why is that?
    1. Unless we have a &self we can't build a vtable with a reference to a concrete type. What are we referencing the table back to?
    2. Remember that generics go through monomorphization under the hood, which means the vtable would have to have every possible monomorphized method in it = infinite number of entries
    3. We don't know the size of Self that would have to be returned, remember Self isn't sized, that's whe whole reason we wrap it in a pointer

    We can actually get around the 1st requirement like so:
     */
    trait Hei {
        fn hei(&self); //trait-safe
        fn weird(); //not trait-safe coz doesn't take &self - will throw a compile error
        fn weird2() where Self: Sized {}; //unless we do this - this will exclude the fn from the vtable and make it uncallable from the object
        fn hei2(&self) where Self: Sized {}; //in fact we could use the above where clause to opt out from trait obj for a otherwise perfectly functional fn

        //TAKEAWAY: if you ever see where Self: Sized - you know that's a way of telling the compiler "don't include this method in any trait objects you might build from this trait"
    }

    /*
    Any vtable for a concrete type includes:
    1. drop method
    2. size
    3. alignment
    (2) and (3) needed for (1) to work
     */

}

