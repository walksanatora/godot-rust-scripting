use std::collections::HashMap;

use elsa::FrozenMap;
use evalexpr::{eval_with_context, Context, DefaultNumericTypes, EvalexprError, Value};
use godot::classes::Script;
use godot::global::{PropertyHint, PropertyUsageFlags};
use godot::meta::{ClassName, PropertyHintInfo, PropertyInfo};
use godot::prelude::*;
use godot::obj::Gd;
use godot::{classes::Object, obj::script::ScriptInstance};
use godot::sys;

use crate::get_mathlang_gd;
use crate::script::MathScript;

#[allow(dead_code)]
pub struct MathInstance {
    placeholder: bool,
    base: Gd<Script>,
    cache: FrozenMap<String,Box<Value<DefaultNumericTypes>>>,
    funcs: HashMap<String,String>,
    math_builtins: bool
}

impl MathInstance {
    pub fn new(placeholder: bool, script: Gd<MathScript>) -> Self {
        let source = script.get_source_code().to_string();
        let mut funcs = HashMap::new();
        for line in source.lines() {
            let split = line.split(':').collect::<Vec<&str>>();
            let name = &split[0];
            let func: &[&str] = &split[1..];
            funcs.insert(name.to_string(), func.join(":"));
        }
        Self {
            placeholder,
            base: script.upcast(),
            cache: FrozenMap::new(),
            funcs,
            math_builtins: true
        }
    }
    pub fn reload(&mut self, source: &str) {
        println!("Reloading source code: \n{source}");
        let mut funcs = HashMap::new();
        for line in source.lines() {
            let split = line.split(':').collect::<Vec<&str>>();
            let name = &split[0];
            let func: &[&str] = &split[1..];
            funcs.insert(name.to_string(), func.join(":"));
        }
        self.funcs = funcs;
        self.cache = FrozenMap::new();
    }
}

fn val_to_float(value: &Value<DefaultNumericTypes>) -> f64 {
    match value {
        Value::String(str) => str.len() as f64,
        Value::Float(flt) => *flt,
        Value::Int(int) => *int as f64,
        Value::Boolean(bool) => if *bool {1.0} else {0.0},
        Value::Tuple(vec) => vec.iter().map(val_to_float).sum(),
        Value::Empty => 0.0,
    }
} 

#[allow(unused_variables)]
impl ScriptInstance for MathInstance {
    type Base = Object;

    fn class_name(&self) -> godot::prelude::GString {
        "MathInstance".into()
    }

    fn set_property(this: godot::obj::script::SiMut<Self>, name: godot::prelude::StringName, value: &godot::prelude::Variant) -> bool {
        false //you cannot edit mathlang
    }

    fn get_property(&self, name: StringName) -> Option<godot::prelude::Variant> {
        godot_print!("let MathInstance.{name}");
        let identifier = name.to_string();
        let val = {
            if let Some(cached) = self.cache.get(&identifier) {
                Some(cached)
            } else if let Some(expr) = self.funcs.get(&identifier) {
                if let Ok(val) = eval_with_context(expr, self) {
                    self.cache.insert(identifier.to_string(), Box::new(val));
                    self.cache.get(&identifier)
                } else {None}
            } else {None}
        };

        val.map(|value| {
            val_to_float(value).to_variant()
        })
    }

    fn get_property_list(&self) -> Vec<godot::meta::PropertyInfo> {
        self.funcs.keys().map(|key| {
            PropertyInfo {
                variant_type: VariantType::FLOAT,
                class_name: ClassName::none(),
                property_name: key.clone().into(),
                hint_info: PropertyHintInfo {
                    hint: PropertyHint::NONE,
                    hint_string: "".into()
                },
                usage: PropertyUsageFlags::READ_ONLY
            }
        }).collect()
    }

    fn get_method_list(&self) -> Vec<godot::meta::MethodInfo> {
        vec![]
    }

    fn call(
        mut this: godot::obj::script::SiMut<Self>,
        method: godot::prelude::StringName,
        args: &[&godot::prelude::Variant],
    ) -> Result<godot::prelude::Variant, sys::GDExtensionCallErrorType> {
        let len = args.len();
        let mut argv = String::new();
        for (i,v) in args.iter().enumerate() {
            if i == len {
                argv += &format!("{v}, ")
            } else {
                argv += &format!("{v}")
            }
        }
        godot_print!("fn MathInstance::{method}({})", argv);

        let reload_method: StringName = "@_reload".into();
        if !args.is_empty() {
            match (&method.to_string().as_str(),args[0].get_type()) {
                (&"@_reload", VariantType::STRING) => {
                    this.reload(&args[0].to_string());
                    Ok(Variant::nil())
                },
                _ => { Err(1) } //GDEXTENSION_CALL_ERROR_INVALID_METHOD
            }
        } else {Err(1)}
    }

    fn is_placeholder(&self) -> bool {
        self.placeholder
    }

    fn has_method(&self, method: StringName) -> bool {
        godot_print!("does fn MathInstance::{method} exists?");
        false //MathLang has no methods
    }

    fn get_script(&self) -> &Gd<Script> {
        &self.base
    }

    fn get_property_type(&self, name: StringName) -> VariantType {
        VariantType::FLOAT
    }

    fn to_string(&self) -> GString {
        "MathScript".into()
    }

    fn get_property_state(&self) -> Vec<(StringName, godot::prelude::Variant)> {
        vec![]
    }

    fn get_language(&self) -> godot::prelude::Gd<godot::classes::ScriptLanguage> {
        get_mathlang_gd().upcast()
    }

    fn on_refcount_decremented(&self) -> bool {
        true
    }

    fn on_refcount_incremented(&self) {}

    fn property_get_fallback(&self, name: godot::prelude::StringName) -> Option<godot::prelude::Variant> {
        None //why are you using fallback?
    }
    fn property_set_fallback(this: godot::obj::script::SiMut<Self>, name: godot::prelude::StringName, value: &godot::prelude::Variant) -> bool {
        false //why are you using fallback?
    }

    fn get_method_argument_count(&self, _method: godot::prelude::StringName) -> Option<u32> {
        None
    }
}


impl Context for MathInstance {
    type NumericTypes = DefaultNumericTypes;

    fn get_value(&self, identifier: &str) -> Option<&evalexpr::Value<Self::NumericTypes>> {
        if let Some(cached) = self.cache.get(identifier) {
            Some(cached)
        } else if let Some(expr) = self.funcs.get(identifier) {
            if let Ok(val) = eval_with_context(expr, self) {
                self.cache.insert(identifier.to_string(), Box::new(val));
                self.cache.get(identifier)
            } else {None}
        } else {None}
    }

    fn call_function(
        &self,
        identifier: &str,
        argument: &evalexpr::Value<Self::NumericTypes>,
    ) -> evalexpr::error::EvalexprResultValue<Self::NumericTypes> {
        Err(EvalexprError::FunctionIdentifierNotFound(
            identifier.to_string(),
        ))
    }

    fn are_builtin_functions_disabled(&self) -> bool {
        self.math_builtins
    }

    fn set_builtin_functions_disabled(
        &mut self,
        disabled: bool,
    ) -> evalexpr::EvalexprResult<(), Self::NumericTypes> {
        self.math_builtins = disabled;
        Ok(())
    }
}