// Copyright (C) Tsukilabs contributors
// SPDX-License-Identifier: AGPL-3.0-only

/* eslint-disable no-extend-native */
import '@tb-dev/prototype-dom';
import { handleError } from '@/lib/error';
import type { Option } from '@tb-dev/utils';

Error.panic = function (message: string): never {
  throw new this(message);
};

Error.throw = function (message: string): never {
  throw new this(message);
};

Error.todo = function (message?: Option<string>): never {
  throw new this(`TODO: ${message ?? 'not yet implemented'}`);
};

Promise.prototype.err = function (message?: Option<string>) {
  this.catch((err: unknown) => handleError(err, message));
};
