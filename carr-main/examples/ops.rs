use carr::ArrOwn;

fn main() {
    let arr_2x3 = ArrOwn::<i32, /* Dims = */ { &[3, 2] }>::from([7, 2, 3, 5, 1, 9].into());
    println!("Array 1: {arr_2x3}");
    let arr_2 = ArrOwn::<i32, { &[2] }>::new_zeroed();
    println!("Array 2: {arr_2}");
    // Unsqueeze a new dimension at the beginning of the array
    let arr_1x2 = arr_2.unsqueeze::<0>();
    println!("Array 2, unsqueezed: {arr_1x2}");
    let mut concatenated = arr_2x3.concat::<0, _, _>(&arr_1x2);
    println!("Concatenated: {concatenated}");
    // Set each element to its flat index
    for (i, x) in concatenated.iter_mut().enumerate() {
        *x = i as i32
    }
    println!("Indices: {concatenated}")
}