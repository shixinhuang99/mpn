use insta::assert_snapshot;
use mpn_config::{Definition, TypeDef};

#[test]
#[should_panic]
fn test_missing_fields() {
	let _def = Definition::builder().build();
}

#[test]
fn test_basic_definition() {
	let def = Definition::builder()
		.key("key")
		.default_value(TypeDef::StringV("some default value"))
		.types(&[TypeDef::Number, TypeDef::String])
		.description("just a test thingie")
		.build();
	assert_eq!(def.hint, "<key>");
	assert_eq!(def.usage, "--key <key>");
	assert_eq!(def.type_description, "Number or String");
	assert_eq!(def.default_description, "\"some default value\"");
	assert!(def.env_export);
	assert_snapshot!("basic", def.describe(), "human-readable description");

	let deprecated = Definition::builder()
		.key("deprecated")
		.deprecated("do not use this")
		.default_value(TypeDef::NumberV(1234))
		.description("  it should not be used\n  ever\n\n  not even once.\n\n")
		.types(&[TypeDef::Number])
		.default_description("A number bigger than 1")
		.type_description("An expression of a numeric quantity using numerals")
		.build();
	assert_snapshot!(
		"deprecated",
		deprecated.describe(),
		"description of deprecated thing"
	);

	let exclusive = Definition::builder()
		.key("exclusive")
		.default_value(TypeDef::NumberV(1234))
		.types(&[TypeDef::Number])
		.description("a number")
		.exclusive(&["x"])
		.build();
	assert_snapshot!(
		"exclusive",
		exclusive.describe(),
		"description of exclusive thing"
	);

	let null_or_umark = Definition::builder()
		.key("key")
		.default_value(TypeDef::Null)
		.types(&[TypeDef::Null, TypeDef::Umask])
		.description("asdf")
		.build();
	assert_eq!(
		null_or_umark.type_description,
		"null or Octal numeric string in range 0000..0777 (0..511)"
	);

	let null_date_or_bool = Definition::builder()
		.key("key")
		.default_value(TypeDef::NumberV(7))
		.types(&[TypeDef::Null, TypeDef::Date, TypeDef::Boolean])
		.description("asdf")
		.build();
	assert_eq!(null_date_or_bool.type_description, "null, Date, or Boolean");

	let many_paths = Definition::builder()
		.key("key")
		.default_value(TypeDef::StringV("asdf"))
		.types(&[TypeDef::Path])
		.multiple(true)
		.description("asdf")
		.build();
	assert_eq!(
		many_paths.type_description,
		"Path (can be set multiple times)"
	);

	let path_or_url = Definition::builder()
		.key("key")
		.default_value(TypeDef::StringV("https://example.com"))
		.types(&[TypeDef::Path, TypeDef::Url])
		.description("asdf")
		.build();
	assert_eq!(path_or_url.type_description, "Path or URL");
}
