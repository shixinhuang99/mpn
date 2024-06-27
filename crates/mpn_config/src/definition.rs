use crate::type_def::TypeDef;

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
	// deprecated: &'static str,
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
			env_export: false,
			short: None,
			default_description,
			type_description,
			hint,
			usage,
		}
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

pub struct DefinitionOptions {
	key: &'static str,
	default: TypeDef,
	types: &'static [TypeDef],
	description: &'static str,
	multiple: bool,
	short: Option<&'static [&'static str]>,
}
