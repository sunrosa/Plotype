use crate::prelude::*;

#[cgp_comp(ProvideUniqueID)]
pub trait HasUniqueID {
  type ID;
  fn unique_id(&self) -> Self::ID;
}
