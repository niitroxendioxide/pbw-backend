use mlua::{Lua, Result, UserData, UserDataMethods};

static DIMENSION: usize = 3;
#[derive(Clone)]
pub struct Grid {
    pub data: [[u8; 4]; DIMENSION*DIMENSION],
}

impl UserData for Grid {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set_pixel", |_, this, (x, y, r, g, b): (i64, i64, u8, u8, u8)| {
            let idx = (y * (DIMENSION as i64) + x) as usize;
            if idx < DIMENSION*DIMENSION {
                this.data[idx] = [r, g, b, 255];
            }

            println!("Edited pixel. Pos: Vec2({}, {}) Color: RGB({}, {}, {})", x, y, r, g, b);
            Ok(())
        });
    }
}

pub fn execute_lua(code: &str) -> Result<Grid> {
    let lua = Lua::new();
    let grid: Grid = Grid { data: [[0; 4]; DIMENSION*DIMENSION] };

    lua.globals().set("grid", grid)?;
    
    // Sandboxing
    lua.load("os = nil; io = nil; debug = nil;").exec()?;
    
    let chunk = lua.load(code);
    chunk.exec()?;

    lua.globals().get("grid")?;

    Ok(grid)
}