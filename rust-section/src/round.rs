const FEE: f32 = 0.075;
const PER_DAY: u16 = 2;

pub fn get_rewards(days: u16) -> f32 {
    (days * PER_DAY) as f32 * FEE
}


fn main() {
    // 1.5 round 2
    let rewards_round = get_rewards(10).round();

    // 1.5 round 2
    let rewards_ceil = get_rewards(10).ceil();

    // 1.5 rounded 1
    let rewards_floor = get_rewards(10).floor();

    println!("Rewards rounded {rewards_round}");
    println!("Rewards ceiled {rewards_ceil}");
    println!("Rewards floored {rewards_floor}")
}