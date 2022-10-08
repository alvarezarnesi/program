use std::io;

fn main() {
    
    let mut input_days = String::new();

    println!("Insert a number of days to covert to seconds:");

    match io::stdin().read_line(&mut input_days){
        Ok(_) => { 
            let days = input_days.trim().parse::<i64>().unwrap();
            let seconds = days*86400;
            println!("days: {}", days);
            println!("seconds: {}", seconds);
        },
        Err(e) => println!("ERROR: {}", e),
        
    }
}

