use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    // Read miner address from user input
    let mut miner_addr = String::new();
    print!("Input a miner address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut miner_addr).expect("Failed to read line");

    // Read difficulty from user input
    let mut difficulty = String::new();
    print!("Difficulty: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut difficulty).expect("Failed to read line");
    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("Difficulty should be an integer");

    println!("Generating genesis block!");
    // Create a new blockchain with the specified miner address and difficulty
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    // Main loop for the menu
    loop {
        // Display the menu options
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine Block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        
        // Read the user's menu choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        // Match the user's choice and execute the corresponding action
        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting!");
                process::exit(0);
            }
            1 => {
                // Handle new transaction creation
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter sender address: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut sender).expect("Failed to read line");
                print!("Enter receiver address: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut receiver).expect("Failed to read line");
                print!("Enter amount: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut amount).expect("Failed to read line");

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                }
            }
            2 => {
                // Handle block mining
                println!("Generating block...");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            }
            3 => {
                // Handle difficulty update
                let mut new_diff = String::new();
                print!("Enter new difficulty: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_diff).expect("Failed to read line");
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated difficulty"),
                    false => println!("Failed to update difficulty"),
                }
            }
            4 => {
                // Handle reward update
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_reward).expect("Failed to read line");
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed to update reward"),
                }
            }
            _ => println!("Invalid option, please retry"),
        }
    }
}
