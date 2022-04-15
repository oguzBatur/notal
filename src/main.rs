use gl33::global_loader::*;
use gl33::*;
// Local crates.
mod inputs;
mod markdown;
//TODO küçük soyutlamalar ile ilerle.
//TODO girdi ve çıktı mantığını çöz ve bunda istediğini aldıktan sonra opengl'e geç.

// Main is here.
fn main() {
    inputs::take_continous_input();
} // Main ends here.

// Create a shader.
unsafe fn _create_shaders(vertex_shader_source: &str, fragment_shader_source: &str) -> u32 {
    // Vertex Shader..
    let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
    assert_ne!(vertex_shader, 0);
    glShaderSource(
        vertex_shader,
        1,
        &(vertex_shader_source.as_bytes().as_ptr().cast()),
        &(vertex_shader_source.len().try_into().unwrap()),
    );
    glCompileShader(vertex_shader);

    // Fragment Shader..
    let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
    assert_ne!(fragment_shader, 0);
    glShaderSource(
        fragment_shader,
        1,
        &(fragment_shader_source.as_bytes().as_ptr().cast()),
        &(fragment_shader_source.len().try_into().unwrap()),
    );
    glCompileShader(fragment_shader);

    let shader_program = glCreateProgram();
    assert_ne!(shader_program, 0);
    glAttachShader(shader_program, vertex_shader);
    glAttachShader(shader_program, fragment_shader);
    glDeleteShader(vertex_shader);
    glDeleteShader(fragment_shader);
    shader_program
}
