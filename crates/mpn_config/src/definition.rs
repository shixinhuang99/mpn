use std::sync::OnceLock;

use unindent::unindent;

use crate::type_def::TypeDef;

pub struct DefinitionOptions {
	key: &'static str,
	default: TypeDef,
	types: &'static [TypeDef],
	description: &'static str,
	multiple: bool,
	short: Option<&'static [&'static str]>,
	deprecated: Option<&'static str>,
	exclusive: Option<&'static [&'static str]>,
}

pub struct Definition {
	key: &'static str,
	default: TypeDef,
	types: &'static [TypeDef],
	description: &'static str,
	env_export: bool,
	multiple: bool,
	default_description: String,
	type_description: String,
	hint: String,
	short: Option<&'static [&'static str]>,
	usage: String,
	deprecated: Option<&'static str>,
	exclusive: Option<&'static [&'static str]>,
	// flatten
}

impl Definition {
	pub fn new(options: DefinitionOptions) -> Self {
		let default_description = options.default.to_string();
		let type_description = describe_type(options.types, options.multiple);
		let hint = hint(options.types, options.key);
		let usage = describe_usage(
			options.types,
			options.short,
			&options.default,
			options.key,
			&hint,
			options.multiple,
		);

		Self {
			key: options.key,
			default: options.default,
			types: options.types,
			description: options.description,
			multiple: options.multiple,
			env_export: true,
			short: None,
			default_description,
			type_description,
			hint,
			usage,
			deprecated: options.deprecated,
			exclusive: options.exclusive,
		}
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

		let tmp = wrap_all(&format!(
			"
			#### `{}`

			* Default: {}
			* Type: {}
			{}
			{}
			{}
			{}
			",
			self.key,
			unindent(&self.default_description),
			unindent(&self.type_description),
			deprecated,
			description,
			exclusive,
			no_env_export,
		));

		unindent(&tmp)
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
		if let Some(size) = terminal_size::terminal_size() {
			return size.0.0 as usize;
		}
		80
	})
}

fn wrap(s: &str) -> String {
	let cols = terminal_columns().clamp(20, 80) - 5;
	let mut ret: Vec<String> = Vec::new();
	let mut words: Vec<&str> = Vec::new();
	let mut len = 0;
	let s = unindent(s);

	for word in s.split_ascii_whitespace() {
		if len + word.len() > cols {
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

fn wrap_all(s: &str) -> String {
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
					.map(|li| wrap(li).replace("\n", "\n  "))
					.collect::<Vec<String>>()
					.join("\n* ");
				return format!("* {}", tmp);
			}
			wrap(block)
		})
		.collect::<Vec<String>>()
		.join("\n\n")
}

#[cfg(test)]
mod tests {
	use super::{Definition, DefinitionOptions, TypeDef};

	#[test]
	fn basic_definition() {
		let def = Definition::new(DefinitionOptions {
			key: "key",
			default: TypeDef::StringV("some default value"),
			types: &[TypeDef::Number, TypeDef::String],
			description: "just a test thingie",
			multiple: false,
			short: None,
			deprecated: None,
			exclusive: None,
		});
		assert_eq!(def.hint, "<key>");
		assert_eq!(def.usage, "--key <key>");
		assert_eq!(def.type_description, "Number or String");
		assert!(def.env_export);
	}
}
