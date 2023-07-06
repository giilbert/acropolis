export class Entity {
  id: number;

  constructor(id: number) {
    this.id = id;
  }

  get<T>(ComponentClass: any): T {
    // TODO: make sure that the component is actually attached to the entity
    return ComponentClass.withEntity(this);
  }
}

export { Transform } from "./transform";
export { Vector2 } from "./math/vector2";
export { Vector3 } from "./math/vector3";
export { Behavior } from "./behavior";
