
// the individual parts of our scripting language. all must exists
mod instance;
mod lang;
mod loader;
mod saver;
mod script;

//we use atomics here to store InstanceIDs to our various resources
use std::sync::atomic::{AtomicI64, Ordering};

//godot classes used in registration
use godot::{
    classes::{Engine, ResourceLoader, ResourceSaver},
    init::{gdextension, ExtensionLibrary, InitLevel},
    obj::{Gd, InstanceId, NewAlloc, NewGd},
};

//and finally our script imports
use lang::MathLang;
use loader::MathLoader;
use saver::MathSaver;

/// mandatory struct for gdext init
struct Extension;

static MATH_LANG_INSTANCE_ID: AtomicI64 = AtomicI64::new(0);
static MATH_LANG_SAVER_ID: AtomicI64 = AtomicI64::new(0);
static MATH_LANG_LOADER_ID: AtomicI64 = AtomicI64::new(0);

#[gdextension]
unsafe impl ExtensionLibrary for Extension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene { // we init on Scene level cause it is the lastest avaliable

            //here we init our new ScriptLanguage implementation. and register it
            let lang = MathLang::new_alloc();
            MATH_LANG_INSTANCE_ID.store(lang.instance_id().to_i64(), Ordering::Relaxed);
            let mut engine = Engine::singleton();
            engine.register_script_language(&lang);

            //here we init our new ResourceFormatSaver for MathLang
            let mut saver = ResourceSaver::singleton();
            let saver_instance = MathSaver::new_gd();
            MATH_LANG_SAVER_ID.store(saver_instance.instance_id().to_i64(), Ordering::Relaxed);
            saver.add_resource_format_saver(&saver_instance);

            //here we init our new ResourceFormatLoader for MathLang
            let mut loader = ResourceLoader::singleton();
            let loader_instance = MathLoader::new_gd();
            MATH_LANG_LOADER_ID.store(loader_instance.instance_id().to_i64(), Ordering::Relaxed);
            loader
                .add_resource_format_loader_ex(&loader_instance)
                .at_front(true)// No clue if it has to be at front
                .done();
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            // get the singletons
            let mut engine = Engine::singleton();
            let mut saver = ResourceSaver::singleton();
            let mut loader = ResourceLoader::singleton();

            // try to get our ScriptLanguage singleton
            let ml_lang: Result<Gd<MathLang>, _> = Gd::try_from_instance_id(InstanceId::from_i64(
                MATH_LANG_INSTANCE_ID.load(Ordering::Relaxed),
            ));
            // and if it exist... delete it
            if let Ok(math_lang) = ml_lang {
                engine.unregister_script_language(&math_lang);
            }

            // try to get our MathSaver singleton
            let ml_saver: Result<Gd<MathSaver>, _> = Gd::try_from_instance_id(
                InstanceId::from_i64(MATH_LANG_SAVER_ID.load(Ordering::Relaxed)),
            );
            // and if it exists... delete it
            if let Ok(ml_saver) = ml_saver {
                saver.remove_resource_format_saver(&ml_saver);
            }

            // you know the drill by now
            let ml_loader: Result<Gd<MathLoader>, _> = Gd::try_from_instance_id(
                InstanceId::from_i64(MATH_LANG_LOADER_ID.load(Ordering::Relaxed)),
            );
            if let Ok(ml_loader) = ml_loader {
                loader.remove_resource_format_loader(&ml_loader);
            }
        }
    }
}

//utility function to get the singleton for MathLang as both Script and Instances need it
#[inline]
pub fn get_mathlang_gd() -> Gd<MathLang> {
    Gd::from_instance_id(InstanceId::from_i64(
        MATH_LANG_INSTANCE_ID.load(Ordering::Relaxed),
    ))
}

