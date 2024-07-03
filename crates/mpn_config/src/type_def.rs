use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq)]
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
		}
	}
}
