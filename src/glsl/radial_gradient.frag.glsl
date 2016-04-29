#version 100
precision mediump float;

uniform vec3 color_from;

varying vec2 v_texcoord;

void main() {
  float dist = length(vec2(0.5, 0.5) - v_texcoord);
  gl_FragColor = vec4(color_from, max(mix(0.5, 0.0, dist * 3.0), 0.0));
}
