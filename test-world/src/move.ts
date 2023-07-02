import { Behavior, Entity } from "@acropolis/core";
import { keyDown } from "@acropolis/input";

const speed = 0.1;

class Move extends Behavior {
  constructor(entity: Entity) {
    super(entity);
  }

  update() {
    if (keyDown("D")) this.transform.position.x += speed;
    if (keyDown("A")) this.transform.position.x -= speed;
    if (keyDown("W")) this.transform.position.y += speed;
    if (keyDown("S")) this.transform.position.y -= speed;
  }
}

export default Move;
