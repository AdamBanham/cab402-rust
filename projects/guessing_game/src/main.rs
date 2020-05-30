use std::io;
use std::cmp::Ordering;
use rand::Rng;

// program
fn main() {
    println!("Guess the number!");
    // make a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1,101);
    // keep asking questions until user guesses number
    loop {
        println!("Please input your guess.");
        // create a mutable reference , which will be shadowed
        // between (mut(string) -> imut(string) -> imut(u32))
        let mut guess = String::new();
        // chain output from one function into another
        // much like a functional pipe |> in F#
        io::stdin() // get a user input
            .read_line(&mut guess) // read user input after enter
            .expect("Failed to read line"); // crash if failed
        // handle string type to unsigned 32-bit number
        let guess: u32 = match guess.trim() 
                                    // trim string of \n then parse to u32
                                    // then match result to either
                                    // Ok type or Err type
                                    .parse() {
                                        Ok(num) => num, // if Ok then keep num
                                        Err(_) => continue, // otherwise reset loop
                                    };
        //display guess                            
        println!("You guessed: {}", guess);
        // match the result of comparing to resulting
        // Ordering enum types                          
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // guess less than
            Ordering::Greater => println!("Too big!"), // guess greater than
            Ordering::Equal => { // guess was equal to
                 println!("You win!");
                 break; // close loop and end program
            }
        }
    }
} 
// end of program
