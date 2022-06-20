struct Game {
    description: String,
    probability: u8 // For example, a value of 4 would mean a 1/4 chance. Values cannot exceed 256.
}

fn main() {
    let games = [
        Game {
            description: "Coin flip".to_string(),
            probability: 2
        }, 
        Game {
            description: "Spin the wheel".to_string(),
            probability: 3
        }, 
        Game {
            description: "Roll a dice".to_string(),
            probability: 3
        },
        Game {
            description: "Pick a suit".to_string(),
            probability: 4
        },
        Game {
            description: "Pick a cup".to_string(),
            probability: 3
        }
    ];

    let mut probability: u16 = 1; // Win chance __cannot__ exceed 65535
    for i in 0..games.len() {
        println!("{}", games[i].description);
        probability *= games[i].probability as u16;
    }
    assert!(probability > 200, "There must be no more than a 1/200 chance of winning");
}
