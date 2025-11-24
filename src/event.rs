use crate::prelude::*;

/// Implementor occurred at a specific oneshot date
#[cgp_comp(ProvideDate)]
pub trait HasDate {
  type Date;

  /// The date that the implementor occurred
  fn date(&self) -> Self::Date;
}

/// Implementor describes the start of a span (such as a character's life)
#[cgp_comp(ProvideSpanStart)]
pub trait HasSpanStart {
  type Date;

  /// The date that the implementor's span started
  fn span_start(&self) -> Self::Date;
}

/// Implementor describes the end of a span (such as a character's life)
#[cgp_comp(ProvideSpanEnd)]
pub trait HasSpanEnd {
  type Date;

  /// The date that the implementor's span ended
  fn span_end(&self) -> Self::Date;
}

/// Implementor describes a span (such as a character's life)
#[cgp_comp(ProvideSpan)]
pub trait HasSpan {
  type Date;
  fn span(&self) -> TimeSpan<Self::Date>;
}

pub struct TimeSpan<Date> {
  pub start: Date,
  pub end: Date,
}

/// Constructs [`HasSpan`] from [`HasSpanStart`] and [`HasSpanEnd`]
#[allow(dead_code)]
pub struct FromStartAndEnd<Date>(PD<Date>);
#[cgp_impl(FromStartAndEnd<Date>)]
impl<S, Date> ProvideSpan for S
where
  S: HasSpanStart<Date = Date> + HasSpanEnd<Date = Date>,
{
  type Date = Date;
  fn span(&self) -> TimeSpan<Self::Date> {
    TimeSpan {
      start: self.span_start(),
      end: self.span_end(),
    }
  }
}

/// Constructs [`HasSpanStart`] and [`HasSpanEnd`] from [`HasDate`]
#[allow(dead_code)]
pub struct FromDate<Date>(PD<Date>);

#[cgp_impl(FromStartAndEnd<Date>)]
impl<S, Date> ProvideSpanStart for S
where
  S: HasDate<Date = Date>,
{
  type Date = Date;
  fn span_start(&self) -> Date {
    self.date()
  }
}

#[cgp_impl(FromStartAndEnd<Date>)]
impl<S, Date> ProvideSpanEnd for S
where
  S: HasDate<Date = Date>,
{
  type Date = Date;
  fn span_end(&self) -> Date {
    self.date()
  }
}
