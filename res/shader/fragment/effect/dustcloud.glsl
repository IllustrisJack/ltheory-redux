#include fragment
#include gamma
#include math
#include noise
#include color

#autovar samplerCube irMap
uniform sampler2D texDust;
uniform float alphaScale;

void main() {
  vec3 V = pos - eye;
  float dist = length(V) / 1024.0;
  vec4 bg = textureCubeLod(irMap, V, 2.0);
  vec3 c = mix(vec3(0.2), mix(bg.xyz, 0.75 * sqrt(bg.xyz), 0.25), 0.8);
  float a = texture2D(texDust, 0.5 + 0.5 * uv).x;
  a *= smoothstep(0.0, 0.4, (1.0 - dist) / 0.25);
  a *= smoothstep(0.0, 0.4, dist / 0.25);
  a *= alphaScale;
  a = clamp(a, 0.0, 1.0);
  a *= a;
  gl_FragColor = vec4(linear(c), a);
  FRAGMENT_CORRECT_DEPTH;
}
