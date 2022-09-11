// maps an entity id to a behavior
const behaviors = {};

const Component = {
  Transform: 0,
};

class Entity {
  constructor(id) {
    this.id = id;
  }
}

class Behavior {
  constructor(entity) {
    this.entity = entity;
    this.transform = new Transform(entity);
  }
}

class Transform {
  constructor(entity) {
    this.entity = entity;
  }

  set position(value) {
    Deno.core.ops.op_set_component_prop(
      this.entity.id,
      Component.Transform,
      "position",
      JSON.stringify(value)
    );
  }

  get position() {
    return JSON.parse(
      Deno.core.ops.op_get_component_prop(
        this.entity.id,
        Component.Transform,
        "position"
      )
    );
  }

  set scale(value) {
    Deno.core.ops.op_set_component_prop(
      this.entity.id,
      Component.Transform,
      "scale",
      JSON.stringify(value)
    );
  }

  get scale() {
    return JSON.parse(
      Deno.core.ops.op_get_component_prop(
        this.entity.id,
        Component.Transform,
        "scale"
      )
    );
  }
}

function runOnce() {
  for (behavior of Object.values(behaviors)) {
    behavior.update();
  }
}

const start = Date.now();

class A extends Behavior {
  constructor(entity) {
    super(entity);
    // this.transform.position = {
    //   x: 0,
    //   y: 0,
    //   z: -10,
    // };
    // this.update();
  }

  update() {
    this.transform.position = {
      x: 0,
      y: 0,
      z: (start - Date.now()) / 500,
    };
  }
}
