import { Behavior, Entity } from "@acropolis/core";
import { keyDown } from "@acropolis/input";

class Move extends Behavior {
  lastPressed: number;

  constructor(entity: Entity) {
    super(entity);
    this.lastPressed = Date.now();
  }

  update() {
    const cooldownOver = Date.now() - this.lastPressed > 250;

    if (!cooldownOver) return;

    if (keyDown("D")) this.transform.position.x += 1;
    if (keyDown("A")) this.transform.position.x -= 1;
    if (keyDown("W")) this.transform.position.y += 1;
    if (keyDown("S")) this.transform.position.y -= 1;

    this.lastPressed = Date.now();
  }
}

export default Move;
