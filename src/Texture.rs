use beryllium::*;
use std::path::PathBuf;
use std::fs;
use ogl33::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Texture
{
	ID: u32
}
impl Texture
{
	pub fn new(path: &str) -> Self
	{
		let bitmap = {
			let mut f = std::fs::File::open(path).unwrap();
			let mut bytes = vec![];
			std::io::Read::read_to_end(&mut f, &mut bytes).unwrap();
			let mut bitmap = imagine::png::parse_png_rgba8(&bytes).unwrap().bitmap;
			bitmap.flip_scanlines();
			bitmap
		};


		let mut id = 0;
		unsafe 
		{
			glGenTextures(1, &mut id);
			glActiveTexture(GL_TEXTURE0);
			glBindTexture(GL_TEXTURE_2D, id);
			glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
		    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
		    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLint);
		    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);
		    glTexImage2D(
		    	GL_TEXTURE_2D,
		    	0,
		    	GL_RGBA as GLint,
		    	bitmap.width().try_into().unwrap(),
		    	bitmap.height().try_into().unwrap(),
		    	0,
		    	GL_RGBA,
		    	GL_UNSIGNED_BYTE,
		    	bitmap.pixels().as_ptr().cast()
		    );
		    glGenerateMipmap(GL_TEXTURE_2D);

		    
		}

		Self
		{
			ID: id
		}
	}

	pub fn activate(&self)
	{
		unsafe
		{
			glActiveTexture(GL_TEXTURE0);
			glBindTexture(GL_TEXTURE_2D, self.ID);
		}
	}
}
