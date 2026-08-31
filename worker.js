// ES-module Web Worker: loads the wasm compute core and runs analyze().
import initWasm, { analyze, init as setPanicHook } from "./pkg/chance_encounters.js";

// Kick off wasm instantiation immediately; reuse the promise across messages.
const ready = initWasm().then(() => setPanicHook());

self.onmessage = async (event) => {
  const { file1, file2, opts } = event.data;
  try {
    await ready;
    const progress = (pct) => self.postMessage({ type: "progress", pct });
    const result = analyze(file1, file2, opts ?? {}, progress);
    self.postMessage({ type: "result", result });
  } catch (err) {
    self.postMessage({ type: "error", message: String(err && err.message ? err.message : err) });
  }
};
