use cardodds::hypergeometric::probability_at_least_one;

fn main() {
    let prob = probability_at_least_one(60, 7, 4);
    println!("{:.4}", prob);
}
