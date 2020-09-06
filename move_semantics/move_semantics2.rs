// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    let mut vec0 = Vec::new();
    fill_vec(&mut vec0);
    // 1. make another copy of vec0 and move that instead
    //let tmp = vec0.to_vec();
    //let mut vec1 = fill_vec(tmp);
    //
    //
    //
    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}




// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     // 2. pass by address
//     let mut vec = vec.to_vec();
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }
fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
