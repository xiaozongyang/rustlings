// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0 = Vec::new();

    let vec1: Vec<i32> = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    let mut vec1 = vec1;
    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32>{
    let mut vec = vec;

    let v = vec![22, 44, 66];
    // v.push(22);
    // v.push(44);
    // v.push(66);

    return v;
}
