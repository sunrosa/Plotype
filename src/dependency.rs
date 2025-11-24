use crate::prelude::*;

#[cgp_comp(ProvideHappenAfterEndDependency)]
pub trait HasHappenAfterEndDependency {
  type ID;
  /// Must happen at any time after the end of event with [`Self::ID`]
  fn must_happen_after(&self) -> Self::ID;
}

#[cgp_comp(ProvideHappenBeforeStartDependency)]
pub trait HasHappenBeforeStartDependency {
  type ID;
  /// Must happen at any time befrore the start of event with [`Self::ID`]
  fn must_happen_before(&self) -> Self::ID;
}

#[cgp_comp(ProvideHappenAfterStartDependency)]
pub trait HasHappenAfterStartDependency {
  type ID;
  /// Must happen at any time after the start of event with [`Self::ID`]
  fn must_happen_after_start(&self) -> Self::ID;
}

#[cgp_comp(ProvideHappenBeforeEndDependency)]
pub trait HasHappenBeforeEndDependency {
  type ID;
  /// Must happen at any time before the end of event with [`Self::ID`]
  fn must_happen_before_end(&self) -> Self::ID;
}

#[cgp_comp(ProvideHappenDuringDependency)]
pub trait HasHappenDuringDependency {
  type ID;
  /// Must happen at any during the event with [`Self::ID`]
  fn must_happen_during(&self) -> Self::ID;
}

pub enum TimeDependency<ID> {
  HappenAfterEnd(ID),
  HappenBeforeStart(ID),
  HappenAfterStart(ID),
  HappenBeforeEnd(ID),
  HappenDuring(ID),
}
