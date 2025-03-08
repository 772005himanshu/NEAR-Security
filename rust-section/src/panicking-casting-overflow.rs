use primitive_type::U256;

fn mina() {
    // using custom u256 type from primitive type library

    let a = U256::MAX;

    // Panics on casting overflow
    println!("{}", a.as_u128());
}