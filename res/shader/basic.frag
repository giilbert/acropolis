#version 330
precision mediump float;
in vec3 vNormal;
uniform float time;
float bias = -0.3;
float PI = 3.1415926535;
vec3 hsb2rgb( in vec3 c ){
  vec3 rgb = clamp(abs(mod(c.x * 6.0 + vec3(0.0, 4.0, 2.0), 6.0) - 3.0) - 1.0, 0.0, 1.0 );
  rgb = rgb * rgb * (3.0 - 2.0 * rgb);
  return c.z * mix(vec3(1.0), rgb, c.y);
}
void main()
{
  vec3 normal = normalize(vNormal);
  float light = dot(normal, vec3(0, 1, 0.5));
  light += 1.0 / (light) - 0.2;
  light = clamp(light, 0.3, 1.0);
  gl_FragColor = vec4(hsb2rgb(vec3(time, 0.7, 0.66)), 1.0);
  gl_FragColor.rgb *= light;
};