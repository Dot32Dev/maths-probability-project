use colored::*;
use rand::Rng;

const SAMPLE_SIZE: u16 = 200;

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
    let mut name_width: usize = "game".chars().count();
    for i in 0..games.len() {
        println!("{}", games[i].name);
        println!("{}", games[i].description.dimmed());
        println!("(1/{} odds) \n", games[i].probability);

        probability *= games[i].probability as u16;
        name_width = (games[i].name.chars().count()).max(name_width);
    }
    assert!(probability > 200, "There must be no more than a 1/200 chance of winning");

    println!("{}{}{}", "There is a 1/".bright_red(), probability.to_string().bright_red(), " chance of passing every game".bright_red());
    println!("{}{}{}", "There are ".bright_yellow(), SAMPLE_SIZE.to_string().bright_yellow(), " games being simulated".bright_yellow());

    // Simulating the games
    let mut sim = vec![0; games.len()];
    for _i in 0..SAMPLE_SIZE {
        'game: for j in 0..games.len() {
            if rand::thread_rng().gen_range(0, games[j].probability) != 0 {
                sim[j] += 1;
                break 'game
            }
        }
    }

    // println!("{:?}", vec);

    println!("\n{}{}", format!("{:_^1$}", "Game".bold(), name_width).to_string().on_black(), "_|_Winners_|_%_Won_|_%_Expected_|".on_black()); // handy format string code I found online
    let mut winners = SAMPLE_SIZE;
    let mut percent_won: f32;
    for i in 0..games.len() {
        // percent_won = (winners-sim[i])/winners*100;
        winners -= sim[i];
        percent_won = if winners+sim[i] != 0 {
            (100.0/(winners as f32 + sim[i] as f32)*winners as f32).floor()
        }  else {
            0.0
        };
        println!("{} |{} |{} |{} |", 
            format!("{: <1$}", games[i].name, name_width), 
            format!("{: <1$}", winners, 8),
            format!("{: <1$}%", percent_won, 5),
            format!("{: <1$}%", 100/games[i].probability, 10),
        );
    }

    println!("\n{}{}\n", winners.to_string().bright_green(), " people won the game".bright_green());
}
