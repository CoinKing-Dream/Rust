use rand::Rng;
fn main(){
    let random_number = rand::thread_rng().gen_range(1..=10);
    println!("Generate Random Number 1 and 10: {}", random_number);
}