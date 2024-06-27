use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq)]
pub enum TypeDef {
	String,
	StringV(&'static str),
	Boolean,
	BooleanV(bool),
	// Url,
	Number,
	NumberV(i32),
	// Path,
	// Stream,
	// Date,
	// Array,
	// Semver,
	// Umask,
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
		}
	}
}
