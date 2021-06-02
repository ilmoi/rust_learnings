/*
To run a function:
1. create a .rs file in respective folder
2. add it to mod.rs inside that folder
3. add folder name to lib.rs
4. access the folder name below starting with _learning::
 */

#[tokio::main]
async fn main() {
    // _learning::traits_trait_objects::crust_of_rust_dispatch_fat_pointers::main();
    // _learning::lifetimes::crust_of_rust_lifetimes::main();
    // _learning::iterators::crust_of_rust_iterators::main();
    // _learning::actix_web::actix_web::main();
    _learning::play_tokio::play_tokio::main().await;
}