#version 100
precision mediump float;

uniform sampler2D sprite;

varying vec2 v_texcoord;

void main() {
  vec4 samp = texture2D(sprite, v_texcoord);
  gl_FragColor = vec4(vec3(0.0, 0.0, 0.0), samp.a);
}
