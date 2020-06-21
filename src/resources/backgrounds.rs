
pub struct BackgroundsMeta {
    pub data: [(&'static str, f32); 9],
}

impl BackgroundsMeta {
    pub fn new() -> Self {
        Self {
            data: get_backgrounds_meta(),
        }
    }
}

pub fn get_backgrounds_meta() -> [(&'static str, f32); 9]  {
    let backgrounds_map = [
        ("z-10_1.png", -10.0f32),
        ("z-10_2.png", -10.0f32),
        ("z-15.png", -15.0f32),
        ("z-20_1.png", -20.0f32),
        ("z-20_2.png", -20.0f32),
        ("z-20_3.png", -20.0f32),
        ("z-30.png", -30.0f32),
        ("z+0.png", -1.0f32),
        ("z+1.png", 1.0f32),
    ];

    return backgrounds_map;
}