export function keyDown(key: string) {
  // @ts-ignore
  if (typeof Deno !== "undefined") {
    // @ts-ignore
    return Deno.core.ops.op_get_key_down(`"${key}"`);
  }

  return window.__ACROPOLIS__.is_key_down(key);
}
