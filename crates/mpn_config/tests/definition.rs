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
		.type_def(TypeDef::Array(&[TypeDef::Number, TypeDef::String]))
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
		.type_def(TypeDef::Number)
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
		.type_def(TypeDef::Number)
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
		.type_def(TypeDef::Array(&[TypeDef::Null, TypeDef::Umask]))
		.description("asdf")
		.build();
	assert_eq!(
		null_or_umark.type_description,
		"null or Octal numeric string in range 0000..0777 (0..511)"
	);

	let null_date_or_bool = Definition::builder()
		.key("key")
		.default_value(TypeDef::NumberV(7))
		.type_def(TypeDef::Array(&[
			TypeDef::Null,
			TypeDef::Date,
			TypeDef::Boolean,
		]))
		.description("asdf")
		.build();
	assert_eq!(null_date_or_bool.type_description, "null, Date, or Boolean");

	let many_paths = Definition::builder()
		.key("key")
		.default_value(TypeDef::Array(&[TypeDef::StringV("asdf")]))
		.type_def(TypeDef::Array(&[TypeDef::Path]))
		.multiple(true)
		.description("asdf")
		.build();
	assert_eq!(
		many_paths.type_description,
		"Path (can be set multiple times)"
	);

	let path_or_url = Definition::builder()
		.key("key")
		.default_value(TypeDef::Array(&[TypeDef::StringV(
			"https://example.com",
		)]))
		.type_def(TypeDef::Array(&[TypeDef::Path, TypeDef::Url]))
		.description("asdf")
		.build();
	assert_eq!(path_or_url.type_description, "Path or URL");

	let multi_12 = Definition::builder()
		.key("key")
		.default_value(TypeDef::Array(&[]))
		.type_def(TypeDef::Array(&[TypeDef::NumberV(1), TypeDef::NumberV(2)]))
		.description("asdf")
		.multiple(true)
		.build();
	assert_eq!(
		multi_12.type_description,
		"1 or 2 (can be set multiple times)"
	);

	let multi_123 = Definition::builder()
		.key("key")
		.default_value(TypeDef::Array(&[]))
		.type_def(TypeDef::Array(&[
			TypeDef::NumberV(1),
			TypeDef::NumberV(2),
			TypeDef::NumberV(3),
		]))
		.description("asdf")
		.multiple(true)
		.build();
	assert_eq!(
		multi_123.type_description,
		"1, 2, or 3 (can be set multiple times)"
	);

	let multi_123_semver = Definition::builder()
		.key("key")
		.default_value(TypeDef::Array(&[]))
		.type_def(TypeDef::Array(&[
			TypeDef::NumberV(1),
			TypeDef::NumberV(2),
			TypeDef::NumberV(3),
			TypeDef::Semver,
		]))
		.description("asdf")
		.multiple(true)
		.build();
	assert_eq!(
		multi_123_semver.type_description,
		"1, 2, 3, or SemVer string (can be set multiple times)"
	);

	let has_usage = Definition::builder()
		.key("key")
		.default_value(TypeDef::StringV("test default"))
		.type_def(TypeDef::String)
		.description("test description")
		.usage("test usage")
		.build();
	assert_eq!(has_usage.usage, "test usage");

	let has_short = Definition::builder()
		.key("key")
		.default_value(TypeDef::StringV("test default"))
		.short(&["t"])
		.type_def(TypeDef::String)
		.description("test description")
		.build();
	assert_eq!(has_short.usage, "-t|--key <key>");

	let multi_has_short = Definition::builder()
		.key("key")
		.default_value(TypeDef::StringV("test default"))
		.short(&["t"])
		.type_def(TypeDef::Array(&[TypeDef::Null, TypeDef::String]))
		.description("test description")
		.build();
	assert_eq!(multi_has_short.usage, "-t|--key <key>");

	let hard_coded_types = Definition::builder()
		.key("key")
		.default_value(TypeDef::StringV("test default"))
		.type_def(TypeDef::Array(&[
			TypeDef::StringV("string1"),
			TypeDef::StringV("string2"),
		]))
		.description("test description")
		.build();
	assert_eq!(hard_coded_types.usage, "--key <string1|string2>");

	let hard_coded_optional_types = Definition::builder()
		.key("key")
		.default_value(TypeDef::StringV("test default"))
		.type_def(TypeDef::Array(&[
			TypeDef::Null,
			TypeDef::StringV("string1"),
			TypeDef::StringV("string2"),
		]))
		.description("test description")
		.build();
	assert_eq!(hard_coded_optional_types.usage, "--key <string1|string2>");

	let has_hint = Definition::builder()
		.key("key")
		.default_value(TypeDef::StringV("test default"))
		.type_def(TypeDef::String)
		.description("test description")
		.hint("<testparam>")
		.build();
	assert_eq!(has_hint.usage, "--key <testparam>");

	let optional_bool = Definition::builder()
		.key("key")
		.default_value(TypeDef::Null)
		.type_def(TypeDef::Array(&[TypeDef::Null, TypeDef::Boolean]))
		.description("asdf")
		.build();
	assert_eq!(optional_bool.usage, "--key");

	let no_exported = Definition::builder()
		.key("methane")
		.default_value(TypeDef::StringV("CH4"))
		.type_def(TypeDef::String)
		.type_description("Greenhouse Gas")
		.description(
			"This is bad for the environment, for our children, do not put it there.",
		)
		.env_export(false)
		.build();
	assert!(!no_exported.env_export);
	assert_eq!(
		no_exported.describe(),
		"#### `methane`

* Default: \"CH4\"
* Type: Greenhouse Gas

This is bad for the environment, for our children, do not put it there.

This value is not exported to the environment for child processes."
	);
}

#[test]
fn test_long_description() {
	let long_def_builder = Definition::builder()
		.key("walden")
		.description(
			"
			WHEN I WROTE the following pages, or rather the bulk of them, I lived
			alone, in the woods, a mile from any neighbor, in a house which I had
			built myself, on the shore of Walden Pond, in Concord, Massachusetts, and
			earned my living by the labor of my hands only. I lived there two years
			and two months. At present I am a sojourner in civilized life again.

			I should not obtrude my affairs so much on the notice of my readers if
			very particular inquiries had not been made by my townsmen concerning my
			mode of life, which some would call impertinent, though they do not
			appear to me at all impertinent, but, considering the circumstances, very
			natural and pertinent.

			```
			this.is('a', {
			  code: 'sample',
			})

			with (multiple) {
			  blocks()
			}
			```
			",
		)
		.default_value(TypeDef::BooleanV(true))
		.type_def(TypeDef::Boolean);

	let long_40 = long_def_builder.clone().terminal_cols(40).build();
	assert_snapshot!(
		"long_40",
		long_40.describe(),
		"description when terminal columns is 40"
	);

	let long_9000 = long_def_builder.clone().terminal_cols(9000).build();
	assert_snapshot!(
		"long_9000",
		long_9000.describe(),
		"description when terminal columns is 9000"
	);

	let long_0 = long_def_builder.clone().terminal_cols(0).build();
	assert_snapshot!(
		"long_0",
		long_0.describe(),
		"description when terminal columns is 0"
	);
}
