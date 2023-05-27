#include fragment
#include math
#include noise

void main() {
  vec2 uvc = 2.0 * uv - 1.0;
  float a = fCellNoise(1.5 * uv + 33.0, 1337.0, 4, 2.0);
  float r = length(uvc);
  a *= smoothstep(0.0, 1.0, 1.0 - r);
  gl_FragColor = vec4(vec3(a), 1.0);
}