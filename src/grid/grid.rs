use mlua::{Lua, Result, Table, UserData, UserDataMethods, FromLua};
use image::{Rgba};

pub static DIMENSION: usize = 32;


#[derive(Clone, serde::Serialize)]
pub struct Frame {
    pub id: usize,
    pub data: Vec<[u8; 4]>,
}


#[derive(Clone, serde::Serialize)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub current_frame: usize,
    pub frames: Vec<Frame>,
}

impl UserData for Grid {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {

        methods.add_method_mut("set_pixel", |_, this, (x, y, r, g, b): (i64, i64, u8, u8, u8)| {
            let base_frame_data = &mut this.frames[this.current_frame].data;
            let idx = (y * (DIMENSION as i64) + x) as usize;
            if idx < (DIMENSION*DIMENSION) {
                base_frame_data[idx] = [r, g, b, 255];
            }

            Ok(())
        });

        methods.add_method_mut("set_area", |_, this: &mut Grid, (top, left, width, height, r, g, b) : (i64, i64, i64, i64, u8, u8, u8)| {
            let base_frame_data = &mut this.frames[this.current_frame].data;

            for x in left..(left + width) {
                for y in top..(top + height) {
                    let idx = (y * (DIMENSION as i64) + x) as usize;
                    if idx < (DIMENSION*DIMENSION) { // TODO: replace for actual size, and use different indexers
                        base_frame_data[idx] = [r, g, b, 255];
                    }
                }
            }

            Ok(())
        });

        methods.add_method_mut("create_frame", |_, this, ()| {
            let new_frame = Frame {
                id: this.frames.len() + 1,
                data: vec![[0; 4]; DIMENSION*DIMENSION]
            };

            this.frames.push(new_frame);

            Ok(())
        });

        methods.add_method_mut("switch_frame", |_, this, frame_to_switch: usize | {
            if frame_to_switch > (this.frames.len()-1) {
                return Ok(());
            }    

            this.current_frame = frame_to_switch;

            Ok(())
        });
    }
}

impl FromLua for Grid {
    fn from_lua(lua_value: mlua::Value, _: &Lua) -> Result<Self> {
        match lua_value {
            mlua::Value::UserData(data) => {
                let grid_obj = data.borrow::<Grid>()?;
                Ok(grid_obj.clone())
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: lua_value.type_name(),
                to: ("Grid").to_string().into(),
                message: Some("Expected a Grid object".to_string()),
            }),
        }

    }
}

impl Frame {
    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let idx = (y as usize) * DIMENSION + (x as usize);
        if let Some(pixel) = self.data.get(idx) {
            return *pixel;
        }

        [0, 0, 0, 0]
    }

    pub fn get_pixel_as_rgba(&self, x: u32, y: u32) -> Rgba<u8> {
        let pixel_data = self.get_pixel(x, y);

        Rgba(pixel_data)
    }
}


impl Grid {
    pub fn get_frame(&self, frame_index: usize) -> &Frame {
        if let Some(frame) = self.frames.get(frame_index) {
            return &frame;
        }

        return &self.frames[0];
    }

    pub fn frame_count(&self) -> usize {
        return self.frames.len();
    }
}

pub fn execute_lua(code: &str) -> Result<Grid> {
    let lua = Lua::new();
    let grid: Grid = Grid {
        width: DIMENSION,
        height: DIMENSION, 
        current_frame: 0, 
        frames: vec![ Frame {id: 1, data: vec![[0; 4]; DIMENSION*DIMENSION]}] 
    };

    lua.globals().set("grid", grid)?;
    
    // Sandboxing
    lua.load("os = nil; io = nil; debug = nil;").exec()?;
    
    let chunk = lua.load(code);
    chunk.exec()?;

    let globals = lua.globals() as Table;
    let grid: Grid = globals.get::<Grid>("grid")?;

    Ok(grid)
}
