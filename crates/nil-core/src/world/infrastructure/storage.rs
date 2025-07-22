// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::infrastructure::building::StorageId;
use crate::infrastructure::storage::OverallStorageCapacity;
use crate::village::Village;
use crate::world::World;

impl World {
  pub(crate) fn get_storage_capacity<'a, V>(&self, villages: V) -> Result<OverallStorageCapacity>
  where
    V: IntoIterator<Item = &'a Village>,
  {
    let silo_stats = self
      .stats
      .infrastructure
      .storage(StorageId::Silo)?;

    let warehouse_stats = self
      .stats
      .infrastructure
      .storage(StorageId::Warehouse)?;

    villages
      .into_iter()
      .try_fold(OverallStorageCapacity::default(), |mut acc, village| {
        let infra = village.infrastructure();
        acc.silo += infra
          .storage(StorageId::Silo)
          .capacity(silo_stats)?;

        acc.warehouse += infra
          .storage(StorageId::Warehouse)
          .capacity(warehouse_stats)?;

        Ok(acc)
      })
  }
}
