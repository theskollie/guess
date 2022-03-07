use std::io;
use std::cmp::Ordering;
//Random Number Generator
use rand::Rng;
//Colored Terminal Output
use colored::*;

fn main() {
    println!("{}", "Guessing Game!".red());

    // Game Config Variables
    let total_allowed_guesses = guess_select();
    let bet_amount: f32 = bet_amount(total_allowed_guesses);
    //Current User Guesses
    let mut total_guesses = 0;
    //Randomly Generated Number from 1-100
    let secret_number = rand::thread_rng().gen_range(1..101);
  
    // User Input
    println!("{}", "Please input your guess. Range: 0-100".green());
     
    // Loop Until Win or Lose
    loop {

    //Mutatable User Guess    
    let mut guess = String::new();

    
    io::stdin()
        .read_line(&mut guess) //Read User Input from Terminal and Store in Guess Variable
        .expect("Failed to read line"); //Catch All Errors

    //Convert Guess String to Unsigned 32Bit Integer
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue, //Catch all errors
    };

    // Match Guess to 3 Outcomes - Too Large, Too Small, Equal
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            //Increase Total Current Guesses
            total_guesses = total_guesses + 1;
            //If all guesses are used, call end game function and break out loop
            if total_guesses == total_allowed_guesses
            {
                game_over(secret_number, total_allowed_guesses);
                break;

            }
            else {
            //User Input lower than Secret Number
            println!("{}", "Too low!".yellow());
            println!("Guesses: {}/{}. Try again!", total_guesses, total_allowed_guesses);
            }
        }
        Ordering::Greater => 
        {   
            //Increase Total Current Guesses
            total_guesses = total_guesses + 1;
             //If all guesses are used, call end game function and break out loop
            if total_guesses == total_allowed_guesses
            {
                game_over(secret_number, total_allowed_guesses);
                break;
            }
            else {   
            //User Input higher than Secret Number 
            println!("{}" ,"Too big!".red());
            println!("Guesses: {}/{}. Try again!", total_guesses, total_allowed_guesses);
            }
        }
        Ordering::Equal => {
        //Call Game Win Function and pass Bet Amount and Total Allowed Guesses - To determine win amount multiplier
        game_win(bet_amount, total_allowed_guesses);
        break;    
        }
    }

    }


}

fn game_over(secret: u32, total_allowed: u32) {
    //End Game and display Secret Number
    println!("{}", "...GAME OVER...".red());
    println!("You've used all {} guesses.", total_allowed);
    println!("The Secret Number was {}", secret);
        
}

fn guess_select () -> u32 {
    //User selects total guesses allowed - Lower guesses = higher multiplier
    println!("{}", "4 Guesses = 3x Profit".yellow());
    println!("5 Guesses = 2x Profit");
    println!("6 Guesses = 1.5x Profit");


    println!("How many guesses would you like? Range: 4-6");
    let mut total_allowed_guesses = String::new();

    io::stdin()
        .read_line(&mut total_allowed_guesses)
        .expect("Failed to read line");

    let mut total_allowed_guesses: u32 = total_allowed_guesses.trim().parse().unwrap();

    //If user inputs value higher than 6, set it to 6
    if total_allowed_guesses > 6 {
        total_allowed_guesses = 6;    
    }

    //If user inputs value below 4, set it to 4
    else if total_allowed_guesses < 4 {
        total_allowed_guesses = 4;
    }
    //Return User input as u32 int
    total_allowed_guesses
}


fn bet_amount (max_guess: u32) -> f32 {
    //User input total amount to bet for game
    println!("Great! How much would you like to bet?");
    let mut bet_amount = String::new();
    io::stdin()
        .read_line(&mut bet_amount)
        .expect("Failed to read line");

    let bet_amount: f32 = bet_amount.trim().parse().unwrap();

    // Display User Bet + Win Multiplier

    if max_guess == 6 {
        println!("You've bet {} SOL with a maximum of {} guesses allowed. You stand to 1.5x your bet.", bet_amount, max_guess);
    }

    else if max_guess == 5 {
        println!("You've bet {} with a maximum of {} guesses allowed. You stand to 2x your bet.", bet_amount, max_guess);
    }

    else {
        println!("You've bet {} with a maximum of {} guesses allowed. You stand to 3x your bet.", bet_amount, max_guess);
    }

    bet_amount

}


fn game_win (bet_amount: f32, max_guesses: u32)  {
    println!("{}", "YOU WIN!".green());

    //Convert Bet Amount to Float32
    let bet_amount_f = bet_amount as f32;
    let mut win_multiplier: f32;
    if max_guesses == 4 {
        win_multiplier = 3.0;
    }
    else if max_guesses == 3 {
        win_multiplier = 2.5;
    }
    else {
        win_multiplier = 1.5;
    }

    println!("You bet {} SOL with maximum allowed guesses of {} , which has a win multiplier of {}x", bet_amount, max_guesses, win_multiplier);
    //Calculate Users total winnings
    let total_winnings: f32 = bet_amount_f * win_multiplier;
    println!("Total Winnings: {} SOL", total_winnings);
}