#version 410 core

struct VertexData {
	vec3 normal;
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
	vData.normal = vec3(aNormal.xy, aNormal.z);
	gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}

#pragma geometry
layout (triangles) in;
layout (line_strip, max_vertices = 6) out;

in VertexData vData[];
out VertexData fData;

@entry void geometry()
{
	float normal_length = 0.08;

	for (int i = 0; i < gl_in.length(); i++) {
		vec3 p = gl_in[i].gl_Position.xyz;
		vec3 n = vData[i].normal;

		fData.normal = vData[i].normal;
		gl_Position = mvp * vec4(p, 1.0);
		EmitVertex();

		fData.normal = vData[i].normal;
		gl_Position = mvp * vec4(p + n * normal_length, 1.0);
		EmitVertex();
		EndPrimitive();
	}
}

#pragma fragment
in VertexData fData;

out vec4 FragColor;

@entry void fragment()
{
	FragColor = vec4(fData.normal.xy, 1.0, 1.0);
}
