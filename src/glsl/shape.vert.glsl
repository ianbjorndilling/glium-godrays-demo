#version 100
precision mediump float;

attribute vec2 position;

uniform float bottom_offset;
uniform float depth;

void main() {
  gl_Position = vec4(position, 0.0, 1.0);
  gl_Position.y += bottom_offset;
}
