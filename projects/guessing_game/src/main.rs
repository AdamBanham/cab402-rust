use std::io;
use std::cmp::Ordering;
use rand::Rng;

// program
fn main() {
    println!("Guess the number!");
    // make a random number between 1 and 100
    let secret_number = rand::thread_rng()
        .gen_range(1,101);
    // keep asking questions until 
    // user guesses number
    loop {
        println!("Please input your guess.");
        // create a mutable reference , 
        // which will be shadowed
        // between (mut(string) -> imut(string) -> imut(u32))
        let mut guess = String::new();
        // chain output from one function into another
        // much like a functional pipe |> in F#
        io::stdin() // get a user input
            // read user input after enter
            .read_line(&mut guess)
            // crash if failed
            .expect("Failed to read line"); 
        // handle string type to 
        // unsigned 32-bit number
        let guess: u32 = match guess.trim() 
        // trim string of \n then parse to u32
        // then match result to either
        // Ok type or Err type
            .parse() {
                Ok(num) => num, 
                Err(_) => continue,
            };
        // if Ok then keep num
        // otherwise reset loop
        //display guess                            
        println!("You guessed: {}", guess);
        // match the result of comparing to resulting
        // Ordering enum types                          
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                // guess less than
                println!("Too small!")
            },
            Ordering::Greater => {
                 // guess greater than
                println!("Too big!")
            }, 
            Ordering::Equal => { 
                // guess was equal to
                println!("You win!");
                break;
                 // close loop and end program
            }
        }
    }
} 
// end of program



