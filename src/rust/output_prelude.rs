use std::{collections::HashMap, ops::DerefMut, rc::Rc};

mod js_cell {
    use std::{
        cell::UnsafeCell,
        marker::PhantomData,
        ops::{Deref, DerefMut},
        ptr::NonNull,
    };

    /// Implements RefCell like behaviour but without checking ownership rules during runtime.
    ///
    /// This may be completely invalid and may cause undefined behaviour,
    /// so I may need to replace this with RefCell, if strange behaviour is found during runtime
    ///

    pub struct JsCell<T> {
        value: UnsafeCell<T>,
    }
    impl<T> JsCell<T> {
        pub fn new(value: T) -> Self {
            JsCell {
                value: UnsafeCell::new(value),
            }
        }
        pub fn borrow(&self) -> &T {
            unsafe { &*self.value.get() }
        }

        pub fn borrow_mut<'a>(&'a self) -> RefMut<'a, T> {
            let value = unsafe { NonNull::new_unchecked(self.value.get()) };
            RefMut {
                value,
                marker: PhantomData,
            }
        }
    }

    pub struct RefMut<'a, T: ?Sized> {
        value: NonNull<T>,
        marker: PhantomData<&'a T>,
    }

    impl<T: ?Sized> Deref for RefMut<'_, T> {
        type Target = T;

        #[inline]
        fn deref(&self) -> &T {
            // SAFETY: the value is accessible as long as we hold our borrow.
            unsafe { self.value.as_ref() }
        }
    }

    impl<T: ?Sized> DerefMut for RefMut<'_, T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut T {
            // SAFETY: the value is accessible as long as we hold our borrow.
            unsafe { self.value.as_mut() }
        }
    }
}

use js_cell::JsCell;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct JsString {
    // TODO: Replace with something that can be used to represent UTF16 strings efficiently
    // (strings in JS behave as UTF16 strings)
    //
    // An idea that I have is to use SSO, such that the inlined string is UTF8 so that
    // it fits more chars and the outlined (long) strings are UTF16 so that we save
    // processing power by avoiding the conversion between UTF16 and UTF8
    // (because JS string operations all treat the string as if it was UTF16)
    //
    value: Rc<str>,
}

impl<'a> From<&'a str> for JsString {
    #[inline]
    fn from(value: &'a str) -> Self {
        JsString {
            value: Rc::from(value),
        }
    }
}

impl JsString {
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

enum ObjectSubtype {
    RegularObject,
    Function(Box<dyn Fn(&[JsValue])>),
    Array(Vec<JsValue>),
}

pub struct JsObjectContents {
    // TODO: replace this with VecMap, (or ArrayMap, I'm still not sure about the name)
    // a map that stores all key-value pairs (maybe up until a certain amount)
    // in a Vec or array. (Because I THINK that most objects contain few keys,
    // so it might help performance to store them in contiguous memory)
    properties: HashMap<JsString, JsValue>,

    /// Subtype is a bit of a hack/cheat.
    /// It is used to help handling callable objects (aka functions) and arrays.
    ///
    /// Without this, it would need a lot of extra work to implement arrays through "just" an
    /// object (in particular because of the length property for example)
    subtype: ObjectSubtype,
}

pub type JsObject = Rc<JsCell<JsObjectContents>>;

#[derive(Clone)]
pub enum JsValue {
    Null,
    Undefined,
    Boolean(bool),
    Number(f64),
    String(JsString),
    Object(JsObject),
}

impl From<f64> for JsValue {
    fn from(value: f64) -> Self {
        JsValue::Number(value)
    }
}

impl From<usize> for JsValue {
    fn from(value: usize) -> Self {
        JsValue::Number(value as f64)
    }
}

impl JsValue {
    pub fn add(&self, other: &JsValue) -> JsValue {
        use JsValue::*;
        match (self, other) {
            (Number(self_num), Number(other_num)) => JsValue::Number(self_num + other_num),
            _ => unimplemented!(),
        }
    }

    pub fn get_prop(&self, name: JsValue) -> JsValue {
        match self {
            JsValue::Undefined => {
                panic!(
                    "Cannot read properties of undefined, reading '{}'",
                    name.to_js_string().as_str()
                );
            }
            JsValue::Object(obj) => {
                let obj = obj.borrow();
                if let ObjectSubtype::Array(ref array) = obj.subtype {
                    match name {
                        JsValue::Number(index) => {
                            assert_eq!(index, index.round());
                            let index = index as usize;
                            return array[index].clone();
                        }
                        JsValue::String(s) if s == JsString::from("length") => {
                            return array.len().into();
                        }
                        _ => unimplemented!(),
                    }
                }
                return obj
                    .properties
                    .get(&name.to_js_string())
                    .unwrap_or(&JsValue::Undefined)
                    .clone();
            }
            _ => unimplemented!(),
        }
    }

    pub fn set_prop(&self, name: JsValue, value: JsValue) {
        match self {
            JsValue::Object(obj) => {
                let mut obj = obj.borrow_mut();
                if let ObjectSubtype::Array(ref mut array) = obj.subtype {
                    match name {
                        JsValue::Number(index) => {
                            assert_eq!(index, index.round());
                            let index = index as usize;
                            array[index] = value;
                            return;
                        }
                        _ => unimplemented!(),
                    }
                }
                obj.properties.insert(name.to_js_string(), value);
            }
            _ => unimplemented!(),
        }
    }

    /// The ubiquitous `toString` function from JS
    pub fn to_js_string(&self) -> JsString {
        match self {
            JsValue::Null => JsString::from("null"),
            JsValue::Undefined => JsString::from("undefined"),
            JsValue::Boolean(val) => JsString {
                value: Rc::from(format!("{val}")),
            },
            JsValue::Number(val) => JsString {
                value: Rc::from(format!("{val}")),
            },
            JsValue::String(val) => val.clone(),
            JsValue::Object(_) => JsString::from("[object Object]"),
        }
    }
}

// ----------------------------------------------------------
// END OF PRELUDE
// ----------------------------------------------------------

