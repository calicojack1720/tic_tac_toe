/*
  tic_tac_toe
  A friendly game of tic_tac_toe between a user and an automated opponent.
  Created by calicojack1720
  Created: 5/21/2024
  Updated: 5/29/2024
*/

use std::io;
use tic_tac_toe::Board; //brining in the board struct
use rand::Rng;

fn main() {
	const PLAYER: char = 'X'; //symbol for the player
	const COMPUTER: char = 'O'; //symbol for the computer
	
	//create and intialize the game board
	let mut game_board = Board {marks: [[' '; 3]; 3]}; //the tic-tac-toe board
	let mut game_over: u8 = 0; //tracks whether the game is over (0 - still going, 1 - player wins, 2 - computer wins)
	let mut player_x: u16 = 0; //x coordinate for player moves
	let mut player_y: u16 = 0; //y coordinate for player moves
	let mut user_move = String::new();

	//print out the game board
	println!("{}", game_board);
	
	while game_over == 0 {
		
		//getting the player's move
		//TODO -> continue after non-numerical input
		println!("Enter x: ");

		io::stdin()
		    .read_line(&mut user_move)
		    .expect("Failed to read line");

		let trimmed = user_move.trim();
		match trimmed.parse::<u16>() {
		    	Ok(i) => player_x = i,
		    	Err(..) => println!("Err: Non-numerical input entered"),
		};

		user_move = String::new();
		
		println!("Enter y: ");

		io::stdin()
			.read_line(&mut user_move)
			.expect("Failed to read line");

		let trimmed = user_move.trim();
		match trimmed.parse::<u16>() {
			Ok(i) => player_y = i,
			Err(..) => println!("Err: Non-numerical input entered"),
		};

		user_move = String::new();

		//make the player's move
		let move_success = make_move(&mut game_board, player_x, player_y, PLAYER);

		//check to see if it was a valid move, if not, restart the loop
		if move_success == false {
			println!("Sorry but {}, {} is already filled, try again.", player_x, player_y);
			continue;
		}

		//check for a win
		game_over = check_win(&mut game_board, &PLAYER, &COMPUTER);
		if game_over != 0 {
			continue;
		}

		//print out the game board
		println!("{}", game_board);

		//have the computer make a random move
		computer_move(&mut game_board, COMPUTER);

		//print out the game board
		println!("{}", game_board);
	}

	//print the final results
	println!("{}", game_board);
	if game_over == 1 {
		println!("Congratulations! You won!!!");
	} else if game_over == 2 {
		println!("Too bad...you lost :(");
	} else {
		println!("It's a tie.");
	}
}

//Precondition: takes a mutable reference to the board, a u16 integer for the x axis, a u16 integer for the y axis, and the character being used.
//Postcondition: makes the move if the character at the location is a space, returning true for a successful move and false for an unsuccessful one.
fn make_move(b: &mut Board, x: u16, y: u16, symbol: char) -> bool {
	if b.marks[(y-1) as usize][(x-1) as usize] == ' ' {
		b.marks[(y-1) as usize][(x-1) as usize] = symbol;
		true
	}
	else {
		false
	}
}

//Precondition: takes a mutable reference to the game board as well as the character for the computer's marks.
//Postcondition: makes a random move on the board against the player.
fn computer_move(b: &mut Board, symbol: char) {
	let mut valid_move = false; //tracks whether the computer has made a valid move

	let mut tmp = b;
	
	while valid_move == false {
		let move_x = rand::thread_rng().gen_range(1..=3); //get a random x coordinate in the range 1 to 3
		let move_y = rand::thread_rng().gen_range(1..=3); //get a random y coordinate in the range 1 to 3
		
		valid_move = make_move(&mut tmp, move_x, move_y, symbol);
	}
}

//Precondition: takes a mutable reference to the board, a reference to the player character, and a reference to the computer character.
//Postcondition: returns an unsigned 8-bit integer (0 for not win yet, 1 for player win, 2 for computer win, and 3 for a tie)
fn check_win(b: &mut Board, p: &char, c: &char) -> u8 {

	let mut result = 0;

	//checking all the win conditions
	if b.marks[0][0] == b.marks[0][1] && b.marks[0][0] == b.marks[0][2] {
		if b.marks[0][0] == *p {
			result = 1;
		} else if b.marks[0][0] == *c {
			result = 2
		}
	} else if b.marks[1][0] == b.marks[1][1] && b.marks[1][0] == b.marks[1][2] {
		if b.marks[1][0] == *p {
			result = 1;
		} else if b.marks[1][0] == *c {
			result = 2;
		}
	} else if b.marks[2][0] == b.marks[2][1] && b.marks[2][0] == b.marks[2][2] {
		if b.marks[2][0] == *p {
			result = 1;
		} else if b.marks[2][0] == *c {
			result = 2;
		}
	} else if b.marks[0][0] == b.marks[1][0] && b.marks[0][0] == b.marks[2][0] {
		if b.marks[0][0] == *p {
			result = 1;
		} else if b.marks[0][0] == *c {
			result = 2;
		}
	} else if b.marks[0][1] == b.marks[1][1] && b.marks[0][1] == b.marks[2][1] {
		if b.marks[0][1] == *p {
			result = 1;
		} else if b.marks[0][1] == *c {
			result = 2;
		}
	} else if b.marks[0][2] == b.marks[1][2] && b.marks[0][2] == b.marks[2][2] {
		if b.marks[0][2] == *p {
			result = 1;
		} else if b.marks[0][2] == *c {
			result = 2;
		}
	} else if b.marks[0][0] == b.marks[1][1] && b.marks[0][0] == b.marks[2][2] {
		if b.marks[0][0] == *p {
			result = 1;
		} else if b.marks[0][0] == *c {
			result = 2;
		}
	} else if b.marks[0][2] == b.marks[1][1] && b.marks[0][2] == b.marks[2][0] {
		if b.marks[0][2] == *p {
			result = 1;
		} else if b.marks[0][2] == *c {
			result = 2;
		}
	} else {
		//checking for tie, else we return 0
		let mut tie = true;
		for i in 0..3 {
			for j in 0..3 {
				if b.marks[i][j] == ' ' {
					tie = false;
				}
			}
		}

		if tie == false {
			result = 0;
		} else {
			result = 3;
		}
	}

	result as u8
}
