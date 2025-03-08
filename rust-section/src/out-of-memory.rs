// When there is no space enough memory to allocate for a program use OOM error happens. id can lead to denial of service

// Happens when length of buffer si not checked
// There is no limit on unbounded data types (Array , Vec)
// To get rid from this we use mapping in the smart contract
fn main() {
    let mut a: vec<String> = Vec::<String>::new();

    a.push(String::from("A".repeat(usize::MAX)));
}