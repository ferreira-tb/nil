// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use serde::{Deserialize, Serialize};
use std::fmt;
use std::iter::repeat_n;
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Roman(Box<[Numeral]>);

impl Roman {
  const MIN: usize = 1;
  const MAX: usize = 3999;

  pub fn parse(value: impl ToRoman) -> Option<Self> {
    value.to_roman()
  }
}

impl Default for Roman {
  fn default() -> Self {
    Self(Box::from([Numeral::I]))
  }
}

impl fmt::Display for Roman {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self
      .0
      .iter()
      .try_for_each(|numeral| write!(f, "{numeral}"))
  }
}

impl From<&Roman> for u16 {
  fn from(roman: &Roman) -> Self {
    let mut value = 0u16;
    for numeral in &roman.0 {
      let numeral = u16::from(*numeral);
      value = value.saturating_add(numeral);
    }

    value
  }
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Deserialize, Serialize, EnumIter)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum Numeral {
  I,
  IV,
  V,
  IX,
  X,
  XL,
  L,
  XC,
  C,
  CD,
  D,
  CM,
  M,
}

impl From<Numeral> for u16 {
  fn from(numeral: Numeral) -> Self {
    match numeral {
      Numeral::I => 1,
      Numeral::IV => 4,
      Numeral::V => 5,
      Numeral::IX => 9,
      Numeral::X => 10,
      Numeral::XL => 40,
      Numeral::L => 50,
      Numeral::XC => 90,
      Numeral::C => 100,
      Numeral::CD => 400,
      Numeral::D => 500,
      Numeral::CM => 900,
      Numeral::M => 1000,
    }
  }
}

macro_rules! impl_from_numeral {
  ($($target:ident),+ $(,)?) => {
    $(
      impl From<Numeral> for $target {
        fn from(numeral: Numeral) -> Self {
          u16::from(numeral).into()
        }
      }
    )+
  };
}

impl_from_numeral!(i32, i64, u32, u64, usize);

pub trait ToRoman {
  fn to_roman(self) -> Option<Roman>;
}

impl ToRoman for usize {
  fn to_roman(mut self) -> Option<Roman> {
    if self < Roman::MIN || self > Roman::MAX {
      None
    } else {
      let mut roman = Vec::new();
      for numeral in Numeral::iter().rev() {
        if self == 0 {
          break;
        }

        let value = usize::from(numeral);
        let count = self.saturating_div(value);
        roman.extend(repeat_n(numeral, count));
        self = self.saturating_sub(count * value);
      }

      Some(Roman(roman.into_boxed_slice()))
    }
  }
}

macro_rules! impl_unsigned_to_roman {
  ($($num:ident),+ $(,)?) => {
    $(
      impl ToRoman for $num {
        fn to_roman(self) -> Option<Roman> {
          (self as usize).to_roman()
        }
      }
    )+
  };
}

impl_unsigned_to_roman!(u8, u16, u32, u64, u128);

macro_rules! impl_signed_to_roman {
  ($($num:ident),+ $(,)?) => {
    $(
      impl ToRoman for $num {
        fn to_roman(self) -> Option<Roman> {
          self.unsigned_abs().to_roman()
        }
      }
    )+
  };
}

impl_signed_to_roman!(i8, i16, i32, i64, i128);
