/* eslint-disable no-extend-native */
import { handleError as onError } from '@/lib/error';

Error.panic = function (message: string): never {
  throw new this(message);
};

Error.todo = function (message = 'not yet implemented'): never {
  throw new this(`TODO: ${message}`);
};

Error.unimplemented = function (message = 'not implemented'): never {
  throw new this(message);
};

Promise.prototype.handleError = function () {
  this.catch(onError);
};
