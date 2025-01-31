export function app() {
  return window.__NIL__.app;
}

export function runWithContext<T>(fn: () => T) {
  return window.__NIL__.app.runWithContext(fn);
}
