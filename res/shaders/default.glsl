#version 410 core

#pragma vertex
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec3 aColor;
layout (location = 3) in vec2 aUV;

uniform mat4 mvp;

@entry void vertex()
{
	gl_Position = mvp * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}

#pragma fragment
out vec4 FragColor;

float rand(float n) {
	return fract(sin(n) * 43758.5453123);
}

float noise(float p){
	float fl = floor(p);
	float fc = fract(p);
	return mix(rand(fl), rand(fl + 1.0), fc);
}

@entry void fragment()
{
	float n = noise(gl_PrimitiveID);
	n = pow(n, 1.5) / 1.05;

	n = mix(0.1, 1.0, n);

    //FragColor = vec4(noise(gl_PrimitiveID), noise(gl_PrimitiveID + 1290), noise(gl_PrimitiveID + 215), 1.0f);
    FragColor = vec4(n, n, n, 1.0);
}
