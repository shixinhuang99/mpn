use derive_builder::Builder;
use unindent::unindent;

use crate::{type_def::TypeDef, util::TERMINAL_COLUMNS};

#[derive(Builder)]
#[builder(
	pattern = "owned",
	setter(strip_option),
	build_fn(skip),
	derive(Clone)
)]
pub struct Definition {
	pub key: &'static str,
	pub default_value: TypeDef,
	pub type_def: TypeDef,
	pub description: &'static str,
	pub env_export: bool,
	#[allow(unused)]
	multiple: bool,
	#[builder(setter(into))]
	pub default_description: String,
	#[builder(setter(into))]
	pub type_description: String,
	#[builder(setter(into))]
	pub hint: String,
	pub short: Option<&'static [&'static str]>,
	#[builder(setter(into))]
	pub usage: String,
	pub deprecated: Option<&'static str>,
	pub exclusive: Option<&'static [&'static str]>,
	terminal_cols: usize,
}

impl DefinitionBuilder {
	pub fn build(self) -> Definition {
		let key = self.key.expect("`key` is required");
		let default_value =
			self.default_value.expect("`default_value` is required");
		let type_def = self.type_def.expect("`type_def` is required");
		let description = self.description.expect("`description` is required");
		let multiple = self.multiple.unwrap_or(false);
		let default_description = self
			.default_description
			.unwrap_or_else(|| default_value.to_string());
		let type_description = self
			.type_description
			.unwrap_or_else(|| describe_type(&type_def, multiple));
		let hint = self.hint.unwrap_or_else(|| hint(&type_def, key));
		let short = self.short.unwrap_or_default();
		let usage = self.usage.unwrap_or_else(|| {
			describe_usage(
				&type_def,
				short,
				&default_value,
				key,
				&hint,
				multiple,
			)
		});
		let deprecated = self.deprecated.unwrap_or_default();
		let exclusive = self.exclusive.unwrap_or_default();
		let terminal_cols =
			self.terminal_cols.unwrap_or_else(|| *TERMINAL_COLUMNS);
		let env_export = self.env_export.unwrap_or(true);

		Definition {
			key,
			default_value,
			type_def,
			description,
			env_export,
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

fn describe_type(type_def: &TypeDef, multiple: bool) -> String {
	if multiple {
		format!("{} (can be set multiple times)", type_def)
	} else {
		type_def.to_string()
	}
}

fn hint(type_def: &TypeDef, key: &str) -> String {
	if *type_def == TypeDef::Number {
		"<number>".to_string()
	} else {
		format!("<{}>", key)
	}
}

fn describe_usage(
	type_def: &TypeDef,
	short: Option<&[&str]>,
	default: &TypeDef,
	key: &str,
	hint: &str,
	multiple: bool,
) -> String {
	if let TypeDef::Array(types) = type_def {
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
	} else {
		let mut s = String::new();

		if let Some(short) = short {
			s.push_str(&format!("-{}|", short.join(",")));
		}

		if *type_def == TypeDef::Boolean && *default != TypeDef::BooleanV(false)
		{
			s.push_str(&format!("--no-{}", key));
		} else {
			s.push_str(&format!("--{}", key));
		}

		if *type_def != TypeDef::Boolean {
			s.push_str(&format!(" {}", hint));
		}

		s
	}
}

fn wrap(s: &str, cols: usize) -> String {
	unindent(s).split_ascii_whitespace().fold(
		String::new(),
		|mut left, right| {
			if left.is_empty() {
				return right.to_string();
			}
			let last = left.split("\n").last();
			let join = if last.is_some_and(|l| l.len() + right.len() > cols) {
				"\n"
			} else {
				" "
			};
			left.push_str(join);
			left.push_str(right);
			left
		},
	)
}

fn wrap_all(s: String, terminal_cols: usize) -> String {
	let mut in_code_block = false;
	let cols = terminal_cols.clamp(20, 80) - 5;

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
					.map(|li| wrap(li, cols).replace("\n", "\n  "))
					.collect::<Vec<String>>()
					.join("\n* ");
				return format!("* {}", tmp);
			}
			wrap(block, cols)
		})
		.collect::<Vec<String>>()
		.join("\n\n")
}
