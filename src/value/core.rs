#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum Value {
    Float(f64),
    Boolean(bool),
    Nil,
    Obj(Object),
}

impl Value {
    pub fn new_string(pointer: usize) -> Self {
        Value::Obj(Object::StringObject(pointer))
    }

    pub fn is_string_object(&self) -> bool {
        matches!(self, Value::Obj(Object::StringObject(..)))
    }

    pub fn get_string_ref(&self) -> Option<usize> {
        match self {
            Value::Obj(Object::StringObject(s)) => Some(*s),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum Object {
    StringObject(usize),
}
