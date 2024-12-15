use elsa::FrozenVec;
use godot::{
    classes::{IScriptExtension, Script, ScriptExtension, ScriptLanguage, Object},
    global::Error,
    obj::script::create_script_instance,
    prelude::*,
    sys::{self, SysPtr}
};

use crate::{get_mathlang_gd, instance::MathInstance};

#[derive(Clone,Debug)]
enum Attachment {
    Obj(Gd<Object>),
    RefCounted(Gd<RefCounted>)
}

impl Attachment {
    fn valid(&self) -> bool {
        match self {
            Attachment::Obj(gd) => gd.is_instance_valid(),
            Attachment::RefCounted(gd) => {
                gd.is_instance_valid() && gd.get_reference_count() > 1
            },
        }
    }
}

#[derive(GodotClass)]
#[class(base = ScriptExtension, init, tool)]
pub struct MathScript {
    #[var(usage_flags = [STORAGE])]
    class_name: GString,
    source: GString,
    base: Base<ScriptExtension>,
    attachments: FrozenVec<Box<Attachment>>
}

impl MathScript {
    fn attach(&self, target: Gd<Object>) {
        let att = match target.try_cast::<RefCounted>() {
            Ok(rc) => Attachment::RefCounted(rc),
            Err(obj) => Attachment::Obj(obj),
        };
        self.attachments.push(Box::from(att));
    }
}

#[godot_api]
#[allow(unused_variables)]
impl IScriptExtension for MathScript {
    fn editor_can_reload_from_file(&mut self) -> bool {
        true
    }

    fn can_instantiate(&self) -> bool {
        true
    }

    fn get_base_script(&self) -> Option<Gd<Script>> {
        None
    }

    fn get_global_name(&self) -> StringName {
        self.class_name.clone().into()
    }

    fn inherits_script(&self, script: Gd<Script>) -> bool {
        false
    }

    fn get_instance_base_type(&self) -> StringName {
        "Object".into()
    }

    unsafe fn instance_create(&self, for_object: Gd<Object>) -> *mut std::ffi::c_void {
        self.attach(for_object.clone());
        create_script_instance(MathInstance::new(false, self.to_gd()), for_object)
    }
    unsafe fn placeholder_instance_create(&self, for_object: Gd<Object>) -> *mut std::ffi::c_void {
        self.attach(for_object.clone());
        create_script_instance(MathInstance::new(true, self.to_gd()), for_object)
    }

    fn instance_has(&self, object: Gd<Object>) -> bool {
        false
    }

    fn has_source_code(&self) -> bool {
        true
    }
    fn set_source_code(&mut self, code: GString) {
        self.source = code
    }
    fn get_source_code(&self) -> GString {
        self.source.clone()
    }

    fn reload(&mut self, keep_state: bool) -> Error {
        godot_print!("Reloading MathScript");
        let lang = get_mathlang_gd();
        let lang_ptr = lang.obj_sys();
        let interface = sys::interface_fn!(object_get_script_instance);
        
        let mut re_owned: Vec<Box<Attachment>> = self.attachments.clone().into_vec().into_iter().filter(|it|
            it.valid()
        ).collect();

        godot_print!("attachments: {re_owned:?}");

        let source = self.source.to_string().to_variant();

        for attache in re_owned.iter_mut() {
            let ptr = match attache.as_mut() {
                Attachment::Obj(gd) => gd.call("@_reload", &[source.clone()]),
                Attachment::RefCounted(gd) => gd.call("@_reload", &[source.clone()]),
            };
            
        }

        self.attachments = re_owned.into();

        Error::ERR_UNAVAILABLE
    }

    fn get_documentation(&self,) -> Array< Dictionary > {
        array![]
    }
    fn has_method(&self, method: StringName,) -> bool {
        false
    }
    fn has_static_method(&self, method: StringName,) -> bool {
        false
    }

    fn get_method_info(&self, method: StringName,) -> Dictionary {
        dict! {} //there is no methods though?
    }

    fn is_tool(&self,) -> bool {
        true //I *guess* you can run it in editor
    }

    fn is_valid(&self,) -> bool {
        true //mathlang cannot be invalidated!!!
    }

    fn get_language(&self,) -> Option<Gd<ScriptLanguage>> {
        Some(get_mathlang_gd().upcast())
    }

    fn has_script_signal(&self, signal: StringName,) -> bool {
        false
    }

    fn get_script_signal_list(&self,) -> Array< Dictionary > {
        array![]
    }

    fn has_property_default_value(&self, property: StringName,) -> bool {
        false
    }

    fn get_property_default_value(&self, property: StringName,) -> Variant {
        Variant::nil()
    }

    fn update_exports(&mut self,) {}

    fn get_script_method_list(&self,) -> Array< Dictionary > {
        array![]
    }

    fn get_script_property_list(&self,) -> Array< Dictionary > {
        array![] //todo: make this return each "function" in mathlang
    }

    fn get_member_line(&self, member: StringName,) -> i32 {
        -1
    }

    fn get_constants(&self,) -> Dictionary {
        dict! {}
    }

    fn get_members(&self,) -> Array< StringName > {
        array![] //todo: make this return lines
    }

    fn is_placeholder_fallback_enabled(&self,) -> bool {
        false
    }

    fn get_rpc_config(&self,) -> Variant {
        Variant::nil()
    }
}
