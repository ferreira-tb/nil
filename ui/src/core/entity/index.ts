// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from './abstract';
import { ChatEntity } from './chat';
import { RoundEntity } from './round';
import { WorldEntity } from './world';
import { CurrentPlayerEntity } from './current-player';
import { CurrentVillageEntity } from './current-village';

export function initEntities() {
  if (!Object.hasOwn(window, 'NIL')) {
    Object.defineProperty(window, 'NIL', {
      configurable: false,
      enumerable: true,
      writable: false,
      value: {
        update: () => Entity.updateAll(),
      },
    });
  }

  WorldEntity.init();
  ChatEntity.init();
  RoundEntity.init();
  CurrentPlayerEntity.init();
  CurrentVillageEntity.init();
}
