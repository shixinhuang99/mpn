use std::{collections::BTreeMap, sync::LazyLock};

use crate::{definition::Definition, type_def::TypeDef};

type Definitions = BTreeMap<&'static str, Definition>;

pub static DEFINITIONS: LazyLock<Definitions> = LazyLock::new(init_definitions);

fn init_definitions() -> Definitions {
	BTreeMap::from([
        ("_auth", Definition::builder()
        .key("_auth")
        .default_value(TypeDef::Null)
        .type_def(TypeDef::Array(&[TypeDef::Null, TypeDef::String]))
        .description("
            A basic-auth string to use when authenticating against the npm registry.
            This will ONLY be used to authenticate against the npm registry.  For other
            registries you will need to scope it like \"//other-registry.tld/:_auth\"

            Warning: This should generally not be set via a command-line option.  It
            is safer to use a registry-provided authentication bearer token stored in
            the ~/.npmrc file by running `npm login`.
        ")
        .build()),
    ])
}
