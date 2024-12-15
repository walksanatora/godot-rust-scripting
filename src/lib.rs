/// behold: MathLang a revolutionary lang that has *no* functions only externally accessable properties

mod lang;
mod script;
mod instance;
mod saver;
mod loader;
use std::sync::atomic::{AtomicI64, Ordering};

use godot::{classes::{Engine, Object, ResourceLoader, ResourceSaver, ScriptLanguage}, init::{gdextension, ExtensionLibrary, InitLevel}, obj::{script::ScriptInstance, Gd, InstanceId, NewAlloc, NewGd}};
use lang::*;
use loader::MathLoader;
use saver::MathSaver;

struct Extension;


static MATH_LANG_INSTANCE_ID: AtomicI64 = AtomicI64::new(0);
static MATH_LANG_SAVER_ID: AtomicI64 = AtomicI64::new(0);
static MATH_LANG_LOADER_ID: AtomicI64 = AtomicI64::new(0);

#[gdextension]
unsafe impl ExtensionLibrary for Extension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            let new = MathLang::new_alloc();
            MATH_LANG_INSTANCE_ID.store(new.instance_id().to_i64(), Ordering::Relaxed);
            let mut engine = Engine::singleton();
            engine.register_script_language(&new);

            let mut saver = ResourceSaver::singleton();
            let saver_instance = MathSaver::new_gd();
            MATH_LANG_SAVER_ID.store(saver_instance.instance_id().to_i64(), Ordering::Relaxed);
            saver.add_resource_format_saver(&saver_instance);

            let mut loader = ResourceLoader::singleton();
            let loader_instance = MathLoader::new_gd();
            MATH_LANG_LOADER_ID.store(loader_instance.instance_id().to_i64(), Ordering::Relaxed);
            loader.add_resource_format_loader_ex(&loader_instance).at_front(true).done();
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let mut saver = ResourceSaver::singleton();
            let mut loader = ResourceLoader::singleton();

            let ml_lang: Result<Gd<MathLang>,_> = Gd::try_from_instance_id(InstanceId::from_i64(MATH_LANG_INSTANCE_ID.load(Ordering::Relaxed)));
            if let Ok(math_lang) = ml_lang {
                engine.unregister_script_language(&math_lang);
            }
            let ml_saver: Result<Gd<MathSaver>,_> = Gd::try_from_instance_id(InstanceId::from_i64(MATH_LANG_SAVER_ID.load(Ordering::Relaxed)));
            if let Ok(ml_saver) = ml_saver {
                saver.remove_resource_format_saver(&ml_saver);
            }
            let ml_loader: Result<Gd<MathLoader>,_> = Gd::try_from_instance_id(InstanceId::from_i64(MATH_LANG_LOADER_ID.load(Ordering::Relaxed)));
            if let Ok(ml_loader) = ml_loader {
                loader.remove_resource_format_loader(&ml_loader);
            }

 
        }
    }
}

pub fn get_mathlang_gd() -> Gd<MathLang> {
    Gd::from_instance_id(InstanceId::from_i64(MATH_LANG_INSTANCE_ID.load(Ordering::Relaxed)))
}

use godot::sys::interface_fn;
use godot::sys::SysPtr;

// fn get_script_instance<T>(object: Gd<Object>, language: Gd<ScriptLanguage>) -> Option<ScriptInstanceData<T>> where T: ScriptInstance {
//     let obj_ptr = object.obj_sys().as_const();
//     let lang_ptr = language.obj_sys();
//     let data_ptr = unsafe { interface_fn!(object_get_script_instance)(obj_ptr,lang_ptr) }
//     if !data_ptr.is_null() {
//       Some(ScriptInstanceData::borrow_script_sys(data_ptr))
//     } else {None}
// }
  