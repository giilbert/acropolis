((files) => {
  const exports = {};

  for (const path of Object.keys(files)) {
    exports[path] = {};
  }

  for (const [path, instantiate] of Object.entries(files)) {
    const require = (path) => exports[path];
    instantiate(require, exports[path]);
  }

  console.log(exports);

  globalThis.exports = exports;
})({
  "src/hello.js": (require, exports) => {
    "use strict";
    Object.defineProperty(exports, "__esModule", {
      value: true,
    });
    Object.defineProperty(exports, "a", {
      enumerable: true,
      get: () => a,
    });
    const _rotateJs = require("src/rotate.js");
    const a = _rotateJs.b;
  },

  "src/rotate.js": (require, exports) => {
    "use strict";
    Object.defineProperty(exports, "__esModule", {
      value: true,
    });
    Object.defineProperty(exports, "b", {
      enumerable: true,
      get: () => b,
    });
    const _helloJs = require("src/hello.js");
    const b = _helloJs.a;
    console.log(b);
  },
});
