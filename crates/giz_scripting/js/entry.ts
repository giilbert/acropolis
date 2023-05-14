// @ts-ignore
import { files as f } from "./files";
import { Entity } from "./std/@giz/core";

// maps a behavior id to a behavior
const behaviors = {};

function createBehavior(filePath, entityId, behaviorId) {
  behaviors[behaviorId] = new f[filePath]["default"](new Entity(entityId));
}

let last = Date.now();
function runOnce() {
  let elapsed = Date.now() - last;
  // @ts-ignore
  // Deno.core.print(`scripting elapsed: ${elapsed}ms\n`);

  // @ts-ignore
  for (const behavior of Object.values(behaviors)) {
    behavior.update();
  }

  last = Date.now();
}

globalThis.__giz__ = {
  createBehavior,
  runOnce,
};

// @ts-ignore
globalThis.console = {
  log: (...args) => {
    // @ts-ignore
    Deno.core.print(
      `[LOG] ${args.map((v) => JSON.stringify(v, undefined, "  ")).join(" ")}\n`
    );
  },
};
