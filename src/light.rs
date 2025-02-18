use raylib::prelude::*;

const MAX_LIGHTS: u32 = 4;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LightType {
    LightDirectional = 0,
    LightPoint = 1,
}

impl Default for LightType {
    fn default() -> Self {
        Self::LightDirectional
    }
}

#[derive(Debug, Default, Clone)]
pub struct Light {
    pub enabled: bool,
    pub light_type: LightType,
    pub position: Vector3,
    pub target: Vector3,
    pub color: Color,
    pub enabled_loc: i32,
    pub type_loc: i32,
    pub pos_loc: i32,
    pub target_loc: i32,
    pub color_loc: i32,
}

static mut LIGHTS_COUNT: i32 = 0;

// Defines a light and get locations from PBR shader
pub fn create_light(
    light_type: LightType,
    pos: Vector3,
    targ: Vector3,
    color: Color,
    shader: &mut Shader,
) -> Light {
    let mut light = Light::default();

    if (unsafe { LIGHTS_COUNT as u32 } < MAX_LIGHTS) {
        light.enabled = true;
        light.light_type = light_type;
        light.position = pos.clone();
        light.target = targ.clone();
        light.color = color.clone();

        let lights_count = unsafe { LIGHTS_COUNT };
        let enabled_name = format!("lights[{}].enabled", lights_count);
        let type_name = format!("lights[{}].type", lights_count);
        let pos_name = format!("lights[{}].position", lights_count);
        let target_name = format!("lights[{}].target", lights_count);
        let color_name = format!("lights[{}].color", lights_count);

        // Set location name [x] depending on lights count
        light.enabled_loc = shader.get_shader_location(&enabled_name);
        light.type_loc = shader.get_shader_location(&type_name);
        light.pos_loc = shader.get_shader_location(&pos_name);
        light.target_loc = shader.get_shader_location(&target_name);
        light.color_loc = shader.get_shader_location(&color_name);

        update_light_values(shader, light.clone());
        unsafe {
            LIGHTS_COUNT += 1;
        }
    }

    return light;
}

pub fn update_light_values(shader: &mut Shader, light: Light) {
    // Send to shader light enabled state and type
    shader.set_shader_value(light.enabled_loc, light.enabled as i32);
    shader.set_shader_value(light.type_loc, light.light_type as i32);

    // Send to shader light position values
    shader.set_shader_value(light.pos_loc, light.position);

    // Send to shader light target position values
    shader.set_shader_value(light.target_loc, light.target);

    // Send to shader light color values
    let color: Vector4 = light.color.into();
    shader.set_shader_value(light.color_loc, color);
}
