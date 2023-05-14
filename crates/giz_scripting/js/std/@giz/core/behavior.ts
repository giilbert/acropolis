import { Entity } from ".";
import { Transform } from "./transform";

export class Behavior {
  private entity: Entity;
  protected transform: Transform;

  constructor(entity: Entity) {
    this.entity = entity;
    this.transform = Transform.withEntity(entity);
  }
}
