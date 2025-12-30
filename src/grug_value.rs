use std::{
    any::Any,
    collections::HashMap,
    ffi::{CString, c_void},
};

pub trait GrugAny: Any {
    fn clone_box(&self) -> Box<dyn GrugAny>;
}

impl<T> GrugAny for T
where
    T: Any + Clone,
{
    fn clone_box(&self) -> Box<dyn GrugAny> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn GrugAny> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub enum GrugValue {
    String(String),
    I32(i32),
    F32(f32),
    Bool(bool),
    Custom(Box<dyn GrugAny>),
}

impl GrugValue {
    /// Use a custom type
    ///
    /// # Example
    /// ```rs
    /// struct Cool {
    ///     hi: i32
    /// }
    ///
    /// let custom_type = GrugValue::custom(Cool { hi: 10 });
    /// ```
    pub fn custom<T: GrugAny + 'static>(value: T) -> Self {
        Self::Custom(Box::new(value))
    }
}

/// Arguments to a grug function
///
/// # Example
/// ```
/// let mut args = Arguments::new(vec![GrugValue::String("hello, world".to_string())]);
/// grug.activate_on_function("World", "on_update", &mut Arguments::empty())?;
/// grug.activate_on_function("World", "on_argument_test", &mut args)?;
/// ```
pub struct Arguments {
    pub(crate) values: Vec<GrugValue>,
    raw_values: Option<Vec<*mut c_void>>,
    stored_c_strings: HashMap<String, CString>,
}

impl Arguments {
    pub fn new(values: Vec<GrugValue>) -> Self {
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
                GrugValue::Custom(v) => (*v).as_mut() as *mut _ as *mut c_void,
            });
        }

        self.raw_values = Some(values);

        self.raw_values.as_mut().unwrap().as_mut_ptr()
    }
}
