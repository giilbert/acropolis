import { Entity } from ".";
import { getComponentProperty } from "./deno";
import { Vector3 } from "./math/vector3";

const transformComponentId =
  // @ts-ignore
  __GIZ_COMPONENT["giz_math::components::transform::Transform"];

export class Transform {
  position: Vector3;
  private entity: Entity | undefined;

  constructor() {
    this.position = new Vector3(0, 0, 0);
  }

  static withEntity(entity: Entity): Transform {
    const newTransform = new Transform();
    newTransform.entity = entity;
    const { x, y, z } = getComponentProperty(
      entity,
      transformComponentId,
      "position"
    );
    newTransform.position = Vector3.withEntity(
      x,
      y,
      z,
      transformComponentId,
      entity,
      "position"
    );
    return newTransform;
  }
}
