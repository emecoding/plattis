use ogl33::*;

use core::{
    convert::{TryFrom, TryInto},
    mem::{size_of, size_of_val},
  };

use crate::{ShaderProgram, Shader};
use crate::Texture;

pub struct SpriteRenderer
{
    quadVAO: u32
}

impl SpriteRenderer
{

    pub fn new() -> Self
    {
        //INIT RENDER DATA
        let mut QUADVAO: u32 = 0;
        let mut VBO: u32 = 0;
        type Vertex = [f32; 2 + 2]; //x, y, texX, texY
        let vertices: [Vertex; 6] = [
            [0.0, 1.0, 0.0, 1.0], 
            [1.0, 0.0, 1.0, 0.0], 
            [0.0, 0.0, 0.0, 0.0],

            [0.0, 1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 0.0, 1.0, 0.0]
        ];

        unsafe
        {
            glGenVertexArrays(1, &mut QUADVAO);
            glGenBuffers(1, &mut VBO);
                
            glBindBuffer(GL_ARRAY_BUFFER, VBO);
            glBufferData(
                GL_ARRAY_BUFFER, 
                vertices.len().try_into().unwrap(),
                vertices.as_ptr().cast(),
                GL_STATIC_DRAW
            );

            glBindVertexArray(QUADVAO);
            glEnableVertexAttribArray(0);
            glVertexAttribPointer(0, 
                4, 
                GL_FLOAT, 
                GL_FALSE, 
                size_of::<Vertex>().try_into().unwrap(),
                0 as *const _
            );

            glBindBuffer(GL_ARRAY_BUFFER, 0);
            glBindVertexArray(0);


        }
        
        Self
        {
            quadVAO: QUADVAO
        }

    }


    pub fn render_sprite(&self, shader: &ShaderProgram::ShaderProgram, texture: &Texture::Texture, position: &ultraviolet::Vec2, size: f32, rotation: f32, color: ultraviolet::Vec3)
    {
        unsafe
        {
            shader.use_program();

            let mut model: ultraviolet::Mat4 = ultraviolet::Mat4::from_translation(ultraviolet::Vec3::new(position[0], position[1], 0.0));
                //* ultraviolet::Mat4::from_translation(ultraviolet::Vec3::new(0.5 * position[0], 0.5 * position[1], 0.0));
            model = model * ultraviolet::Mat4::from_scale(size);


            shader.upload_mat4_uniform("model", model);
            shader.upload_vec3_uniform("spriteColor", color);

            texture.activate();

            glBindVertexArray(self.quadVAO);
            glDrawArrays(GL_TRIANGLES, 0, 6);
            glBindVertexArray(0);
        }
        

    }   
}

