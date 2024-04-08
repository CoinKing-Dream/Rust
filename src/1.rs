use rand::Rng;
use std::env;

fn main(){
    // previous();
    // loop_statement();
    // ----- now coding ------
    tuple_type();

}

fn previous() {
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
}

fn loop_statement() {
    let mut number = 0;

    loop {
        number = number + 1;
        if number == 9 {
            break;
        }

        println!("{}", number);
    }
}

fn tuple_type() {
    let em: (u32, &str) = (1, "John");
    println!("LST I am here");
    println!("{:#?}", em);
    println!("{}", em.0);
    println!("{:#?}", em.1);
}

