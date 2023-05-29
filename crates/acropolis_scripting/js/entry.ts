// @ts-ignore
import { files as f } from "./files";
import { Entity } from "./std/@acropolis/core";

// maps a behavior id to a behavior
const behaviors = {};

function createBehavior(
  filePath: string,
  entityId: number,
  behaviorId: number
) {
  behaviors[behaviorId] = new f[filePath]["default"](new Entity(entityId));
}

let last = Date.now();
function runOnce() {
  let elapsed = Date.now() - last;
  // console.log(`scripting elapsed: ${elapsed}ms\n`);

  // @ts-ignore
  for (const behavior of Object.values(behaviors)) {
    behavior.update();
  }

  last = Date.now();
}

if (!globalThis.__ACROPOLIS__) {
  // @ts-ignore
  globalThis.__ACROPOLIS__ = {};
}

globalThis.__ACROPOLIS__.scripting = {
  createBehavior,
  runOnce,
};

// @ts-ignore
if (typeof Deno !== "undefined") {
  // @ts-ignore
  globalThis.console = {
    log: (...args) => {
      // @ts-ignore
      Deno.core.print(
        `[LOG] ${args
          .map((v) => JSON.stringify(v, undefined, "  "))
          .join(" ")}\n`
      );
    },
  };
}
