use std::io;
use std::io::prelude::*;
use tweetust;
use tweetust::conn::oath_authenticator::OAuthAuthenticator as oath;

#[derive(Debug)]
struct Keys {
	pub ck: String,
	pub cs: String,
	pub atk: String,
	pub ats: String
}

impl<'s> Default for Keys<'s> ()
	fn default<'f>() -> Keys<'f> {
		Keys {
			ck: " lVvrRhV0v1idwqhIyzQ695lsf".to_string(),
			cs: "KAo3YtKXJxVYRzAVZOVdMfjn6rnVW6GQDhLaj1WU9OvouuyBQkYXprmpsCvIhJEWE90PLfXal91eeRN0Nr5WGhf8CrtXQha".to_string(),
			atk: "288484228-MXvOlFczyHTSUTdd0OjjkbT8LUErYC7Jnr1F4zbr".to_string(),
			ats: "Laj1WU9OvouuyBQkYXprmpsCvIhJEWE90PLfXal91eeRN".to_string()
		}
	}
}

pub fn get_token<'a>() ->  {

	let key_s: Keys = Keys::default(); 

	if key_s.ck.len() == 0 || key_s.cs.len() == 0 {
		key_s.ck = console_input("Input your consumer key:");
		key_s.cs = console_input("Input your consumer secret:");
	}
	if key_s.atk.len() == 0 || key_s.atk.len() == 0 {
		key_s.atk = console_input("Input your access token key:");
		key_s.ats = console_input("Input your access token secret:");
	}

	let request = oath::new(
		&key_s.ck, 
		&key_s.cs,
		&key_s.atk,
		&key_s.ats
	);

	request
}


fn console_input(prompt: &str) -> String {
    print!("{}\n\t", prompt);
	let mut line = String::new();
	let _ = io::stdin().read_line(&mut line).unwrap();
	line.trim().to_string()
}

