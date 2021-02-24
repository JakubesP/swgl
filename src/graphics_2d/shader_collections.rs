
pub const COLOR_VERTEX2D_SHADER: &'static str = "
#shader vertex
# version 300 es

layout (location=0) in vec2 pos;
layout (location=1) in vec4 color;
layout (location=2) in float z_index;

uniform mat4 model;
uniform mat4 projection;

out vec4 Col;

void main() {
    Col = color;
    gl_Position = projection * model * vec4(pos.x, pos.y, 0.0 + z_index, 1.0);
    gl_PointSize = 2.0;
}

#shader fragment
# version 300 es
precision mediump float;

in vec4 Col;

out vec4 FragColor;

void main() {
    FragColor = Col;
}
";

pub const FULL_VERTEX2D_SHADER: &str = "
#shader vertex
# version 300 es

layout (location=0) in vec2 pos;
layout (location=1) in vec4 color;
layout (location=2) in vec2 texture_coords;
layout (location=3) in float array_index;
layout (location=4) in float z_index;
layout (location=5) in float texture_factor;

uniform mat4 model;
uniform mat4 projection;

out vec2 TexCoord;
out vec4 Col;
out float ArrayIndex;
out float TextureFactor;

void main() {
    TexCoord = texture_coords;
    Col = color;
    ArrayIndex = array_index;
    TextureFactor = texture_factor;
    gl_Position = projection * model * vec4(pos.x, pos.y, 0.0 + z_index, 1.0);
}

#shader fragment
# version 300 es
precision mediump float;
precision highp sampler2DArray;

in vec2 TexCoord;
in float ArrayIndex;
in vec4 Col;
in float TextureFactor;

uniform sampler2DArray tex_0;

out vec4 FragColor;

void main() {
    FragColor = texture(tex_0, vec3(TexCoord * TextureFactor, ArrayIndex)) + Col;
}
";

pub const SIMPLE_TEX_VERTEX2D_SHADER: &str = "
#shader vertex
# version 300 es

layout (location=0) in vec2 pos;
layout (location=1) in vec2 texture_coords;
layout (location=2) in float z_index;
layout (location=3) in float texture_factor;

uniform mat4 model;
uniform mat4 projection;

out vec2 TexCoord;
out float TextureFactor;

void main() {
    TexCoord = texture_coords;
    TextureFactor = texture_factor;
    gl_Position = projection * model * vec4(pos.x, pos.y, 0.0 + z_index, 1.0);
}

#shader fragment
# version 300 es
precision mediump float;
precision highp sampler2DArray;

in vec2 TexCoord;
in float TextureFactor;

uniform sampler2D tex_0;

out vec4 FragColor;

void main() {
    FragColor = texture(tex_0, TexCoord * TextureFactor);
}
";

