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

class Input {
  static keyDown(key) {
    return Deno.core.ops.op_get_key_down(`"${key}"`);
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
    if (Input.keyDown("D")) {
      this.transform.position = {
        x: (this.transform.position.x += 0.01),
        y: this.transform.position.y,
        z: 0,
      };
    }

    if (Input.keyDown("A")) {
      this.transform.position = {
        x: (this.transform.position.x -= 0.01),
        y: this.transform.position.y,
        z: 0,
      };
    }

    if (Input.keyDown("W")) {
      this.transform.position = {
        x: this.transform.position.x,
        y: (this.transform.position.y += 0.01),
        z: 0,
      };
    }

    if (Input.keyDown("S")) {
      this.transform.position = {
        x: this.transform.position.x,
        y: (this.transform.position.y -= 0.01),
        z: 0,
      };
    }
  }
}
