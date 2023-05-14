import { Entity } from "..";
import { setComponentProperty } from "../deno";

export class Vector3 {
  private _x: number;
  private _y: number;
  private _z: number;

  private component: number | undefined;
  private entity: Entity | undefined;
  private property: string | undefined;

  constructor(x: number, y: number, z: number) {
    this._x = x;
    this._y = y;
    this._z = z;
  }

  static withEntity(
    x: number,
    y: number,
    z: number,
    component: number,
    entity: Entity,
    property: string
  ): Vector3 {
    const newVector = new Vector3(x, y, z);
    newVector.entity = entity;
    newVector.property = property;
    newVector.component = component;
    return newVector;
  }

  set x(x: number) {
    this._x = x;
    if (this.entity) this.updateEntityPosition();
  }

  get x(): number {
    return this._x;
  }

  set y(y: number) {
    this._y = y;
    if (this.entity) this.updateEntityPosition();
  }

  get y(): number {
    return this._y;
  }

  set z(z: number) {
    this._z = z;
    if (this.entity) this.updateEntityPosition();
  }

  get z(): number {
    return this._z;
  }

  private updateEntityPosition() {
    setComponentProperty(this.entity!, this.component!, this.property!, {
      x: this.x,
      y: this.y,
      z: this.z,
    });
  }
}
