pub(crate) mod pikeman;
pub(crate) mod swordsman;
pub (crate) mod axeman;

mod prelude {
  pub use nil_macros::Unit;

  pub use nil_core::{UnitKind, Haul, Power, Speed, UnitId, UnitStats};
}
