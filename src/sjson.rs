use std::collections::HashMap;
use serde::{Serialize, Serializer};

#[derive(Clone, Debug)]
pub enum SJsonValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(HashMap<String, SJsonValue>),
    Array(Vec<SJsonValue>),
}


#[derive(Clone, Debug)]
pub struct SJsonElement {
    pub id: String,
    pub params: SJsonValue
}

// impl Serialize for SJsonElement {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer
//     {
//         serializer.serialize(&self.params)
//     }
// }

impl Serialize for SJsonValue {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match &self {
            SJsonValue::String(v) => {
                s.serialize_str(v)
            }
            SJsonValue::Number(v) => {
                s.serialize_f64(*v)
            }
            SJsonValue::Boolean(v) => {
                s.serialize_bool(*v)
            }
            SJsonValue::Object(v) => {
                v.serialize(s)
            }
            SJsonValue::Array(v) => {
                v.serialize(s)
            }
        }
    }
}

pub trait TransformHashMap {
    fn transform_hashmap(&self) -> HashMap<String, SJsonValue>;
}

impl TransformHashMap for Vec<SJsonElement> {
    fn transform_hashmap(&self) -> HashMap<String, SJsonValue> {
        HashMap::<String, SJsonValue>::from_iter(
            self.clone().into_iter().map(|x| (x.id, x.params))
        )
    }
}

pub trait ToSJson {
    fn sjson(&self) -> SJsonValue;
}

impl ToSJson for String {
    fn sjson(&self) -> SJsonValue {
        SJsonValue::String(self.clone())
    }
}

impl ToSJson for &str {
    fn sjson(&self) -> SJsonValue {
        SJsonValue::String(self.to_string())
    }
}

impl ToSJson for bool {
    fn sjson(&self) -> SJsonValue {
        SJsonValue::Boolean(*self)
    }
}

trait Number {}

impl Number for f64 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for usize {}
impl Number for isize {}

impl<T> ToSJson for T
where
    T: Number + Into<f64> + Copy {
    fn sjson(&self) -> SJsonValue {
        SJsonValue::Number((*self).into())
    }
}

impl<T> ToSJson for Vec<T>
where
    T: ToSJson + Copy {
    fn sjson(&self) -> SJsonValue {
        SJsonValue::Array(self.iter().map(|x| (*x).sjson()).collect())
    }
}

impl ToSJson for SJsonValue {
    fn sjson(&self) -> SJsonValue {
        self.clone()
    }
}