import { Entity } from "..";
import {
  getComponentVector3Property,
  setComponentVector3Property,
} from "../deno";

export class Vector3 {
  private _x: number;
  private _y: number;
  private _z: number;

  private component: number | undefined;
  private entity: Entity | undefined;
  private property: number | undefined;

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
    property: number
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
    if (!this.entity) return this._x;
    const { x } = getComponentVector3Property(
      this.entity!,
      this.component!,
      this.property!
    );
    this._x = x;
    return x;
  }

  set y(y: number) {
    this._y = y;
    if (this.entity) this.updateEntityPosition();
  }

  get y(): number {
    if (!this.entity) return this._y;
    const { y } = getComponentVector3Property(
      this.entity!,
      this.component!,
      this.property!
    );
    this._y = y;
    return y;
  }

  set z(z: number) {
    this._z = z;
    if (this.entity) this.updateEntityPosition();
  }

  get z(): number {
    if (!this.entity) return this._z;
    const { z } = getComponentVector3Property(
      this.entity!,
      this.component!,
      this.property!
    );
    this._z = z;
    return z;
  }

  private updateEntityPosition() {
    setComponentVector3Property(
      this.entity!,
      this.component!,
      this.property!,
      this._x,
      this._y,
      this._z
    );
  }
}
