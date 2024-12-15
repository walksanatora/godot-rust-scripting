use godot::{
    classes::{file_access::ModeFlags, FileAccess, IResourceFormatSaver},
    global::Error,
    prelude::*,
};

use crate::script::MathScript;

#[derive(GodotClass)]
#[class(base = ResourceFormatSaver, init)]
pub struct MathSaver {}

#[godot_api]
#[allow(unused_variables)]
impl IResourceFormatSaver for MathSaver {
    fn save(&mut self, resource: Option<Gd<Resource>>, path: GString, flags: u32) -> Error {
        godot_print!("Saving resource with MathSaver, flags: {flags} to {path}");
        if let Some(ress) = resource {
            if let Ok(upcast) = ress.try_cast::<MathScript>() {
                if let Some(mut file) = FileAccess::open(&path, ModeFlags::WRITE) {
                    file.store_string(&upcast.get_source_code());
                    Error::OK
                } else {Error::ERR_FILE_CANT_WRITE}
            } else {
                godot_error!("resource must be MathScript");
                Error::ERR_INVALID_PARAMETER
            }
        } else {
            godot_error!("resource must not be null");
            Error::ERR_INVALID_PARAMETER
        }
    }
    fn get_recognized_extensions(&self, resource: Option<Gd<Resource>>) -> PackedStringArray {
        if let Some(res) = resource {
            if let Ok(upcast) = res.try_cast::<MathScript>() {
                return PackedStringArray::from(["ml".into()] as [GString; 1]);
            }
        }
        PackedStringArray::new()
    }

    fn recognize(&self, resource: Option<Gd<Resource>>) -> bool {
        if let Some(res) = resource {
            res.try_cast::<MathScript>().is_ok()
        } else {false}
    }

    fn recognize_path(&self, resource: Option<Gd<Resource>>, path: GString) -> bool {
        true
    }
}
