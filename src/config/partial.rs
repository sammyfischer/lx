//! Defines config structs, but with all fields made optional. This is to help when merging structs.
//! `None` values get ignored and `Some` values override the previously set values.

use serde::{Deserialize, Serialize};

use crate::config::Style;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PartialConfig {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub style: Option<Style>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub long: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub interactive: Option<bool>,

  #[serde(default)]
  pub eza: PartialEzaConfig,

  #[serde(default)]
  pub pager: PartialPagerConfig,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PartialEzaConfig {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub args: Option<Vec<String>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub interactive_args: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PartialPagerConfig {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bin: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub args: Option<Vec<String>>,
}
