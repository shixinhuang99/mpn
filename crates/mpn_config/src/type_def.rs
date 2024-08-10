use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Clone)]
pub enum TypeDef {
	String,
	StringV(&'static str),
	Boolean,
	BooleanV(bool),
	Url,
	Number,
	NumberV(i32),
	Path,
	// Stream,
	Date,
	Semver,
	Umask,
	Null,
	Array(&'static [TypeDef]),
}

impl TypeDef {
	pub fn validate(&self) {
		unimplemented!();
	}

	pub fn description(&self) {
		unimplemented!();
	}
}

impl Display for TypeDef {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			TypeDef::String => write!(f, "String"),
			TypeDef::StringV(v) => write!(f, "\"{}\"", v),
			TypeDef::Boolean => write!(f, "Boolean"),
			TypeDef::BooleanV(v) => write!(f, "{}", v),
			TypeDef::Number => write!(f, "Number"),
			TypeDef::NumberV(v) => write!(f, "{}", v),
			TypeDef::Null => write!(f, "null"),
			TypeDef::Umask => {
				write!(f, "Octal numeric string in range 0000..0777 (0..511)")
			}
			TypeDef::Url => write!(f, "URL"),
			TypeDef::Date => write!(f, "Date"),
			TypeDef::Path => write!(f, "Path"),
			TypeDef::Semver => write!(f, "SemVer string"),
			TypeDef::Array(v) => {
				let ret = match v.len() {
					0 => "".to_string(),
					1 => v[0].to_string(),
					2 => format!("{} or {}", v[0], v[1]),
					_ => {
						v.iter().enumerate().fold(String::new(), |s, (i, t)| {
							if i == v.len() - 1 {
								format!("{}or {}", s, t)
							} else {
								format!("{}{}, ", s, t)
							}
						})
					}
				};
				write!(f, "{}", ret)
			}
		}
	}
}
