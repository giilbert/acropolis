import { Behavior, Entity } from "@acropolis/core";

class Platform extends Behavior {
  direction: "left" | "right" = "left";

  constructor(entity: Entity) {
    super(entity);
  }

  update() {
    if (this.direction === "left") {
      this.transform.position.x -= 0.1;
    } else {
      this.transform.position.x += 0.1;
    }

    if (this.transform.position.x <= -10) {
      this.direction = "right";
    } else if (this.transform.position.x >= 10) {
      this.direction = "left";
    }
  }
}

export default Platform;
