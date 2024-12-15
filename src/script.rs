use elsa::FrozenVec;
use godot::{
    classes::{IScriptExtension, Script, ScriptExtension, ScriptLanguage, Object},
    global::Error,
    obj::script::create_script_instance,
    prelude::*
};

use crate::{get_mathlang_gd, instance::MathInstance};

// enum for holing refrences. this allows us to dispose of ref counted objects when we are the only thing keeping them alive
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

    //functions I know what they do

    //these create the ScriptInstances and send them to godot. we also call self.attach here to add them to our frozen vec
    unsafe fn instance_create(&self, for_object: Gd<Object>) -> *mut std::ffi::c_void {
        self.attach(for_object.clone());
        create_script_instance(MathInstance::new(false, self.to_gd()), for_object)
    }
    unsafe fn placeholder_instance_create(&self, for_object: Gd<Object>) -> *mut std::ffi::c_void {
        self.attach(for_object.clone());
        create_script_instance(MathInstance::new(true, self.to_gd()), for_object)
    }

    // if we have source, and getters/setters
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
        // first we make a mutable copy of our attachments and filter for valid ones
        let mut re_owned: Vec<Box<Attachment>> = self.attachments.clone().into_vec().into_iter().filter(|it|
            it.valid()
        ).collect();

        //godot_print!("attachments: {re_owned:?}");

        //we turn our source code into a variant so it can be passed in
        let source = self.source.to_string().to_variant();

        //for each object this script is attached to
        for attache in re_owned.iter_mut() {
            //we call our scripts @_reload method with our source code.
            //note: this is a hack untill the ability to get from Gd<Object> back to dyn ScriptInstance is implemneted
            let ptr = match attache.as_mut() {
                Attachment::Obj(gd) => gd.call("@_reload", &[source.clone()]),
                Attachment::RefCounted(gd) => gd.call("@_reload", &[source.clone()]),
            };
            
        }
        // update our attachments. our old FrozenVec is dropped here which should also drop it's contents
        self.attachments = re_owned.into();

        // and we reloaded successfully
        Error::OK
    }

    // gets the ScriptLanguage associated with this script (no clue what godot does with it)
    fn get_language(&self) -> Option<Gd<ScriptLanguage>> {
        Some(get_mathlang_gd().upcast())
    }

    // does this script run in editor
    fn is_tool(&self) -> bool {
        true
    }

    //functions I dont know what they affect
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
        "MathScript".into()
    }

    fn inherits_script(&self, script: Gd<Script>) -> bool {
        false
    }

    fn get_instance_base_type(&self) -> StringName {
        "Object".into()
    }

    fn instance_has(&self, object: Gd<Object>) -> bool {
        false
    }

    fn get_documentation(&self) -> Array< Dictionary > {
        array![]
    }
    fn has_method(&self, method: StringName) -> bool {
        false
    }
    fn has_static_method(&self, method: StringName) -> bool {
        false
    }

    fn get_method_info(&self, method: StringName) -> Dictionary {
        dict! {}
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn has_script_signal(&self, signal: StringName) -> bool {
        false
    }

    fn get_script_signal_list(&self) -> Array< Dictionary > {
        array![]
    }

    fn has_property_default_value(&self, property: StringName) -> bool {
        false
    }

    fn get_property_default_value(&self, property: StringName) -> Variant {
        Variant::nil()
    }

    fn update_exports(&mut self) {}

    fn get_script_method_list(&self) -> Array<Dictionary> {
        array![]
    }

    fn get_script_property_list(&self) -> Array<Dictionary> {
        array![]
    }

    fn get_member_line(&self, member: StringName) -> i32 {
        -1
    }

    fn get_constants(&self) -> Dictionary {
        dict! {}
    }

    fn get_members(&self) -> Array<StringName> {
        array![]
    }

    fn is_placeholder_fallback_enabled(&self) -> bool {
        false
    }

    fn get_rpc_config(&self) -> Variant {
        Variant::nil()
    }
}
