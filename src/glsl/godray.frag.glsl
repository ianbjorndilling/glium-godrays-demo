#version 100
precision mediump float;

uniform sampler2D occlusion;
uniform float exposure;
uniform float decay;
uniform float density;
uniform float weight;

uniform vec2 light_position;

varying vec2 v_texcoord;

const int NUM_SAMPLES = 100;

void main() {
  vec2 texcoord = v_texcoord;
  vec2 delta = texcoord - light_position;
  delta *= 1.0 / float(NUM_SAMPLES) * density;

  float illumination_decay = 1.0;

  //gl_FragColor = texture2D(occlusion, texcoord);
  vec4 color = vec4(0.0);

  for (int i = 0; i < NUM_SAMPLES; i++) {
    texcoord -= delta;
    vec4 samp = texture2D(occlusion, texcoord);
    samp *= illumination_decay * weight;
    color += samp;
    //gl_FragColor += samp;
    illumination_decay *= decay;
  }

  //gl_FragColor *= exposure;
  gl_FragColor = color * exposure;

}

