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
        if let Some(ress) = resource { //make sure the resource is valid
            if let Ok(upcast) = ress.try_cast::<MathScript>() { //make sure the resource is MathScript
                if let Some(mut file) = FileAccess::open(&path, ModeFlags::WRITE) { //use godot file access for res:// support
                    file.store_string(&upcast.get_source_code()); //write it to file and return
                    Error::OK
                } else {Error::ERR_FILE_CANT_WRITE} //we cant open file for writing so we stop here
            } else {
                godot_error!("resource must be MathScript");
                Error::ERR_INVALID_PARAMETER
            }
        } else {
            godot_error!("resource must not be null");
            Error::ERR_INVALID_PARAMETER
        }
    }

    // this just adds .ml as a valid extension for MathLang scripts, else it returns nothing since we dont handle it
    fn get_recognized_extensions(&self, resource: Option<Gd<Resource>>) -> PackedStringArray {
        if let Some(res) = resource {
            if let Ok(upcast) = res.try_cast::<MathScript>() {
                return PackedStringArray::from(["ml".into()] as [GString; 1]);
            }
        }
        PackedStringArray::new()
    }

    // this return true if the resource is a MathScript else false
    fn recognize(&self, resource: Option<Gd<Resource>>) -> bool {
        if let Some(res) = resource {
            res.try_cast::<MathScript>().is_ok()
        } else {false}
    }

    // honestly just easier to recognize all path and let everything else do it's job
    // probally could insert some fs validations stuff but not our job
    fn recognize_path(&self, resource: Option<Gd<Resource>>, path: GString) -> bool {
        true
    }
}
