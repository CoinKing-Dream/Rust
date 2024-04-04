use rand::Rng;
use std::env;

fn main(){
    let day = String::from("Welcome");
    let random_number = rand::thread_rng().gen_range(1..=10);
    println!("Generate Random Number 1 and 10: {}", random_number);
    
    enum WEEKEND{
        SUNDAY,
        MONDAY,
        TUESDAY,
        WEDNESDAY,
        THURSDAY,
        FRIDAY,
        SATURDAY
    }

    let date = 5;

    match date{
        1..=4 => println!("{}", date),
        5 | 6 => println!("wrong"),
        _ => println!("Invalid!")

    }

    match day.as_str() {
        "Welcome" => {
            println!("Welcome!");
            // Additional logic for "Welcome" case
        },
        _ => {
            println!("Invalid day: {}", day);
            // Additional logic for other cases
        }
    }

    let weekend = WEEKEND::SUNDAY;

    match weekend {
        WEEKEND::SUNDAY | WEEKEND::SATURDAY => {
            println!("OK");
        }
        _ => {
            println!("Invalid weekday");
        }

    }

    const Number: i32 = 20;
    const Floating_Number: f32 = 20.0;

    println!("{}", Number);
    println!("{}", Floating_Number);

    let array = [1,2,3,4,5];

    for  item in array{
        println!("{}", item);
    }

    for item in 1..=10 {
        println!("{}", item);
    }

    let numbers: &[i32] = &vec![1, 2, 3, 4, 5, 6];

    for (i, item) in numbers.iter().enumerate() {
        println!("value => {},  Index => {}", item, i);
    }

    
    let add = |first: u32, second: u32| -> u32 {
        return first;
    };
    
    println!("{}", add(12, 25));

    // let employ: (u32, &str, bool) = (1, "john", true);

    // let args: Vec<String> = env::args().collect();

    // let name = &args[1];
    // let filename = &args[2];

    // println!("Name is {}", name);
}

