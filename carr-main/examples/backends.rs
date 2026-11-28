use carr::backend::{ConstantBackend, FnBackend};
use carr::prelude::*;

fn main() {
    let arr_owned = Arr::<
        /* Backend = */ Vec<i32>,
        /* Data Type = */ i32,
        /* Dimensions */ { &[1, 3] },
    >::new_ones();

    println!("Owned backend array: {arr_owned}");

    // Backend of a view is `Vec<&T>`, where `T` is the data type of the array being viewed.
    let arr_view = arr_owned.view_all();
    println!("View backend array: {arr_view}");

    // Constant backends allow for a backend that infinitely returns the same element.
    //
    // This one has a datatype of &'static str, and a backend type of
    // `ConstantBackend<&'static str>`
    let const_backend = ConstantBackend::new(&"Hello, world!");
    let mut iter = const_backend.into_iter_all();
    assert_eq!(iter.next(), Some("Hello, world!"));
    assert_eq!(iter.next(), Some("Hello, world!"));
    assert_eq!(iter.next(), Some("Hello, world!"));
    assert_eq!(iter.next(), Some("Hello, world!"));
    assert_eq!(iter.next(), Some("Hello, world!"));
    assert_eq!(iter.next(), Some("Hello, world!"));
    // ...and so on and so forth, forever

    // These can be used as a backend like so:
    let s: String = "Hi there hello".to_owned();
    let arr_const = Arr::<
        /* Backend = */ ConstantBackend<String>,
        /* Data Type = */ String,
        /* Dimensions */ { &[3, 2] },
    >::from(ConstantBackend::new(&s));
    println!("Const backend array: {arr_const}");

    // Functional backends take a closure `FnMut(usize) -> T` to map indices to a value. A simple
    // example is below:
    let func_backend = FnBackend::new(|i| 2 * i);
    let arr_fn = Arr::<
        /* Backend = */ FnBackend<usize, /* Closure Type = */ _>,
        /* Data Type = */ usize,
        /* Dimensions */ { &[2, 3] },
    >::from(func_backend);
    println!("Functional backend array: {arr_fn}");
}
