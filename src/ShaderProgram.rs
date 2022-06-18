use ogl33::*;
use std::fs;

use crate::Shader::*;
use ultraviolet::*;
use std::ffi::{CString};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShaderProgram(pub GLuint);
impl ShaderProgram
{
	pub fn new() -> Option<Self>
	{
		let prog = unsafe { glCreateProgram() };
		if prog != 0
		{
			Some(Self(prog))
		}else 
		{
			None
		}
	}

	pub fn attach_shader(&self, shader: &Shader)
	{
		unsafe { glAttachShader(self.0, shader.0) };
	}

	pub fn link_program(&self)
	{
		unsafe { glLinkProgram(self.0) };
	}

	pub fn link_success(&self) -> bool 
	{
		let mut success = 0;
		unsafe { glGetProgramiv(self.0, GL_LINK_STATUS, &mut success) };
		success == i32::from(GL_TRUE)
	}

	pub fn info_log(&self) -> String
	{
		let mut needed_len = 0;
		unsafe { glGetProgramiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len) };
		let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
		let mut len_written = 0_i32;
		unsafe 
		{
			glGetProgramInfoLog(
				self.0,
				v.capacity().try_into().unwrap(),
				&mut len_written,
				v.as_mut_ptr().cast()
			);

			v.set_len(len_written.try_into().unwrap());
		}

		String::from_utf8_lossy(&v).into_owned()
	}

	pub fn use_program(&self)
	{
		unsafe { glUseProgram(self.0) };
	}

	pub fn delete(&self)
	{
		unsafe { glDeleteProgram(self.0) };
	}


	pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String>
	{
		let p = Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;
	    let v = Shader::from_source(ShaderType::Vertex, &read_file(vert)).map_err(|e| format!("Vertex Compile Error: {}", e))?;
	    let f = Shader::from_source(ShaderType::Fragment, &read_file(frag)).map_err(|e| format!("Fragment Compile Error: {}", e))?;
	    p.attach_shader(&v);
	    p.attach_shader(&f);
		p.link_program();
		v.delete();
		f.delete();

		if p.link_success()
		{
			Ok(p)
		}else
		{
			let out = format!("Program link error: {}", p.info_log());
			p.delete();
			Err(out)
		}
	}

	pub fn upload_mat4_uniform(&self, name: &str, data: Mat4)
	{
		unsafe
		{	
			//println!("{:.32}", data);
			let c_name = CString::new(name).expect("Failed to convert to c string");
			let loc: i32 = glGetUniformLocation(self.0, c_name.as_ptr());
			glUniformMatrix4fv(loc, 1, GL_FALSE, data.as_ptr());
		}
		
	}

	pub fn upload_vec3_uniform(&self, name: &str, data: Vec3)
	{
		unsafe
		{	
			//println!("{:?}", data);
			let c_name = CString::new(name).expect("Failed to convert to c string");
			let loc: i32 = glGetUniformLocation(self.0, c_name.as_ptr());
			glUniform3f(loc, data[0], data[1], data[2]);
		}
		
	}

}

fn read_file(path: &str) -> String
{
	let content = fs::read_to_string(path).expect("Failed to open file");
	return content;
}