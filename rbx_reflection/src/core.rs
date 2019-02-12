use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{
    types::{RbxInstanceClass, RbxEnum},
    dump::{generate_classes, generate_enums},
};

lazy_static! {
    static ref CLASSES: HashMap<&'static str, RbxInstanceClass> = generate_classes();
    static ref ENUMS: HashMap<&'static str, RbxEnum> = generate_enums();
}

pub fn get_classes() -> &'static HashMap<&'static str, RbxInstanceClass> {
    &CLASSES
}

pub fn get_enums() -> &'static HashMap<&'static str, RbxEnum> {
    &ENUMS
}