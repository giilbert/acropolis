import { Entity } from ".";

export function setComponentProperty(
  entity: Entity,
  componentId: number,
  key: string,
  value: any
) {
  // @ts-ignore
  Deno.core.ops.op_set_component_prop(
    entity.id,
    componentId,
    key,
    JSON.stringify(value)
  );
}

export function getComponentProperty(
  entity: Entity,
  componentId: number,
  key: string
) {
  return JSON.parse(
    // @ts-ignore
    Deno.core.ops.op_get_component_prop(entity.id, componentId, key)
  );
}
