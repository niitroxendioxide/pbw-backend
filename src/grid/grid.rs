use mlua::{Lua, Result, Table, UserData, UserDataMethods, FromLua};
use image::{Rgba};

#[derive(Clone, serde::Serialize)]
pub struct Frame {
    pub id: usize,
    pub size: usize,
    pub data: Vec<[u8; 4]>,
}


#[derive(Clone, serde::Serialize)]
pub struct Grid {
    pub size: usize,
    pub current_frame: usize,
    pub frames: Vec<Frame>,
}

fn get_location(size_dimension: usize, x: i64, y: i64) -> usize {
    let corrected_x = x.clamp(0, size_dimension as i64);
    
    return (corrected_x + (y * size_dimension as i64)) as usize;
}

fn is_index_in_bounds(idx: usize, size_dimension: usize) -> bool {
    return idx <= ((size_dimension*size_dimension) - 1);
}

impl UserData for Grid {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {

        methods.add_method_mut("set_pixel", |_, this, (x, y, r, g, b): (i64, i64, u8, u8, u8)| {
            let base_frame_data = &mut this.frames[this.current_frame].data;
            let idx = get_location(this.size, x, y);
            if is_index_in_bounds(idx, this.size) {
                base_frame_data[idx] = [r, g, b, 255];
            }

            Ok(())
        });

        methods.add_method_mut("set_pixel_rgba", |_, this, (x, y, r, g, b, a): (i64, i64, u8, u8, u8, u8)| {
            let base_frame_data = &mut this.frames[this.current_frame].data;
            let idx = get_location(this.size, x, y);
            if is_index_in_bounds(idx, this.size) {
                base_frame_data[idx] = [r, g, b, a];
            }

            Ok(())
        });

        methods.add_method_mut("set_area", |_, this: &mut Grid, (left, top, width, height, r, g, b) : (i64, i64, i64, i64, u8, u8, u8)| {
            let base_frame_data = &mut this.frames[this.current_frame].data;

            for x in left..(left + width) {
                for y in top..(top + height) {
                    let idx = get_location(this.size, x, y);
                    if is_index_in_bounds(idx, this.size) {
                        base_frame_data[idx] = [r, g, b, 255];
                    }
                }
            }

            Ok(())
        });

        methods.add_method_mut("create_frame", |_, this, ()| {
            this.create_frame();

            Ok(())
        });

        methods.add_method_mut("switch_frame", |_, this, frame_to_switch: usize | {
            if frame_to_switch > (this.frames.len()-1) {
                return Ok(());
            }   else {
                return Err(mlua::Error::RuntimeError("Frame index out of bounds".to_string()));
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
        let idx = (y as usize) * self.size + (x as usize);
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

    pub fn create_frame(&mut self) {
        let new_frame = Frame {
            id: self.frame_count() + 1,
            size: self.size,
            data: vec![[0; 4]; self.size*self.size]
        };

        self.frames.push(new_frame);
    }

    pub fn frame_count(&self) -> usize {
        return self.frames.len();
    }
}

pub fn execute_lua(code: &str, p_dimension: usize) -> Result<Grid> {
    let lua = Lua::new();
    let mut grid: Grid = Grid {
        size: p_dimension,
        current_frame: 0, 
        frames: vec![] 
    };

    grid.create_frame();

    match lua.globals().set("grid", grid) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match lua.load("os = nil; io = nil; debug = nil;").exec() {
        Ok(_) => (),
        Err(err) => return Err(err),
    }   
    
    let chunk = lua.load(code);
    if let Err(err) = chunk.exec() {
        return Err(err);
    }

    let globals = lua.globals() as Table;
    match globals.get::<Grid>("grid") {
        Ok(grid) => Ok(grid),
        Err(err) => Err(err),
    }
}
