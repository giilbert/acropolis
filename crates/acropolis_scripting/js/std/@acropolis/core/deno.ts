import { Entity, Vector3 } from ".";

export function setComponentProperty(
  entity: Entity,
  componentId: number,
  key: string,
  value: any
) {
  // @ts-ignore
  if (typeof Deno === "undefined") {
    // @ts-ignore
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
    // @ts-ignore
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

export function getComponentVector3Property(
  entity: Entity,
  componentId: number,
  key: number
): Vector3 {
  // @ts-ignore
  if (typeof Deno === "undefined") {
    throw new Error("TODO: implement getComponentVector3Property for the web");
  } else {
    const ret: [number, number, number] =
      // @ts-ignore
      Deno.core.ops.op_get_component_vec3_prop(entity.id, componentId, key);

    return new Vector3(...ret);
  }
}

export function setComponentVector3Property(
  entity: Entity,
  componentId: number,
  key: number,
  value: Vector3
) {
  // @ts-ignore
  if (typeof Deno === "undefined") {
    throw new Error("TODO: implement setComponentVector3Property for the web");
  } else {
    // @ts-ignore
    Deno.core.ops.op_set_component_vec3_prop(
      entity.id,
      componentId,
      key,
      value.x,
      value.y,
      value.z
    );
  }
}
