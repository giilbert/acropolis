// @ts-ignore
import wasmUrl from "../pkg/web_bg.wasm?url";
// @ts-ignore
import projectZipUrl from "../test-project.zip?url";
// @ts-ignore
import {
  initSync,
  op_get_component_prop,
  op_get_key_down,
  op_set_component_prop,
} from "../pkg";
import * as zip from "@zip.js/zip.js";

// @ts-ignore
window.__ACROPOLIS_COMPONENT = {};
declare global {
  interface Window {
    __ACROPOLIS__: {
      entries: Record<string, zip.Entry>;
    };
  }
}

async function downloadWithProgress(wasmUrl: string, indicator: string) {
  console.time(indicator);
  const res = await fetch(wasmUrl);
  const reader = res.body?.getReader();
  const contentLength = Number(res.headers.get("Content-Length"));

  if (!reader) {
    throw new Error("No reader");
  }

  let receivedLength = 0;
  let chunks: Uint8Array[] = [];
  while (true) {
    const { done, value } = await reader.read();

    if (done) {
      break;
    }

    chunks.push(value);
    receivedLength += value.length;

    // console.log(
    //   `[${indicator}] Current progress: ${receivedLength}/${contentLength} (${Math.floor(
    //     (receivedLength / contentLength) * 100
    //   )}%))`
    // );
  }

  let bytes = new Uint8Array(receivedLength);
  let position = 0;
  for (let chunk of chunks) {
    bytes.set(chunk, position);
    position += chunk.length;
  }

  console.timeEnd(indicator);

  return bytes;
}

function bytesToString(bytes: Uint8Array) {
  const decoder = new TextDecoder("utf-8");
  return decoder.decode(bytes);
}

async function run() {
  const bytes = await downloadWithProgress(
    projectZipUrl,
    "Downloading project"
  );
  // read the contents of the files using zip.js
  const reader = new zip.ZipReader(new zip.Uint8ArrayReader(bytes));
  const entries = await reader.getEntries();
  const entriesDict: Record<string, Uint8Array> = {};

  for (let entry of entries) {
    if (!entry.getData) {
      throw new Error("No getData");
    }

    // TODO: promise all ify
    const data = await entry.getData(new zip.Uint8ArrayWriter());
    entriesDict[entry.filename] = data;
  }

  const bundleCode = bytesToString(entriesDict["bundle.js"]);

  console.log(bundleCode);

  const __ACROPOLIS__ = {
    entries: entriesDict,
    behaviors: {},
    keysDown: new Set(),
    scripting: 0,
    set_component_ids(ids: string) {
      const data = JSON.parse(ids);
      // @ts-ignore
      window.__ACROPOLIS_COMPONENT = data;

      eval(bundleCode);
    },
    js_read_file(path: string) {
      const entry: Uint8Array = globalThis.__ACROPOLIS__.entries[path];
      return entry;
    },
    run_once() {
      // @ts-ignore
      __ACROPOLIS__.scripting.runOnce();
    },
    create_behavior(filePath: string, entityId: string, behaviorId: string) {
      // @ts-ignore
      __ACROPOLIS__.scripting.createBehavior(filePath, entityId, behaviorId);
    },
    is_key_down(key: string) {
      return __ACROPOLIS__.keysDown.has(key);
    },

    get_component_prop: op_get_component_prop,
    set_component_prop: op_set_component_prop,
  };
  globalThis.__ACROPOLIS__ = __ACROPOLIS__;

  window.addEventListener("keydown", (e) => {
    __ACROPOLIS__.keysDown.add(e.key.toUpperCase());
  });

  window.addEventListener("keyup", (e) => {
    __ACROPOLIS__.keysDown.delete(e.key.toUpperCase());
  });

  const wasmBytes = await downloadWithProgress(wasmUrl, "Downloading wasm");
  const wasm = initSync(wasmBytes);
  wasm.run();
}

run();
