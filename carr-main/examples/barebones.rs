use carr::ArrOwn;

fn main() {
    let arr1 = ArrOwn::<i32, { &[2, 2] }>::from([7, 2, 3, 5].into());
    let arr2 = ArrOwn::<i32, { &[2, 2] }>::new_ones();
    println!("Arr1: {arr1}\nArr2: {arr2}\n---\n");
    let arr3 = arr1.view_all() - arr2.view_all();
    println!("Arr1 - Arr2: {arr3}");
    let arr4 = arr1.view_all() % arr3.view_all();
    println!("Arr1 % (Arr1 - Arr2): {arr4}");
}