use std::io::stdin;


fn main() {
    println!("Enter any number");

    let secret_number = 28;

    let mut number = String::new();

    stdin().read_line(&mut number).expect("Failed to read input");

    let number = number.split_whitespace().collect::<String>().parse::<i32>().unwarp();

    if number% 2 != 0 {
        println!("Ony even number are expected");
    }

    if number > 10000 {
        unimplemented!();
    }

    assert!(number == secret_number, "You picked wrong number");

    println!("Entered number is {number}");


}