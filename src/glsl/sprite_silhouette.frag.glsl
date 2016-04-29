#version 100
precision mediump float;

uniform sampler2D sprite;
uniform vec3 color;
uniform float depth;
uniform float depth_max;

varying vec2 v_texcoord;

void main() {
  vec4 samp = texture2D(sprite, v_texcoord);
  gl_FragColor = vec4(color, samp.a);
}
