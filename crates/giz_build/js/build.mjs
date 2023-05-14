import { buildSync } from "esbuild";

buildSync({
  bundle: true,
  target: ["chrome58"],
  entryPoints: ["./entry.js"],
  outfile: "out.js",
  nodePaths: ["std"],
});
