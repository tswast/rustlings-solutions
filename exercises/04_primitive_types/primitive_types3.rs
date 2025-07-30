fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let mut a : [i32; 1000] = [0; 1000];
    for (i, item) in a.iter_mut().enumerate() {
        *item = i32::try_from(i).expect("shouldnt happen");
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array! {:?}", a);
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
