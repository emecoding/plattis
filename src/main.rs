use ogl33::*;
use ultraviolet::*;
use beryllium::*;
use core::{
  convert::{TryFrom, TryInto},
  mem::{size_of, size_of_val},
};

mod VertexArray;
mod ShaderProgram;
mod Shader;
mod SpriteRenderer;
mod Camera;
mod Config;
mod Texture;

fn initialize_sdl() -> SDL
{
    let sdl = SDL::init(InitFlags::Everything).expect("Failed to create a window...");
    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();

    #[cfg(target_os = "macos")]
    {
    sdl
        .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
        .unwrap();
    }

    sdl.set_relative_mouse_mode(true);

    return sdl;
}

fn create_window(sdl: &SDL, title: &str, width: u32, height: u32) -> GlWindow
{
    let window = sdl.create_gl_window(title, WindowPosition::Centered, width, height, WindowFlags::Shown).expect("Failed to create a window...");

    return window;
}

fn main() {
    let sdl = initialize_sdl();
    let WINDOW: GlWindow = create_window(&sdl, "Plattis", 800, 600);
    let CAMERA: Camera::Camera = Camera::Camera::new(800, 600);
    

    unsafe
    {
        load_gl_with(|f_name| WINDOW.get_proc_address(f_name));
        //glClearColor(1.0, 1.0, 1.0, 1.0);
    }

    let SPRITE_RENDERER: SpriteRenderer::SpriteRenderer = SpriteRenderer::SpriteRenderer::new();
    let mut CONFIG = Config::Config::new();

    CONFIG.add_texture(Texture::Texture::new("res/sprites/tree.png"));
    
    let projection: ultraviolet::Mat4 = CAMERA.get_orthographic_projection();
    let SpriteShader = ShaderProgram::ShaderProgram::from_vert_frag("res/shaders/sprite.vs", "res/shaders/sprite.fs").unwrap();

    SpriteShader.use_program();

    SpriteShader.upload_mat4_uniform("projection", projection);
    CONFIG.add_shader(SpriteShader);


    game_loop(&sdl, &WINDOW, &CAMERA, &SPRITE_RENDERER, &CONFIG);

}

fn game_loop(sdl: &SDL, WINDOW: &GlWindow, CAMERA: &Camera::Camera, SPRITE_RENDERER: &SpriteRenderer::SpriteRenderer, CONFIG: &Config::Config)
{
    'main_loop: loop
    {   
        let should_close = handle_input(&sdl);
        if should_close {break 'main_loop; }

        render(&CAMERA, &WINDOW, &SPRITE_RENDERER, &CONFIG);
    }
        
}

fn handle_input(sdl: &SDL) -> bool
{

    let mut should_close: bool = false;

    while let Some(event) = sdl.poll_events().and_then(Result::ok)
    {
        match event
        {
            Event::Quit(_) => {
                should_close = true;
            },
            Event::Keyboard(KeyboardEvent {
                is_pressed,
                key: KeyInfo {keycode, ..},
                ..
            }) => {
                if keycode == beryllium::Keycode(27) { should_close = true; }//ESCAPE
            }

            _ => (),
        }
    }

    return should_close;


}

fn render(CAMERA: &Camera::Camera, WINDOW: &GlWindow, SPRITE_RENDERER: &SpriteRenderer::SpriteRenderer, CONFIG: &Config::Config)
{
    let shader = CONFIG.get_shader(0);
    let texture = CONFIG.get_texture(0);

    unsafe
    {
        glClearColor(0.2, 0.3, 0.7, 1.0);
        glClear(GL_COLOR_BUFFER_BIT);
    }

    
    SPRITE_RENDERER.render_sprite(&shader, &texture, &Vec2::new(200.0, 0.0), 100.0, 0.0, Vec3::new(1.0, 0.0, 0.0));

    

    WINDOW.swap_window();
}