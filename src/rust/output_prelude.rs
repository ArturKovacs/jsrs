use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct JsString {
    // TODO: Replace with something that can be used to represent UTF16 strings efficiently
    // (strings in JS behave as UTF16 strings)
    //
    // An idea that I have is to use SSO, such that the inlined string is UTF8 so that
    // it fits more chars and the outlined (long) strintgs are UTF16 so that we save
    // processing power by avoiding the conversion between UTF16 and UTF8
    // (because JS string operations all treat the string as if it was UTF16)
    // 
    value: Rc<String>
}

pub type JsObject = Rc<JsObjectContents>;

pub struct JsObjectContents {
    // TODO: replace this with VecMap, (or ArrayMap, I'm still not sure about the name)
    // a map that stores all key-value pairs (maybe up until a certain amount)
    // in a Vec or array. (Because I THINK that most objects contain few keys,
    // so it might help performance to store them in contiguous memory)
    properties: HashMap<JsString, JsValue>,

    
    /// If this object is a function, then this will be `Some`.
    ///
    /// The slice is used to pass the parameters here
    function: Option<Box<dyn Fn(&[JsValue])>>,
}

#[derive(Clone)]
pub enum JsValue {
    Null,
    Undefined,
    Boolean(bool),
    Number(f64),
    String(JsString),
    Object(JsObject)
}

impl JsValue {
    pub fn add(&self, other: &JsValue) -> JsValue {
        use JsValue::*;
        match (self, other) {
            (Number(self_num), Number(other_num)) => {
                JsValue::Number(self_num + other_num)
            }
            _ => unimplemented!()
        }
    }
}

// ----------------------------------------------------------
// END OF PRELUDE
// ----------------------------------------------------------

