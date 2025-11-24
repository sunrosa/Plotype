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
  HappenAfterEnd(AfterEnd<ID>),
  HappenBeforeStart(BeforeStart<ID>),
  HappenAfterStart(AfterStart<ID>),
  HappenBeforeEnd(BeforeEnd<ID>),
  HappenDuring(During<ID>),
}

#[cgp_ctx(AfterEndComponents: CgpDefault)]
pub struct AfterEnd<ID>(pub ID);
#[cgp_ctx(BeforeStartComponents: CgpDefault)]
pub struct BeforeStart<ID>(pub ID);
#[cgp_ctx(AfterStartComponents: CgpDefault)]
pub struct AfterStart<ID>(pub ID);
#[cgp_ctx(BeforeEndComponents: CgpDefault)]
pub struct BeforeEnd<ID>(pub ID);
#[cgp_ctx(DuringComponents: CgpDefault)]
pub struct During<ID>(pub ID);

#[cgp_impl(AfterEndComponents)]
impl<ID> ProvideHappenAfterEndDependency for AfterEnd<ID>
where
  ID: Clone,
{
  type ID = ID;
  fn must_happen_after(&self) -> Self::ID {
    self.0.clone()
  }
}

#[cgp_impl(BeforeStartComponents)]
impl<ID> ProvideHappenBeforeStartDependency for BeforeStart<ID>
where
  ID: Clone,
{
  type ID = ID;
  fn must_happen_before(&self) -> Self::ID {
    self.0.clone()
  }
}

#[cgp_impl(AfterStartComponents)]
impl<ID> ProvideHappenAfterStartDependency for AfterStart<ID>
where
  ID: Clone,
{
  type ID = ID;
  fn must_happen_after_start(&self) -> Self::ID {
    self.0.clone()
  }
}

#[cgp_impl(BeforeEndComponents)]
impl<ID> ProvideHappenBeforeEndDependency for BeforeEnd<ID>
where
  ID: Clone,
{
  type ID = ID;
  fn must_happen_before_end(&self) -> Self::ID {
    self.0.clone()
  }
}

#[cgp_impl(DuringComponents)]
impl<ID> ProvideHappenDuringDependency for During<ID>
where
  ID: Clone,
{
  type ID = ID;
  fn must_happen_during(&self) -> Self::ID {
    self.0.clone()
  }
}
