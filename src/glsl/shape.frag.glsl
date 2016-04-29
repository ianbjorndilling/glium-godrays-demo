#version 100
precision mediump float;

uniform vec3 color;
uniform float depth;
uniform float depth_max;

void main() {
  gl_FragColor = vec4(color, max(1.0 - (depth / depth_max), 0.0));
}
