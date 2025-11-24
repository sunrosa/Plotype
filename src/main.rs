// #![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links, unfulfilled_lint_expectations)]
#![warn(
  clippy::pedantic,
  clippy::nursery,
  clippy::cargo,
  clippy::style,
  clippy::perf,
  clippy::complexity
)]
#![allow(
  clippy::wildcard_imports,
  clippy::option_if_let_else,
  clippy::enum_glob_use,
  clippy::unreadable_literal
)]

mod prelude {
  #![allow(unused_imports)]
  #![doc(hidden)]

  pub(crate) use core::marker::PhantomData as PD;

  pub(crate) use thiserror::Error as ThisError;

  pub(crate) use cgp::prelude::{
    CanUseComponent, DelegateComponent, IsProviderFor, Life, UseContext, UseDelegate, UseType,
    cgp_component as cgp_comp, cgp_context as cgp_ctx, cgp_impl, cgp_preset as cgp_pre,
    delegate_and_check_components as del_check, delegate_components as del,
    re_export_imports as rex_im,
  };

  pub(crate) use crate::presets::*;
}

mod test_prelude {
  #![cfg(any(test, feature = "doctests"))]
  #![allow(unused_imports)]
  #![doc(hidden)]

  pub(crate) use {
    assert_matches::{assert_matches, debug_assert_matches},
    more_asserts::{
      assert_ge, assert_gt, assert_le, assert_lt, debug_assert_ge, debug_assert_gt,
      debug_assert_le, debug_assert_lt, debug_unreachable,
    },
  };
}

mod dependency;
mod event;
mod id;
mod presets;

fn main() {}
