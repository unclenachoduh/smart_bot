use std::io;

fn main() {
    
    let mut question = false;

    loop {
    	if question == false {
	    	println!("So, it has come to this...");
	    } else {
	    	println!("Hmm... Let me think about that.");
	    };

    	let mut gibberish = String::new();

    	io::stdin().read_line(&mut gibberish)
			.expect("Failed to read line");

		let gibberish = gibberish.trim();

		let last: Vec<char> = gibberish.chars().rev().take(1).collect();

		let last_char: String = last.into_iter().collect();

		// println!("{}", last_char);
		if last_char == "?" {
			question = true;
		} else {
			question = false
		};

		let silence = vec!["shut up", "Shut up", "shut up.", "Shut up.", "shut up!", "Shut up!"];

		//println!("{}", silence);

		let mut end_conv = false;

		for e in silence {
			if gibberish == e {
				end_conv = true;
			}
		}

		if end_conv == true {
			break;
		}

    }
}
