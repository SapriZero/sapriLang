// core/core/src/config_loader.rs

use sapri_obj::Obj;
use crate::Runtime;

pub trait FromSson: Sized {
    fn from_sson(obj: &Obj) -> Result<Self, String>;
}

impl FromSson for String {
    fn from_sson(obj: &Obj) -> Result<Self, String> {
        obj.as_str().map(|s| s.to_string()).ok_or_else(|| "Not a string".to_string())
    }
}

impl FromSson for u64 {
    fn from_sson(obj: &Obj) -> Result<Self, String> {
        obj.as_number().map(|n| n as u64).ok_or_else(|| "Not a number".to_string())
    }
}

impl FromSson for Vec<String> {
    fn from_sson(obj: &Obj) -> Result<Self, String> {
        if let Some(arr) = obj.as_array() {
            arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
        } else {
            Ok(Vec::new())
        }
    }
}

pub fn load_config<T: FromSson>(runtime: &mut Runtime, path: &str) -> Result<T, String> {
    let result = runtime.execute(Command::Eval { 
        expr: format!("load_sson '{}'", path)
    })?;
    // TODO: parse result into Obj
    let obj = Obj::new();
    T::from_sson(&obj)
}
