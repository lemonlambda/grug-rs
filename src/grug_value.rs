use std::{
    any::Any,
    collections::HashMap,
    ffi::{CString, c_void},
};

pub enum GrugValue<'a> {
    String(String),
    I32(i32),
    F32(f32),
    Bool(bool),
    Custom(&'a mut dyn Any),
}

/// Arguments to a grug function
///
/// # Example
/// ```
/// let mut args = Arguments::new(vec![GrugValue::String("hello, world".to_string())]);
/// grug.activate_on_function("World", "on_update", &mut Arguments::empty())?;
/// grug.activate_on_function("World", "on_argument_test", &mut args)?;
/// ```
pub struct Arguments<'a> {
    pub(crate) values: Vec<GrugValue<'a>>,
    raw_values: Option<Vec<*mut c_void>>,
    stored_c_strings: HashMap<String, CString>,
}

impl<'a> Arguments<'a> {
    pub fn new(values: Vec<GrugValue<'a>>) -> Self {
        Self {
            values,
            raw_values: None,
            stored_c_strings: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            values: vec![],
            raw_values: None,
            stored_c_strings: HashMap::new(),
        }
    }

    pub fn into_raw(&mut self) -> *mut *mut c_void {
        let mut values = vec![];

        for v in self.values.iter_mut() {
            values.push(match v {
                GrugValue::String(v) => {
                    let _ = self
                        .stored_c_strings
                        .entry(v.clone())
                        .or_insert(CString::new(v.clone()).unwrap());
                    self.stored_c_strings.get_mut(v).unwrap() as *mut _ as *mut c_void
                }
                GrugValue::I32(v) => v as *mut i32 as *mut c_void,
                GrugValue::F32(v) => v as *mut f32 as *mut c_void,
                GrugValue::Bool(v) => v as *mut bool as *mut c_void,
                GrugValue::Custom(v) => *v as *mut _ as *mut c_void,
            });
        }

        self.raw_values = Some(values);

        self.raw_values.as_mut().unwrap().as_mut_ptr()
    }
}
