#version 410 core

#pragma vertex
layout (location = 0) in vec3 aPos;
layout (location = 2) in vec3 aColor;

layout (location = 0) out vec3 color;

uniform mat4 mvp;

@entry void vertex()
{
	color = aColor;
	gl_Position = mvp * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}

#pragma fragment
out vec4 FragColor;

layout (location = 0) in vec3 color;

@entry void fragment()
{
    FragColor = vec4(color, 1.0);
}
