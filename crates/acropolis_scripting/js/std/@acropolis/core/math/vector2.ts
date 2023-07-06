import { Entity } from "..";

// TODO: make it work with vec2s
export class Vector2 {
  private _x: number;
  private _y: number;

  private component: number | undefined;
  private entity: Entity | undefined;
  private property: number | undefined;

  constructor(x: number, y: number) {
    this._x = x;
    this._y = y;
  }

  static withEntity(
    x: number,
    y: number,
    component: number,
    entity: Entity,
    property: number
  ): Vector2 {
    const newVector = new Vector2(x, y);
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
    const { x, y } = getComponentVector2Property(
      this.entity!,
      this.component!,
      this.property!
    );
    this._x = x;
    this._y = y;
    return x;
  }

  set y(y: number) {
    this._y = y;
    if (this.entity) this.updateEntityPosition();
  }

  get y(): number {
    if (!this.entity) return this._y;
    const { x, y } = getComponentVector2Property(
      this.entity!,
      this.component!,
      this.property!
    );
    this._x = x;
    this._y = y;
    return y;
  }

  private updateEntityPosition() {
    // setComponentVector2Property(
    //   this.entity!,
    //   this.component!,
    //   this.property!,
    //   this._x,
    //   this._y
    // );
  }
}
