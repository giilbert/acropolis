import { Behavior, Entity } from "@acropolis/core";
import { keyDown } from "@acropolis/input";

class A extends Behavior {
  constructor(entity: Entity) {
    super(entity);
  }

  update() {
    if (keyDown("D")) {
      for (let i = 0; i < 10000; i++) {
        this.transform.position.x += 0.000001;
      }
    }

    if (keyDown("A")) {
      this.transform.position.x -= 0.01;
    }

    if (keyDown("W")) {
      this.transform.position.y += 0.01;
    }

    if (keyDown("S")) {
      this.transform.position.y -= 0.01;
    }
  }
}

export default A;
