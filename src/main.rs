struct Game {
    name: String,
    description: String,
    probability: u8 // For example, a value of 4 would mean a 1/4 chance. Values cannot exceed 256.
}

fn main() {
    let games = [
        Game {
            name: "Coin flip".to_string(),
            description: "Choose a side of the coin. You pass this stage if, after it is flipped, it lands on your chosen side.".to_string(),
            probability: 2
        }, 
        Game {
            name: "Spin the wheel".to_string(),
            description: "Choose one colour from the three options. You pass this stage if the wheel stops on your colour.".to_string(),
            probability: 3
        }, 
        Game {
            name: "Roll a dice".to_string(),
            description: "Choose two numbers on a 6 sided dice. You pass this stage if, after rolling the dice, it lands on either of your chosen numbers.".to_string(),
            probability: 3
        },
        Game {
            name: "Pick a suit".to_string(),
            description: "Guess a suit (Clubs, Diamonds, Hearts, Spades), and choose a random card. This level is passed if your card matches the suit you guessed.".to_string(),
            probability: 4
        },
        Game {
            name: "Pick a cup".to_string(),
            description: "The final challenge! Your prize money is in one of the 3 cups. Choose the right one to win it all.".to_string(),
            probability: 3
        }
    ];

    let mut probability: u16 = 1; // Win chance __cannot__ exceed 65535
    for i in 0..games.len() {
        println!("{}", games[i].name);
        println!("{}", games[i].description);
        println!("(1/{} odds) \n", games[i].probability);
        probability *= games[i].probability as u16;
    }
    assert!(probability > 200, "There must be no more than a 1/200 chance of winning");
}
