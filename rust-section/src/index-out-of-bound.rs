fn main() {
    let vec = vec![1,2,3,4,4,5,6,7];

    // last element
    println!("{}", vec[vec.len() - 1]);

    // Index out of bounds
    // println!("{}", vec[vec.len()]);

    // .get() method
    let val = vec.get(vec.len());

    match val {
        Some(val) => println!("{val}"),

        None => println!("Index out of bound. Please try again"),
    }
}