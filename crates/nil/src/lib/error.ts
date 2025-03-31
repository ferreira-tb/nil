import * as dialog from '@/lib/dialog';
import type { Option } from '@tb-dev/utils';

export function handleError(err: unknown, message?: Option<string>) {
  console.error(err);
  if (message) {
    dialog.error(message);
  } else if (err instanceof Error) {
    dialog.error(err.message);
  } else {
    dialog.error(String(err));
  }
}
