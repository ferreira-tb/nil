// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod index;

use crate::behavior::index::IdleBehavior;
use crate::error::Result;
use crate::world::World;
use derive_more::Into;
use rand::seq::IndexedRandom;
use std::any::Any;
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

pub trait Behavior: Any {
  fn score(&self, world: &World) -> Result<BehaviorScore>;
  fn behave(&self, world: &mut World) -> Result<()>;

  fn boxed(self) -> Box<dyn Behavior>
  where
    Self: Sized + 'static,
  {
    Box::new(self)
  }
}

pub(crate) fn process(world: &mut World, behaviors: &[Box<dyn Behavior>]) -> Result<()> {
  let mut buffer = Vec::with_capacity(behaviors.len());
  let mut candidates = Vec::new();
  let mut rng = rand::rng();
  loop {
    for (idx, behavior) in behaviors.iter().enumerate() {
      buffer.push((idx, behavior.score(world)?));
    }

    buffer.sort_unstable_by_key(|(_, score)| *score);

    if buffer
      .last()
      .is_none_or(|(idx, _)| is_idle(behaviors, *idx))
    {
      break;
    }

    let (_, highest_score) = buffer[buffer.len() - 1];
    buffer
      .iter()
      .filter(|(_, score)| highest_score.is_within_range(*score, 0.2))
      .map(|(idx, score)| (*idx, f64::from(*score)))
      .collect_into(&mut candidates);

    let Some(idx) = candidates
      .choose_weighted(&mut rng, |(_, score)| *score)
      .ok()
      .map(|(idx, _)| *idx)
      .filter(|idx| !is_idle(behaviors, *idx))
    else {
      break;
    };

    buffer.clear();
    candidates.clear();
    behaviors[idx].behave(world)?;
  }

  Ok(())
}

fn is_idle(behaviors: &[Box<dyn Behavior>], idx: usize) -> bool {
  (behaviors[idx].as_ref() as &dyn Any).is::<IdleBehavior>()
}

#[derive(Clone, Copy, Debug, Into)]
pub struct BehaviorScore(f64);

impl BehaviorScore {
  #[inline]
  pub const fn new(score: f64) -> Self {
    debug_assert!(score.is_finite());
    debug_assert!(!score.is_subnormal());
    Self(score.clamp(0.0, 1.0))
  }

  #[inline]
  pub(crate) fn is_within_range(self, other: BehaviorScore, range: f64) -> bool {
    (self.0 - other.0).abs() < range
  }
}

impl Default for BehaviorScore {
  fn default() -> Self {
    Self(0.0)
  }
}

impl PartialEq for BehaviorScore {
  fn eq(&self, other: &Self) -> bool {
    matches!(self.0.total_cmp(&other.0), Ordering::Equal)
  }
}

impl Eq for BehaviorScore {}

impl PartialOrd for BehaviorScore {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for BehaviorScore {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.total_cmp(&other.0)
  }
}

impl PartialEq<f64> for BehaviorScore {
  fn eq(&self, other: &f64) -> bool {
    self.0.eq(other)
  }
}

impl PartialOrd<f64> for BehaviorScore {
  fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}

impl Add<f64> for BehaviorScore {
  type Output = f64;

  fn add(self, rhs: f64) -> Self::Output {
    self.0 + rhs
  }
}

impl Sub<f64> for BehaviorScore {
  type Output = f64;

  fn sub(self, rhs: f64) -> Self::Output {
    self.0 - rhs
  }
}

impl Mul<f64> for BehaviorScore {
  type Output = f64;

  fn mul(self, rhs: f64) -> Self::Output {
    self.0 * rhs
  }
}

impl Div<f64> for BehaviorScore {
  type Output = f64;

  fn div(self, rhs: f64) -> Self::Output {
    self.0 / rhs
  }
}
