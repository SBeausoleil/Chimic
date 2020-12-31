use std::collections::HashMap;

pub trait Concentrate<K, V: std::ops::Add> {
    fn concentrate(&self) -> HashMap<K, V>;
}

#[macro_export]
macro_rules! impl_ConcentrateTrait {
    ($name: ty, $key: ty, $value: ty) => {
        impl crate::concentrate::Concentrate<$key, $value> for $name {
            fn concentrate(&self) -> HashMap<$key, $value> {
                let mut map: HashMap<$key, $value> = HashMap::new();
                for (key, value) in self.iter() {
                    match map.get_mut(key) {
                        Some(val) => {
                            *val += value;
                        }
                        None => {
                            map.insert(key.clone(), *value);
                        }
                    }
                }
                map
            }
        }
    };
}
