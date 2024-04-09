fn main() {
    String_Practice();
}

fn String_Practice() {
    let mut str = String::new();
    str.push_str("Hello");
    println!("{}", str);

    let add_str = "TutorialPoint".to_string();
    str.push_str("how");
    println!("{}", str);

    let mut token_number = 1;
    let msg = "How are you?".to_string();
    let tokens:Vec<&str> = msg.split_whitespace().collect();

    for token in msg.split_whitespace(){
        println!("token {} {}", token_number, token);
        token_number += 1;
    }

    println!("{}", tokens[2]);

    ref_func(5);
    
    tupple_func();

    array_func();
}

fn ref_func(mut val:i32) {
    val = val + 2;
    println!("{}", val);
}

fn tupple_func(){
    let tuple:(i32, f64, u8) = (-32, 2.3, 98);
    let temp:(i32, f64, u8);

    temp = tuple;

    println!("{:?}", temp.0);
    println!("{:?}", tuple.1);
    println!("{:?}", tuple.2);
}

fn array_func(){
    let arr:[i32;4] = [1, 2, 3, 4];
    
    for var in arr.iter(){
        println!("{}", var);
    }
}