use godot::{classes::{file_access::ModeFlags, FileAccess, IResourceFormatLoader, IScriptLanguageExtension}, prelude::*};

use crate::{get_mathlang_gd, MathLang};

#[derive(GodotClass)]
#[class(base = ResourceFormatLoader, init)]
pub struct MathLoader {}

#[godot_api]
#[allow(unused_variables)]
impl IResourceFormatLoader for MathLoader {
    fn load(
        &self,
        path: GString,
        original_path: GString,
        use_sub_threads: bool,
        cache_mode: i32,
    ) -> Variant {
        //try and open the requested file
        if let Some(file) = FileAccess::open(&path, ModeFlags::READ) {
            let mpt: GString = "".into(); // emptystring since MathLang has 1 template that is applied no matter the args. and so it creates it
            let mut ml = get_mathlang_gd().cast::<MathLang>().bind().make_template(mpt.clone(),mpt.clone(),mpt).unwrap();
            ml.set_source_code(&file.get_as_text()); //we then set the source code of the mathlang (reload not nesecarry since MathInstance just takes source and parses it)
            ml.to_variant()
        } else {
            godot_error!("Couldn't open file {path} for reading MathLoader script from it");
            Variant::nil()
        }
    }

    // check if we handle the type
    fn handles_type(&self, typ: StringName) -> bool {
        let cast = typ.to_string();
        let str: &str = &cast;
        // VERY IMPORTANT: must contain "Script" otherwise it *will* try to load as a TextFile
        // and then procede to crash godot as it cant cast TestFile to Script
        ["MathScript","Script"].contains(&str)
    }

    fn exists(&self, path: GString) -> bool {
        std::fs::exists(path.to_string()).unwrap_or(false)
    }

    fn get_recognized_extensions(&self) -> PackedStringArray {
       PackedStringArray::from(["ml".into()] as [GString; 1])
    }

    //check if the file ends in .ml so it must be a MathLang script. otherwise we dont know. return empty
    fn get_resource_type(&self, path: GString) -> GString {
        if path.to_string().ends_with(".ml") {
            "MathScript"
        } else {
            ""
        }.into()
    }
}
