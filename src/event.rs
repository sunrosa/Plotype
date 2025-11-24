use crate::prelude::*;

/// Implementor occurred at a specific date
#[cgp_comp(ProvideDate)]
pub trait HasDate {
  type Date;
  fn date(&self) -> Self::Date;
}
