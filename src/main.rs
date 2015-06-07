#![warn(bad_style,
    unused, unused_extern_crates, unused_import_braces,
	unused_qualifications, unused_results)]
#![feature(into_cow)]
extern crate twitter_api as twitter;
extern crate oauth_client as oauth;

//use oauth::Token;
mod auth;
mod int_twitter;

fn main() {
	println!("Getting token");
	let (con_tok, acc_tok) = auth::get_token();
	println!("Got token, creating request vars");
	let mut params = int_twitter::ReqVars::default();
	params.name = "c0mmiebstrd".to_string();

	match int_twitter::follower_list(&con_tok, &acc_tok, &params) {
		None => println!("Error in follow_list"),
		Some(rst) => {
			println!("{}", rst)
		},
	}
}
