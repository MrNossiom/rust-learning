use inquire::Text;
use rand::Rng;
use std::{cmp::Ordering, process::exit};

fn main() {
	let mut game_succeed = false;
	let number_to_guess = rand::thread_rng().gen_range(1, 101);

	for _ in 0..10 {
		let supposition = Text::new("Entrez un nombre:").prompt().unwrap();

		let supposition: u32 = match supposition.trim().parse() {
			Ok(number) => number,
			Err(_) => {
				println!("Vous devez entrez un nombre !");
				continue;
			}
		};

		match supposition.cmp(&number_to_guess) {
			Ordering::Less => println!("C'est plus..."),
			Ordering::Greater => println!("C'est moins..."),
			Ordering::Equal => {
				println!("Vous avez gagné !");
				game_succeed = true;
				break;
			}
		}
	}

	match game_succeed {
		true => exit(0),
		false => {
			println!("Tu n'a pas trouvé en moins de 10 essais.");
			exit(1)
		}
	}
}
