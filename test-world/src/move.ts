import { Behavior, Entity, Vector2 } from "@acropolis/core";
import { keyDown } from "@acropolis/input";
import { RigidBody2D } from "@acropolis/physics";

class Move extends Behavior {
  rigidbody: RigidBody2D;

  constructor(entity: Entity) {
    super(entity);
    this.rigidbody = entity.get(RigidBody2D);
  }

  update() {
    if (keyDown("W")) this.rigidbody.applyImpulse(new Vector2(0, 1));
  }
}

export default Move;
