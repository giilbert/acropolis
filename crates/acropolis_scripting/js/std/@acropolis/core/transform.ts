import { Entity } from ".";
import { getComponentVector3Property } from "./deno";
import { Vector3 } from "./math/vector3";

const transformComponentId =
  // @ts-ignore
  __ACROPOLIS_COMPONENT["acropolis_math::components::transform::Transform"];

export class Transform {
  position: Vector3;
  private entity: Entity | undefined;

  constructor() {
    this.position = new Vector3(0, 0, 0);
  }

  static withEntity(entity: Entity): Transform {
    const newTransform = new Transform();
    newTransform.entity = entity;
    const { x, y, z } = getComponentVector3Property(
      entity,
      transformComponentId,
      0
    );
    newTransform.position = Vector3.withEntity(
      x,
      y,
      z,
      transformComponentId,
      entity,
      0
    );
    return newTransform;
  }
}
