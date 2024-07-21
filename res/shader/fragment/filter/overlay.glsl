in vec2 uv;
out vec4 outColor;

uniform sampler2D fg;
uniform sampler2D bg;
uniform float fgMult;
uniform float bgMult;

void main() {
  vec3 c1 = fgMult * texture(fg, uv).xyz;
  vec3 c2 = bgMult * texture(bg, uv).xyz;
  c2 = pow(c2, vec3(1.4, 1.2, 1.2));
  float avg = (c1.x + c1.y + c1.z) / 3.0;
  c2 *= exp(-8.0 * pow(avg, 0.75));
  vec3 c = c1 + c2;
  outColor = vec4(c, 1.0);
}
