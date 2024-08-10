#![allow(unused)]

use std::any::{Any, TypeId};

pub struct Definition<T> {
	pub key: &'static str,
	pub default_value: Option<T>,
	pub possible_values: Option<Vec<T>>,
	pub description: &'static str,
	pub env_export: bool,
	pub default_description: String,
	pub type_description: String,
	pub hint: String,
	pub short: Option<&'static [&'static str]>,
	pub usage: String,
	pub deprecated: Option<&'static str>,
	pub exclusive: Option<&'static [&'static str]>,
	#[allow(unused)]
	multiple: bool,
	width: usize,
}

fn describe_value(value: &dyn Any) -> String {
	if let Some(v) = value.downcast_ref::<String>() {
		return format!("\"{}\"", v);
	}

	if let Some(v) = value.downcast_ref::<f64>() {
		return v.to_string();
	}

	if let Some(v) = value.downcast_ref::<bool>() {
		return v.to_string();
	}

	if let Some(v) = value.downcast_ref::<Vec<String>>() {
		return v.join(",");
	}

	if let Some(v) = value.downcast_ref::<Vec<f64>>() {
		return v
			.iter()
			.map(|n| n.to_string())
			.collect::<Vec<String>>()
			.join(",");
	}

	if let Some(v) = value.downcast_ref::<Vec<bool>>() {
		return v
			.iter()
			.map(|n| n.to_string())
			.collect::<Vec<String>>()
			.join(",");
	}

	panic!("unsupport type");
}

fn describe_type(value: &dyn Any) -> String {
	if value.type_id() == TypeId::of::<String>() {
		return "String".to_string();
	}

	if value.type_id() == TypeId::of::<f64>() {
		return "Number".to_string();
	}

	if value.type_id() == TypeId::of::<bool>() {
		return "Boolean".to_string();
	}

	unimplemented!();
}
