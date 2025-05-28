use mlua::{Lua, Result, Table, UserData, UserDataMethods, FromLua};

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

impl FromLua for Grid {
    fn from_lua(lua_value: mlua::Value, lua: &Lua) -> Result<Self> {
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

pub fn execute_lua(code: &str) -> Result<Grid> {
    let lua = Lua::new();
    let grid: Grid = Grid { data: [[0; 4]; DIMENSION*DIMENSION] };

    lua.globals().set("grid", grid)?;
    
    // Sandboxing
    lua.load("os = nil; io = nil; debug = nil;").exec()?;
    
    let chunk = lua.load(code);
    chunk.exec()?;

    let globals = lua.globals() as Table;
    let grid: Grid = globals.get::<Grid>("grid")?;

    Ok(grid)
}