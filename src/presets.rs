use crate::prelude::*;

#[rex_im]
mod preset {
  #![allow(unused_imports)]
  use super::*;
  use crate::event::{
    FromDate, FromStartAndEnd, ProvideSpanComponent, ProvideSpanEndComponent,
    ProvideSpanStartComponent,
  };

  cgp_pre! {
    CgpDefault {
      ProvideSpanComponent: FromStartAndEnd<u64>,
      [
        ProvideSpanStartComponent,
        ProvideSpanEndComponent
      ]: FromDate<u64>
    }
  }
}
