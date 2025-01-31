/* eslint-disable no-extend-native */
import type { MaybePromise } from '@tb-dev/utils';
import { handleError as onError } from '@/lib/error';

Error.throw = function (message: string): never {
  throw new this(message);
};

Promise.prototype.handleError = function (onfinally?: () => MaybePromise<void>) {
  const promise = this.catch(onError);
  if (onfinally) {
    void promise.finally(() => Promise.try(onfinally).handleError());
  }
};
