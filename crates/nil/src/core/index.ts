import { Round } from './round';
import { CurrentPlayer } from './current-player';
import { CurrentVillage } from './current-village';

export function initCore() {
  if (!Object.hasOwn(window, 'NIL')) {
    Object.defineProperty(window, 'NIL', {
      configurable: false,
      enumerable: true,
      value: {},
      writable: false,
    });
  }

  CurrentPlayer.init();
  CurrentVillage.init();
  Round.init();
}
