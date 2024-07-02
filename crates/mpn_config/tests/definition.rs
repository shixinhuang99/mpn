use insta::assert_snapshot;
use mpn_config::{Definition, TypeDef};

#[test]
#[should_panic]
fn test_missing_fields() {
	let _def = Definition::builder().build();
}

#[test]
fn basic_definition() {
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
		"deprecated_1",
		deprecated.describe(),
		"description of deprecated thing 1"
	);
}
