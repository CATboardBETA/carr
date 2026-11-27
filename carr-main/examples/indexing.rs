use carr::ArrOwn;
use carr::prelude::*;

fn main() {
    let arr = ArrOwn::<i32, { &[2, 5] }>::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into());
    println!("Arr: {arr}");
    // Indexing with no indices is the same as arr.view_all()
    ix!(arr => []);
    // The first item of the first dimension, and the first item of the second dimension, respectably.
    let first_first = ix!(arr => [0, 0]);
    let first_second = ix!(arr => [1, 0]);
    println!("Arr[0, 0]: {first_first}");
    println!("Arr[1, 0]: {first_second}");
    // This index doesn't exist. With the standard indexing macro, this wouldn't compile. With
    // try_ix, however, it will return `Option::None`.
    let first_third = try_ix!(arr => [2, 0]);
    println!("Arr[2, 0]: {first_third:?}");
    // If the indexing slice has a small length than the dimensions of the source array, it will
    // slice it.
    let first_dim = ix!(arr => [0]);
    let second_dim = ix!(arr => [1]);
    println!("Arr[0]: {first_dim}");
    println!("Arr[1]: {second_dim}");
    // Like above, this slice doesn't exist. It will return `Option::None`
    let third_dim = try_ix!(arr => [2]);
    println!("Arr[2]: {third_dim:?}");
}