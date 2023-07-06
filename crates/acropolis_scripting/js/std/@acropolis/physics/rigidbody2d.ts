import { Entity, Vector2 } from "../core";
import { callComponentMethod } from "../core/deno";

const rigidbody2dComponentId =
  // @ts-ignore
  __ACROPOLIS_COMPONENT[
    "acropolis_physics::components::rigidbody2d::RigidBody2D"
  ];

export class RigidBody2D {
  private entity: Entity | undefined;

  constructor() {}

  addForce(force: Vector2) {
    if (!this.entity) return;

    callComponentMethod(this.entity, rigidbody2dComponentId, 0, [
      force.x,
      force.y,
    ]);
  }

  applyImpulse(impulse: Vector2) {
    if (!this.entity) return;

    callComponentMethod(this.entity, rigidbody2dComponentId, 1, [
      impulse.x,
      impulse.y,
    ]);
  }

  static withEntity(entity: Entity) {
    const newRigidbody2D = new RigidBody2D();
    newRigidbody2D.entity = entity;
    return newRigidbody2D;
  }

  static id() {
    return rigidbody2dComponentId;
  }
}
