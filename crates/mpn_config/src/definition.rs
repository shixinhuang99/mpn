use std::sync::OnceLock;

use derive_builder::Builder;
use unindent::unindent;

use crate::type_def::TypeDef;

#[derive(Builder)]
#[builder(pattern = "owned", setter(strip_option), build_fn(skip))]
pub struct Definition {
	pub key: &'static str,
	pub default_value: TypeDef,
	pub types: &'static [TypeDef],
	pub description: &'static str,
	#[builder(setter(skip))]
	pub env_export: bool,
	#[allow(unused)]
	multiple: bool,
	#[builder(setter(into))]
	pub default_description: String,
	#[builder(setter(into))]
	pub type_description: String,
	pub hint: String,
	pub short: Option<&'static [&'static str]>,
	pub usage: String,
	pub deprecated: Option<&'static str>,
	pub exclusive: Option<&'static [&'static str]>,
	terminal_cols: usize,
	// flatten
}

impl DefinitionBuilder {
	pub fn build(self) -> Definition {
		let key = self.key.expect("`key` is required");
		let default_value =
			self.default_value.expect("`default_value` is required");
		let types = self.types.expect("`types` is required");
		let description = self.description.expect("`description` is required");
		let multiple = self.multiple.unwrap_or(false);
		let default_description = self
			.default_description
			.unwrap_or_else(|| default_value.to_string());
		let type_description = self
			.type_description
			.unwrap_or_else(|| describe_type(types, multiple));
		let hint = self.hint.unwrap_or_else(|| hint(types, key));
		let short = self.short.unwrap_or_default();
		let usage = self.usage.unwrap_or_else(|| {
			describe_usage(types, short, &default_value, key, &hint, multiple)
		});
		let deprecated = self.deprecated.unwrap_or_default();
		let exclusive = self.exclusive.unwrap_or_default();
		let terminal_cols = self.terminal_cols.unwrap_or_else(terminal_columns);

		Definition {
			key,
			default_value,
			types,
			description,
			env_export: true,
			multiple,
			default_description,
			type_description,
			hint,
			short,
			usage,
			deprecated,
			exclusive,
			terminal_cols,
		}
	}
}

impl Definition {
	pub fn builder() -> DefinitionBuilder {
		DefinitionBuilder::create_empty()
	}

	pub fn describe(&self) -> String {
		let description = unindent(self.description);
		let no_env_export = if self.env_export {
			""
		} else {
			"\nThis value is not exported to the environment for child processes.\n"
		};
		let deprecated = if let Some(v) = self.deprecated {
			format!("* DEPRECATED: {}\n", unindent(v))
		} else {
			"".to_string()
		};
		let exclusive = if let Some(v) = self.exclusive {
			format!("\nThis config can not be used with: `{}`", v.join("`, `"))
		} else {
			"".to_string()
		};

		wrap_all(
			format!(
				"#### `{}`

* Default: {}
* Type: {}
{}
{}
{}
{}",
				self.key,
				unindent(&self.default_description),
				unindent(&self.type_description),
				deprecated,
				description,
				exclusive,
				no_env_export,
			),
			self.terminal_cols,
		)
	}
}

fn describe_type(types: &[TypeDef], multiple: bool) -> String {
	let len = types.len();
	let words = match len {
		0 => "".to_string(),
		1 => types[0].to_string(),
		2 => format!("{} or {}", types[0], types[1]),
		_ => {
			types.iter().enumerate().fold(String::new(), |s, (i, t)| {
				if i == len - 1 {
					format!("{}or {}", s, t)
				} else {
					format!("{}{}, ", s, t)
				}
			})
		}
	};

	if multiple {
		format!("{} (can be set multiple times)", words)
	} else {
		words
	}
}

fn hint(type_def: &[TypeDef], key: &str) -> String {
	if type_def == [TypeDef::Number] {
		"<number>".to_string()
	} else {
		format!("<{}>", key)
	}
}

fn describe_usage(
	types: &[TypeDef],
	short: Option<&[&str]>,
	default: &TypeDef,
	key: &str,
	hint: &str,
	multiple: bool,
) -> String {
	if types.len() == 1 {
		let mut s = String::new();

		if let Some(short) = short {
			s.push_str(&format!("-{}|", short.join(",")));
		}

		if types == [TypeDef::Boolean] && *default != TypeDef::BooleanV(false) {
			s.push_str(&format!("--no-{}", key));
		} else {
			s.push_str(&format!("--{}", key));
		}

		if types != [TypeDef::Boolean] {
			s.push_str(&format!(" {}", hint));
		}

		return s;
	}

	let mut s = format!("--{}", key);

	if let Some(short) = short {
		s = format!("-{}|--{}", short.join(","), key);
	}

	let filtered_types: Vec<&TypeDef> = types
		.iter()
		.filter(|t| !matches!(t, TypeDef::Null | TypeDef::Boolean))
		.collect();

	if filtered_types.is_empty() {
		return s;
	}

	let spcific_strs: Vec<&str> = filtered_types
		.iter()
		.filter_map(|t| {
			if let TypeDef::StringV(s) = t {
				if !s.is_empty() {
					return Some(*s);
				}
			}
			None
		})
		.collect();

	let desc = if spcific_strs.is_empty() {
		hint.to_string()
	} else {
		format!("<{}>", spcific_strs.join("|"))
	};

	if types.contains(&TypeDef::Boolean) {
		s = format!("--no-{}|{}", key, s);
	}

	let usage = format!("{} {}", s, desc);

	if multiple {
		format!("{} [{} ...]", usage, usage)
	} else {
		usage
	}
}

fn terminal_columns() -> usize {
	static TERMINAL_COLUMNS: OnceLock<usize> = OnceLock::new();

	*TERMINAL_COLUMNS.get_or_init(|| {
		if cfg!(test) {
			return 75;
		}
		if let Some(size) = terminal_size::terminal_size() {
			return (size.0.0.clamp(20, 80) - 5) as usize;
		}
		75
	})
}

fn wrap(s: &str, terminal_cols: usize) -> String {
	let mut ret: Vec<String> = Vec::new();
	let mut words: Vec<&str> = Vec::new();
	let mut len = 0;
	let s = unindent(s);

	for word in s.split_ascii_whitespace() {
		if len + word.len() > terminal_cols {
			ret.push(words.join(" "));
			words.clear();
			len = 0;
		} else {
			words.push(word);
			len += word.len();
		}
	}

	if !words.is_empty() {
		ret.push(words.join(" "));
	}

	ret.join("\n")
}

fn wrap_all(s: String, terminal_cols: usize) -> String {
	let mut in_code_block = false;

	s.split("\n\n")
		.map(|block| {
			if in_code_block || block.starts_with("```") {
				in_code_block = !block.ends_with("```");
				return block.to_string();
			}
			if block.starts_with('*') {
				let tmp = block
					.chars()
					.skip(1)
					.collect::<String>()
					.trim_ascii()
					.split("\n* ")
					.map(|li| wrap(li, terminal_cols).replace("\n", "\n  "))
					.collect::<Vec<String>>()
					.join("\n* ");
				return format!("* {}", tmp);
			}
			wrap(block, terminal_cols)
		})
		.collect::<Vec<String>>()
		.join("\n\n")
}
