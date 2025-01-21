fn main() {
    // Create an array called `a` with fewer than 100 elements in it.
    // let a: [i32; 99] = [0; 99];
    let a: [i32; 100] = [0; 100];
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}