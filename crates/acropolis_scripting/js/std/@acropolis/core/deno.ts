import { Entity } from ".";

export function setComponentProperty(
  entity: Entity,
  componentId: number,
  key: string,
  value: any
) {
  // @ts-ignore
  if (typeof Deno === "undefined") {
    __ACROPOLIS__.set_component_prop(
      entity.id,
      componentId,
      key,
      JSON.stringify(value)
    );
  } else {
    // @ts-ignore
    Deno.core.ops.op_set_component_prop(
      entity.id,
      componentId,
      key,
      JSON.stringify(value)
    );
  }
}

export function getComponentProperty(
  entity: Entity,
  componentId: number,
  key: string
) {
  // @ts-ignore
  if (typeof Deno === "undefined") {
    const d = __ACROPOLIS__.get_component_prop(entity.id, componentId, key);
    console.log(d);
    return JSON.parse(d);
  } else {
    return JSON.parse(
      // @ts-ignore
      Deno.core.ops.op_get_component_prop(entity.id, componentId, key)
    );
  }
}
