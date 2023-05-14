export function keyDown(key: string) {
  // @ts-ignore
  return Deno.core.ops.op_get_key_down(`"${key}"`);
}
