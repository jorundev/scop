#version 410 core

struct VertexOut {
	vec4 position;
	vec3 normal;
	vec2 uv;
};

uniform mat4 mvp;
uniform mat4 modelMatrix;
uniform sampler2D diffuseTex;
uniform float mixFactor;
uniform float lightFactor;

#pragma vertex
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec3 aColor;
layout (location = 3) in vec2 aUV;

out VertexOut vData;

@entry void vertex()
{
	mat3 normalMatrix = mat3(transpose(inverse(modelMatrix)));

	vData.normal = normalMatrix * aNormal;
	vData.position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
	vData.uv = aUV;
	gl_Position = mvp * vData.position;
}

#pragma fragment
in VertexOut vData;

out vec4 FragColor;

vec3 phong(vec3 lightColor, vec3 lightDir, vec3 ambient, vec3 objectColor) {
	float diff = max(dot(vData.normal, lightDir), 0.0);
	vec3 diffuse = diff * lightColor * objectColor;
	return(ambient + diffuse);
}

vec3 normalColor(vec3 normal) {
	return (vec3(1.0) + normal) * 0.5;
}

float rand(float n) {
	return fract(sin(n) * 43758.5453123);
}

float noise(float p) {
	float fl = floor(p);
	float fc = fract(p);
	return mix(rand(fl), rand(fl + 1.0), fc);
}

@entry void fragment()
{
	vec3 lightDir = vec3(1.0, 0.0, 0.0);
	vec3 lightColor = vec3(1.0, 1.0, 1.0);
	float ambientStength = 0.2;
	vec3 ambient = ambientStength * lightColor;

	float n = noise(gl_PrimitiveID);
	vec4 faceColor = vec4(n, n, n, 1.0);

	vec4 texColor = texture(diffuseTex, vData.uv);
	vec4 textureFaceMix = mix(faceColor, texColor, mixFactor);
	vec4 lightPure = vec4(phong(lightColor, lightDir, ambient, vec3(1.0)), 1.0);
	vec4 light = mix(vec4(1.0), lightPure, lightFactor);

    FragColor = textureFaceMix * light;
}
