mod definition;
mod type_def;

use std::path::PathBuf;

use indexmap::IndexMap;

pub struct Config {
	loaded: bool,
	flat_options: Option<IndexMap<String, String>>,
	env: IndexMap<String, String>,
	pub npm_path: PathBuf,
	deprecated: IndexMap<String, String>,
}

impl Config {
	pub fn new(options: ConfigOptions) -> Self {
		let env = options
			.env
			.unwrap_or_else(|| IndexMap::from_iter(std::env::vars()));

		Self {
			loaded: false,
			flat_options: None,
			env,
			npm_path: options.npm_path,
			deprecated: IndexMap::new(),
		}
	}

	pub fn loaded(&self) -> bool {
		self.loaded
	}
}

pub struct ConfigOptions {
	pub npm_path: PathBuf,
	pub env: Option<IndexMap<String, String>>,
}
