#version 100
precision mediump float;

attribute vec2 position;
attribute vec2 texcoord;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;
uniform float depth;

varying vec2 v_texcoord;

void main() {
  v_texcoord = texcoord;
  gl_Position = projection * view * model * vec4(position, -depth, 1.0);
}
