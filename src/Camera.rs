pub struct Camera
{
    window_width: u32,
    window_height: u32
}

impl Camera
{
    pub fn new(width: u32, height: u32) -> Self
    {
        Self
        {
            window_width: width,
            window_height: height
        }
    
    }

    pub fn get_orthographic_projection(&self) -> ultraviolet::Mat4
    {
        println!("{}, {}", self.window_width, self.window_height);
        return ultraviolet::projection::rh_yup::orthographic_gl(0.0, self.window_width as f32, self.window_height as f32, 0.0, -1.0, 1.0);
    }
}