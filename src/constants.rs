/* Constants */
pub const SCREEN_WIDTH: f32 = 640f32 / 2f32;
pub const SCREEN_HEIGHT: f32 = 480f32 / 2f32;
pub const VERSION_NAME: &str = "Sound fix";


/* Audio*/
pub const AUDIO_MUSIC: &[u8; 2764117] = include_bytes!("../assets/Noster_MF_SC1.mp3");
pub const AUDIO_SOUND: &[u8; 6751] = include_bytes!("../assets/enemyExplosion.mp3");

/* Textures */
pub const TEXTURE_TEXEL_CHECKER: &[u8; 57153] = include_bytes!("../assets/texel_checker.png");
pub const TEXTURE_GROUND: &[u8; 119400] = include_bytes!("../assets/ground.png");
pub const TEXTURE_TREE_LEFT: &[u8; 16708] = include_bytes!("../assets/tree_left.png");
pub const TEXTURE_TREE_RIGHT: &[u8; 15178] = include_bytes!("../assets/tree_right.png");

/* Shaders */
#[cfg(not(target_arch = "wasm32"))]
pub const GLSL_VERSION: i32 = 330;
#[cfg(target_arch = "wasm32")]
pub const GLSL_VERSION: i32 = 100;
// GLSL 330
pub const FRACTAL_SHADER_GLSL330: &str = include_str!("../shaders/glsl330/fog.fs");
pub const VERTEX_SHADER_GLSL330: &str = include_str!("../shaders/glsl330/base_lighting.vs");
// GLSL 100
pub const FRACTAL_SHADER_GLSL100: &str = include_str!("../shaders/glsl100/fog.fs");
pub const VERTEX_SHADER_GLSL100: &str = include_str!("../shaders/glsl100/base_lighting.vs");
