#version 410 core

struct VertexData {
	vec4 position;
};

uniform mat4 mvp;
uniform mat4 modelMatrix;

#pragma vertex
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec3 aColor;
layout (location = 3) in vec2 aUV;

out VertexData vData;

@entry void vertex()
{
	vData.position = gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}

#pragma geometry
layout (triangles) in;
layout (line_strip, max_vertices = 6) out;

in VertexData vData[];

@entry void geometry()
{
	gl_Position = mvp * vData[0].position;
	EmitVertex();
	gl_Position = mvp * vData[1].position;
	EmitVertex();
	gl_Position = mvp * vData[2].position;
	EmitVertex();
	gl_Position = mvp * vData[0].position;
	EmitVertex();
	EndPrimitive();
}

#pragma fragment
out vec4 FragColor;

@entry void fragment()
{
	FragColor = vec4(vec3(0.05), 1.0);
}
