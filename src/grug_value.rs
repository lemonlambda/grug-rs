use std::{
    any::Any,
    borrow::Borrow,
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

pub struct Arguments {
    pub(crate) values: Vec<GrugValue>,
    raw_values: Option<Vec<*mut c_void>>,
}

impl Arguments {
    pub fn new(values: Vec<GrugValue>) -> Self {
        Self {
            values,
            raw_values: None,
        }
    }

    pub fn empty() -> Self {
        Self {
            values: vec![],
            raw_values: None,
        }
    }

    pub fn into_raw(&mut self) -> *mut *mut c_void {
        let mut values = vec![];

        for v in self.values.clone().iter_mut() {
            values.push(match v {
                GrugValue::String(_) => todo!(),
                GrugValue::I32(v) => v as *mut i32 as *mut c_void,
                GrugValue::F32(_) => todo!(),
                GrugValue::Bool(_) => todo!(),
                GrugValue::Custom(_) => todo!(),
            });
        }

        self.raw_values = Some(values);

        self.raw_values.as_mut().unwrap().as_mut_ptr()
    }
}
