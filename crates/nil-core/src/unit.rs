use derive_more::{Deref, Display};
use std::num::NonZeroU32;

pub trait Unit: Send + Sync {
  fn id(&self) -> UnitId;
  fn amount(&self) -> u32;
  fn stats(&self) -> UnitStats;
}

#[derive(Clone, Copy, Debug, Deref, Display)]
pub struct UnitId(NonZeroU32);

impl UnitId {
  pub const fn new(value: u32) -> Self {
    Self(NonZeroU32::new(value).unwrap())
  }
}

#[derive(Clone, Copy, Debug)]
pub struct UnitStats {
  pub attack: Power,
  pub general_defense: Power,
  pub cavalry_defense: Power,
  pub speed: Speed,
  pub haul: Haul,
}

#[derive(Clone, Copy, Debug, Deref)]
pub struct Power(u16);

impl Power {
  pub const fn new(value: u16) -> Self {
    Self(value)
  }
}

impl From<Power> for f64 {
  fn from(power: Power) -> Self {
    f64::from(power.0)
  }
}

#[derive(Clone, Copy, Debug, Deref)]
pub struct Speed(f64);

impl Speed {
  pub const fn new(value: f64) -> Self {
    Self(value.max(0.0))
  }
}

#[derive(Clone, Copy, Debug, Deref)]
pub struct Haul(u16);

impl Haul {
  pub const fn new(value: u16) -> Self {
    Self(value)
  }
}

impl From<Haul> for f64 {
  fn from(haul: Haul) -> Self {
    f64::from(haul.0)
  }
}

#[derive(Deref)]
pub struct UnitBox(Box<dyn Unit>);

impl UnitBox {
  pub fn new(unit: Box<dyn Unit>) -> Self {
    Self(unit)
  }
}
