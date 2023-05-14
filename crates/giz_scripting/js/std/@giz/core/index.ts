export class Entity {
  id: string;

  constructor(id: string) {
    this.id = id;
  }
}

export { Transform } from "./transform";
export { Vector3 } from "./math/vector3";
export { Behavior } from "./behavior";
