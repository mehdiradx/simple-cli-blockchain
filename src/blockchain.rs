use serde_derive::Serialize;
use sha2::{Digest, Sha256};
use chrono::prelude::*;

// Represents a transaction between a sender and receiver with a certain amount
#[derive(Debug, Clone, Serialize)]
struct Transaction {
	sender: String,
	receiver: String,
	amount: f32,
}

// Represents the header of a block containing metadata
#[derive(Serialize, Debug)]
pub struct Blockheader {
	timestamp: i64,     // Timestamp of block creation
	nonce: u32,         // Nonce used for proof of work
	pre_hash: String,   // Hash of the previous block
	merkle: String,     // Merkle root of transactions
	difficulty: u32,    // Difficulty target for mining
}

// Represents a block containing a header and a list of transactions
#[derive(Serialize, Debug)]
pub struct Block {
	header: Blockheader,
	count: u32,                 // Number of transactions in the block
	transactions: Vec<Transaction>,  // List of transactions
}

// Represents the blockchain itself
pub struct Chain {
	chain: Vec<Block>,          // List of blocks in the blockchain
	curr_trans: Vec<Transaction>, // List of current transactions to be added to the next block
	difficulty: u32,            // Current difficulty of the proof of work algorithm
	miner_addr: String,         // Address of the miner receiving rewards
	reward: f32,                // Mining reward
}

impl Chain {
	// Creates a new blockchain with a genesis block
	pub fn new(miner_addr: String, difficulty: u32) -> Chain {
		let mut chain = Chain {
			chain: Vec::new(),
			curr_trans: Vec::new(),
			difficulty,
			miner_addr,
			reward: 100.0, // Initial mining reward
		};
		chain.generate_new_block(); // Generate the genesis block
		chain
	}

	// Adds a new transaction to the current transactions list
	pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
		self.curr_trans.push(Transaction { sender, receiver, amount });
		true
	}

	// Gets the hash of the last block in the chain, or a default value if the chain is empty
	pub fn last_hash(&self) -> String {
		match self.chain.last() {
			Some(block) => Chain::hash(&block.header),
			None => "0".repeat(64), // Return a string of 64 zeros if the chain is empty
		}
	}

	// Updates the mining difficulty
	pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
		self.difficulty = difficulty;
		true
	}

	// Updates the mining reward
	pub fn update_reward(&mut self, reward: f32) -> bool {
		self.reward = reward;
		true
	}

	// Generates a new block and adds it to the chain
	pub fn generate_new_block(&mut self) -> bool {
		// Create a new block header
		let header = Blockheader {
			timestamp: Utc::now().timestamp_millis(),
			nonce: 0,
			pre_hash: self.last_hash(),
			merkle: String::new(),
			difficulty: self.difficulty,
		};

		// Create the mining reward transaction
		let reward_trans = Transaction {
			sender: "Root".into(),
			receiver: self.miner_addr.clone(),
			amount: self.reward,
		};

		// Initialize the new block with the header and transactions
		let mut block = Block {
			header,
			count: 0,
			transactions: vec![],
		};

		// Add the reward transaction and current transactions to the block
		block.transactions.push(reward_trans);
		block.transactions.append(&mut self.curr_trans);
		block.count = block.transactions.len() as u32;

		// Calculate the Merkle root of the transactions
		block.header.merkle = Chain::get_merkle(&block.transactions);

		// Perform the proof of work to mine the block
		Chain::proof_of_work(&mut block.header);

		println!("{:#?}", &block); // Print the block for debugging
		self.chain.push(block); // Add the block to the chain
		true
	}

	// Calculates the Merkle root of a list of transactions
	fn get_merkle(transactions: &[Transaction]) -> String {
		// Hash each transaction
		let mut merkle: Vec<String> = transactions.iter().map(Chain::hash).collect();

		// If the number of hashes is odd, duplicate the last hash
		if merkle.len() % 2 == 1 {
			if let Some(last) = merkle.last().cloned() {
				merkle.push(last);
			}
		}

		// Combine pairs of hashes until only one hash remains
		while merkle.len() > 1 {
			let mut new_merkle = vec![];

			for pair in merkle.chunks(2) {
				let mut concatenated = pair[0].clone();
				if let Some(second) = pair.get(1) {
					concatenated.push_str(second);
				}
				new_merkle.push(Chain::hash(&concatenated));
			}

			merkle = new_merkle;
		}

		merkle.pop().unwrap_or_default() // Return the final hash
	}

	// Performs the proof of work algorithm to mine a block
	pub fn proof_of_work(header: &mut Blockheader) {
		while {
			// Calculate the hash of the block header
			let hash = Chain::hash(header);
			let prefix = &hash[..header.difficulty as usize];

			// Check if the hash satisfies the difficulty requirement
			if prefix.chars().all(|ch| ch == '0') {
				println!("Block hash: {}", hash); // Print the valid hash
				false // Exit the loop
			} else {
				header.nonce += 1; // Increment the nonce and try again
				true // Continue the loop
			}
		} {}
	}

	// Calculates the SHA-256 hash of a serializable item
	pub fn hash<T: serde::Serialize>(item: &T) -> String {
		let input = serde_json::to_string(item).unwrap(); // Serialize the item to JSON
		let mut hasher = Sha256::new();
		hasher.update(input.as_bytes());
		let res = hasher.finalize();
		Chain::hex_to_string(&res) // Convert the hash to a hexadecimal string
	}

	// Converts a byte array to a hexadecimal string
	fn hex_to_string(bytes: &[u8]) -> String {
		bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
	}
}
