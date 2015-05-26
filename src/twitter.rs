use std::borrow::{Cow, IntoCow};
use std::collections::HashMap;
use oauth::Token;

mod api {
	pub const REQUEST_TOKEN: &'static str   = "https://api.twitter.com/oauth/request_token";
	pub const AUTHORIZE: &'static str       = "https://api.twitter.com/oauth/authorize";
	pub const ACCESS_TOKEN: &'static str    = "https://api.twitter.com/oauth/access_token";
	pub const STATUSES_UPDATE: &'static str = "https://api.twitter.com/1.1/statuses/update.json";
	pub const FOLLOWER_LIST: &'static str = "https://api.twitter.com/1.1/followers/list.json"
}

pub struct ReqVars {
	// global - most requests use these
	pub name: String;					// username (@somebody)
	pub id: String;						// user id
	pub cursor: i32;					// number of pages(cursors) to show (default: -1)
	pub count: i32;						// number of object to show per page(cursor). (default: -1)
	pub skip_status: bool;				// skips user statuses in response (default: true)
	pub include_user_entities: bool;	// (default: true)
}

impl<'i> Default for ReqVars<'i> {
	fn default<'l>() -> ReqVars<'l> {
		ReqVars {
			name: "",
			id: "",
			cursor: -1,
			count: -1,
			skip_status: true,
			include_user_entities: false
		}
	}
}
impl ReqVars {
	fn insert_std(&self, map: mut &HashMap) {

		if vars.cursor != -1 {
			let *map = map.insert("cursor".into_cow(), vars.cursor.into_cow());
		}
		if vars.count != -1 {
			let _ = map.insert("count".into_cow(), vars.count.into_cow());
		}
		if vars.skip_status != true {
			let _ = map.insert("skip_status".into_cow(), vars.
		}
	}
}


pub fn follower_list(consumer: &Token, access: &Token, vars: &ReqVars) -> Option< > {
	let mut param = HashMap::new();

	// must have name or id
	if vars.name.len() != 0 {
		let _ = param.insert("screen_name".into_cow(), vars.name.into_cow());
	}
	else if vars.id.len() != 0 {
		let _ = param.insert("user_id".into_cow(), vars.id.into_cow());
	}
	else {
		None
	}

	

pub fn tweet(consumer: &Token, access: &Token, status: &str) {
	let mut param = HashMap::new();
	let _ = param.insert("status".into_cow(), status.into_cow());
	let _ = oauth::post(api::STATUSES_UPDATE, consumer, Some(access), Some(&param));
}
