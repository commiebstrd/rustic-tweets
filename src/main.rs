#![warn(bad_style,
    unused, unused_extern_crates, unused_import_braces,
	unused_qualifications, unused_results)]
extern crate twitter_api as twitter;
extern crate oauth_client as oauth;

use oauth::Token;
mod auth;

fn main() {
	let (con_tok, acc_tok) = auth::get_token();
}
