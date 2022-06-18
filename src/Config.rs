use crate::Texture;
use crate::ShaderProgram;

pub struct Config
{
    TEXTURES: Vec<Texture::Texture>,
    SHADERS: Vec<ShaderProgram::ShaderProgram>
}

impl Config 
{
    pub fn new() -> Self
    {
        Self
        {
            TEXTURES: Vec::new(),
            SHADERS: Vec::new()
        }
    }    

    pub fn add_texture(&mut self, tex: Texture::Texture)
    {
        self.TEXTURES.push(tex);
    }

    pub fn add_shader(&mut self, shader: ShaderProgram::ShaderProgram)
    {
        self.SHADERS.push(shader);
    }

    pub fn get_shader(&self, index: usize) -> ShaderProgram::ShaderProgram
    {
        return self.SHADERS[index];
    }

    pub fn get_texture(&self, index: usize) -> Texture::Texture
    {
        return self.TEXTURES[index];
    }

}