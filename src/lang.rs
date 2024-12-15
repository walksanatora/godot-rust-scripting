use std::ptr::null_mut;

use godot::{
    classes::{script_language::ScriptNameCasing, IScriptLanguageExtension, Script},
    global::Error,
    prelude::*,
};

use crate::script::MathScript;

#[derive(GodotClass)]
#[class(base = ScriptLanguageExtension, init, tool)]
pub struct MathLang {}


#[godot_api]
#[allow(unused_variables)]
impl IScriptLanguageExtension for MathLang {
    fn get_name(&self) -> GString {
        "MathLang".into()
    }

    fn init_ext(&mut self) {} //mathlang needs no lateinit

    fn get_type(&self) -> GString {
        "MathLang".into() //idk why these are seperate
    }

    fn get_extension(&self) -> GString {
        "ml".into()
    }

    fn finish(&mut self) {} //this does.. smth?

    fn get_reserved_words(&self) -> PackedStringArray {
        PackedStringArray::new() //nope we are not gonna be in editor. and we dont have any words
    }

    fn is_control_flow_keyword(&self, keyword: GString) -> bool {
        false //no words means no keyworkds
    }

    fn get_comment_delimiters(&self) -> PackedStringArray {
        PackedStringArray::new() //no commends. it is math if you need to explain it we have bigger problems
    }

    fn get_string_delimiters(&self) -> PackedStringArray {
        PackedStringArray::new() //no words no strings
    }

    fn make_template(
        &self,
        template: GString,
        class_name: GString,
        base_class_name: GString,
    ) -> Option<Gd<Script>> {
        Some(MathScript::new_gd().upcast())
    }

    fn get_built_in_templates(&self, object: StringName) -> Array<Dictionary> {
        let mut arr = array![];
        arr.push(&dict! {
            "name": "Blank",
            "description": "A Blank MathLang file",
            "content": "Idk What Goes Here"
        });
        arr
    }

    fn is_using_templates(&mut self) -> bool {
        false //No Templates
    }

    fn validate(
        &self,
        script: GString,
        path: GString,
        validate_functions: bool,
        validate_errors: bool,
        validate_warnings: bool,
        validate_safe_lines: bool,
    ) -> Dictionary {
        dict! {} //TODO: figure out how to format this
    }

    fn validate_path(&self, path: GString) -> GString {
        "".into() //TODO: figure out what this does
    }

    fn create_script(&self) -> Option<Gd<Object>> {
        Some(MathScript::new_gd().upcast())
    }

    fn has_named_classes(&self) -> bool {
        false //TODO: figure out what this means
    }

    fn supports_builtin_mode(&self) -> bool {
        true //TODO: figure out what this means (assumption: can it be edited in internal editor)
    }

    fn supports_documentation(&self) -> bool {
        false //No Documentation
    }

    fn can_inherit_from_file(&self) -> bool {
        false //TODO: figure out what this does
    }

    fn find_function(&self, function: GString, code: GString) -> i32 {
        -1 //we have no functions
    }

    fn make_function(
        &self,
        class_name: GString,
        function_name: GString,
        function_args: PackedStringArray,
    ) -> GString {
        "".into() //we have no functions you sinner
    }

    fn can_make_function(&self) -> bool {
        false //
    }

    fn open_in_external_editor(
        &mut self,
        script: Option<Gd<Script>>,
        line: i32,
        column: i32,
    ) -> Error {
        Error::ERR_UNAVAILABLE //we dont have any external editors yet
    }

    fn overrides_external_editor(&mut self) -> bool {
        false //no external editor
    }

    fn preferred_file_name_casing(&self) -> ScriptNameCasing {
        ScriptNameCasing::AUTO
    }

    fn complete_code(&self, code: GString, path: GString, owner: Option<Gd<Object>>) -> Dictionary {
        dict! {} //TODO: figure out the format for this
    }

    fn lookup_code(
        &self,
        code: GString,
        symbol: GString,
        path: GString,
        owner: Option<Gd<Object>>,
    ) -> Dictionary {
        dict! {} //TODO: figure out the format for this
    }

    fn auto_indent_code(&self, code: GString, from_line: i32, to_line: i32) -> GString {
        "".into() //I think it just takes source and tells you to re-indent the section specified
    }

    fn add_global_constant(&mut self, name: StringName, value: Variant) {
        //no
    }

    fn add_named_global_constant(&mut self, name: StringName, value: Variant) {
        //no
    }

    fn remove_named_global_constant(&mut self, name: StringName) {
        //no
    }

    fn thread_enter(&mut self) {
        //Useless callback
    }

    fn thread_exit(&mut self) {
        //Uselesscallback
    }

    fn debug_get_error(&self) -> GString {
        "".into() //beautiful lang has no errors
    }

    fn debug_get_stack_level_count(&self) -> i32 {
        -1 //negative?
    }

    fn debug_get_stack_level_line(&self, level: i32) -> i32 {
        -1
    }

    fn debug_get_stack_level_function(&self, level: i32) -> GString {
        "".into()
    }

    fn debug_get_stack_level_source(&self, level: i32) -> GString {
        "".into()
    }

    fn debug_get_stack_level_locals(
        &mut self,
        level: i32,
        max_subitems: i32,
        max_depth: i32,
    ) -> Dictionary {
        dict! {}
    }

    fn debug_get_stack_level_members(
        &mut self,
        level: i32,
        max_subitems: i32,
        max_depth: i32,
    ) -> Dictionary {
        dict! {}
    }

    unsafe fn debug_get_stack_level_instance(&mut self, level: i32) -> *mut std::ffi::c_void {
        null_mut()
    }

    fn debug_get_globals(&mut self, max_subitems: i32, max_depth: i32) -> Dictionary {
        dict! {}
    }

    fn debug_parse_stack_level_expression(
        &mut self,
        level: i32,
        expression: GString,
        max_subitems: i32,
        max_depth: i32,
    ) -> GString {
        "".into()
    }

    fn debug_get_current_stack_info(&mut self) -> Array<Dictionary> {
        array![]
    }

    fn reload_all_scripts(&mut self) {
        //TODO: make it keep a internal buffer of Gd<MathScript>s and reload them
    }

    fn reload_tool_script(&mut self, script: Option<Gd<Script>>, soft_reload: bool) {
        //all MathLang is tool so i think it just wants to do this? but what does soft reload mean
    }

    fn get_recognized_extensions(&self) -> PackedStringArray {
        PackedStringArray::from([self.get_extension()])
    }

    fn get_public_functions(&self) -> Array<Dictionary> {
        array![] //no funcs
    }

    fn get_public_constants(&self) -> Dictionary {
        dict! {} //no constants
    }

    fn get_public_annotations(&self) -> Array<Dictionary> {
        array![] //no annotations
    }

    fn profiling_start(&mut self) {} //useless callback

    fn profiling_stop(&mut self) {} //useless callback

    fn profiling_set_save_native_calls(&mut self, enable: bool) {} //useless callback

    unsafe fn profiling_get_accumulated_data(
        &mut self,
        info_array: *mut godot::classes::native::ScriptLanguageExtensionProfilingInfo,
        info_max: i32,
    ) -> i32 {
        -1 //idk what they want
    }

    unsafe fn profiling_get_frame_data(
        &mut self,
        info_array: *mut godot::classes::native::ScriptLanguageExtensionProfilingInfo,
        info_max: i32,
    ) -> i32 {
        -1 //idk what they want
    }

    fn frame(&mut self) {} //useless callback (that doesn't appear to be called?)

    fn handles_global_class_type(&self, type_: GString) -> bool {
        false
    }

    fn get_global_class_name(&self, path: GString) -> Dictionary {
        dict! {}
    }
}
