use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // The line below generating the random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess a number between 1 = 100 🤔!");

    loop {
        // it will give you new String() instance
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input"); // Result:: <user_input>\n

        
        // ! The following variable `guess` will behave shadow variable. That means, the `guess` variable in TOP is removed from the memory  after then it will TIMED and PARSED then store it into guess variable. So those are not same guess. First on will removed after taking and placing it to new guess variable
          
          // If the method failed to parse the it will match the `Err()` block and continuing to ask the user for input
         // If it is number then it will just parse. SIMPLE ^:)
         
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid input");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You won the guessing game!");
                break;
            },
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small. Think BIG!"),
        }
    }
}
