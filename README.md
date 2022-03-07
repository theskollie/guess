
# Gamble Guess Game - Solana

Gamble Guess Game is my take on RNG number guessing.  
Written in Rust, intended for Solana smart contract.  
A random number is generated between 1-100 and users choose max guess limit.

#### Guess Multiplier Winnings
4 Guesses = 3x Bet Amount  
5 Guesses = 2x Bet Amount  
6 Guesses = 1.5x Bet Amount 

Users are told whether their guess is too low, or high after each guess.  
If user correctly guesses secret number within max allowed tries, they win their bet amount multiplied by the win multiplier (Calculated based on user selected max guess allowance)

This is just a demo and all outputs are printed to terminal.




## Demo

![DemoFile](https://github.com/theskollie/guess/blob/main/GuessDemo.gif?raw=true)


## Acknowledgements

 - [Rust Lang Book](https://doc.rust-lang.org/stable/book/)
 
## Roadmap

- V1 Base Guessing Game Written in Rust

- Solana Devnet Implementation with Static Secret Number

- Implement Chainlink for Random Number Generation (RNG)

- React UI

- Solana Main Net


## Support

Discord: Skollie#9829

