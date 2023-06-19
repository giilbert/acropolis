import { Behavior, Entity } from "@acropolis/core";
import { keyDown } from "@acropolis/input";

class Camera extends Behavior {
  constructor(entity: Entity) {
    super(entity);
  }

  update() {
    if (keyDown("Q")) this.transform.position.z += 1;
    if (keyDown("E")) this.transform.position.z -= 1;
  }
}

export default Camera;
