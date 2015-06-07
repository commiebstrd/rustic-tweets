use std::io;
use std::io::prelude::*;
use oauth::Token;
use twitter;

#[derive(Debug)]
pub struct Keys {
	pub consumer_key: String,
	pub consumer_secret: String,
	pub access_key: String,
	pub access_secret: String
}


pub fn get_token<'a>() -> (Token<'a>, Token<'a>) {

	let mut key_s: Keys = Keys {
		consumer_key: " lVvrRhV0v1idwqhIyzQ695lsf".to_string(),
		consumer_secret: "KAo3YtKXJxVYRzAVZOVdMfjn6rnVW6GQDhLaj1WU9OvouuyBQkYXprmpsCvIhJEWE90PLfXal91eeRN0Nr5WGhf8CrtXQha".to_string(),
		access_key: "288484228-MXvOlFczyHTSUTdd0OjjkbT8LUErYC7Jnr1F4zbr".to_string(),
		access_secret: "Laj1WU9OvouuyBQkYXprmpsCvIhJEWE90PLfXal91eeRN".to_string()
	};

	if key_s.consumer_key.len() == 0 || key_s.consumer_secret.len() == 0 {
		key_s.consumer_key = console_input("Input your consumer key:");
		key_s.consumer_secret = console_input("Input your consumer secret:");
	}
	let consumer_token = Token::new(key_s.consumer_key, key_s.consumer_secret);

//	if key_s.access_key.len() == 0 || key_s.access_secret.len() == 0 {
		println!("created keys and consumer_token");
		let request = twitter::get_request_token(&consumer_token);
		println!("Open the following url:");
		println!("\t{}", twitter::get_authorize_url(&request));
		let pin = console_input("Input pin:");
		let access = twitter::get_access_token(&consumer_token, &request, &pin);
		key_s.access_key = access.key.to_string();
		key_s.access_secret = access.secret.to_string();
//	}
	let access_token = Token::new(key_s.access_key, key_s.access_secret);

	(consumer_token, access_token)

}


fn console_input(prompt: &str) -> String {
    print!("{}\n\t", prompt);
	let mut line = String::new();
	let _ = io::stdin().read_line(&mut line).unwrap();
	line.trim().to_string()
}

