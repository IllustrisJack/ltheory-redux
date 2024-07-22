#include fragment

uniform sampler2D src;

void main() {
  vec4 c = texture(src, uv);
  c.xyz = max(vec3(0.0), vec3(1.0) - c.xyz);
  outColor = c;
}
