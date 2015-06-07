use std::borrow::{Cow, IntoCow};
use std::collections::HashMap;
use oauth;
use oauth::Token;

#[doc = "Twitter API URIs"]
mod api {
	pub const REQUEST_TOKEN: &'static str   = "https://api.twitter.com/oauth/request_token";
	pub const AUTHORIZE: &'static str       = "https://api.twitter.com/oauth/authorize";
	pub const ACCESS_TOKEN: &'static str    = "https://api.twitter.com/oauth/access_token";
	pub const STATUSES_UPDATE: &'static str = "https://api.twitter.com/1.1/statuses/update.json";
	pub const FOLLOWER_LIST: &'static str = "https://api.twitter.com/1.1/followers/list.json";
}

#[doc = "Get request variables"]
pub struct ReqVars {
	// global - most requests use these
	pub name: String,					// username (@somebody)
	pub id: String,						// user id
	pub cursor: i32,					// number of pages(cursors) to show (default: -1)
	pub count: i32,						// number of object to show per page(cursor). (default: -1)
	pub skip_status: bool,				// skips user statuses in response (default: true)
	pub include_user_entities: bool,	// (default: true)
}
#[doc = "Default settings for ReqVars"]
impl Default for ReqVars {
	fn default() -> ReqVars {
		ReqVars {
			name: "".to_string(),
			id: "".to_string(),
			cursor: -1,
			count: -1,
			skip_status: true,
			include_user_entities: false
		}
	}
}
#[doc = "Impliment insertion of default options"]
impl ReqVars {
	fn insert_std(&self, mut map: &mut HashMap<Cow<str>, Cow<str>>) {

		if self.cursor != -1 {
			let _ = map.insert(
				"cursor".into_cow(),
				self.cursor.to_string().into_cow()
			);
		}
		if self.count != -1 {
			let _ = map.insert(
				"count".into_cow(),
				self.count.to_string().into_cow()
			);
		}
		if self.skip_status != true {
			let _ = map.insert(
				"skip_status".into_cow(),
				self.skip_status.to_string().into_cow()
			);
		}
		if self.include_user_entities != false {
			let _ = map.insert(
				"include_user_entities".into_cow(),
				self.include_user_entities.to_string().into_cow()
			);
		}
	}
}

#[doc = "
Get list of followers.

# Arguements

* 'consumer'	- Consumer access token.
* 'access'		- Request access token.
* 'vars'		- Filled ReqVars struct.

# Return value

* Result<String, None> - String is json encoded result.

#Failure

Fails and returns None if either username or userid are not provided via vars.
"]
pub fn follower_list(consumer: &Token, access: &Token, vars: &ReqVars) -> Option<String> {
	let mut param = HashMap::new();

	// must have name or id
	if vars.name.len() != 0 {
		let _ = param.insert(
			"screen_name".into_cow(),
			vars.name.to_string().into_cow()
		);
	}
	else if vars.id.len() != 0 {
		let _ = param.insert(
			"user_id".into_cow(),
			vars.id.to_string().into_cow());
	}
	else {
		return None
	}

	let _ = vars.insert_std(&mut param);
	Some(oauth::get(api::FOLLOWER_LIST, consumer, Some(access), Some(&param)))
}

pub fn tweet(consumer: &Token, access: &Token, status: &str) {
	let mut param = HashMap::new();
	let _ = param.insert("status".into_cow(), status.into_cow());
	let _ = oauth::post(api::STATUSES_UPDATE, consumer, Some(access), Some(&param));
}
