use rand::Rng;
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


}