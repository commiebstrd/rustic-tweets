#![warn(bad_style,
    unused, unused_extern_crates, unused_import_braces,
	unused_qualifications, unused_results)]
extern crate tweetust;

mod auth;

fn main() {
	let oauth = auth::get_token();
	println!("{:?}", oath);
}
