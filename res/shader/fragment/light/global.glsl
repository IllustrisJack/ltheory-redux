#include fragment
#include color
#include deferred
#include gamma
#include math

#autovar samplerCube irMap
#autovar samplerCube envMap
#autovar vec3 eye

in vec3 worldOrigin;
in vec3 worldDir;

uniform vec3 lightColor;
uniform vec3 lightPos;

uniform sampler2D texNormalMat;
uniform sampler2D texDepth;

float roughnessToLOD (float r) {
  return 8.0 * (pow(2.0, r) - 1.0);
}

void main () {
  vec4 normalMat = texture(texNormalMat, uv);
  float depth = texture(texDepth, uv).x;
  vec3 N = decodeNormal(normalMat.xy);
  float rough = normalMat.z;
  float mat = normalMat.w;
  vec3 pos = worldOrigin + depth * normalize(worldDir);
  vec3 V = normalize(pos - eye);
  vec3 R = normalize(reflect(V, N));

  vec3 light = vec3(0.0);

  if (mat == Material_Diffuse) {
    light += linear(textureLod(irMap, N, 8.0).xyz);
  }

  else if (mat == Material_Metal) {
    #ifdef HIGHQ
      light += linear(textureLod(irMap, R, roughnessToLOD(rough)).xyz);
    #else
      light += linear(texture(envMap, R).xyz);
    #endif
  }

  else if (mat == Material_NoShade) {
    light += vec3(1.0);
  }

  outColor = vec4(light, 1.0);
}
