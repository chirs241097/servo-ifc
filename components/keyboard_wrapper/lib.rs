#![feature(negative_impls)]

use keyboard_types::KeyboardEvent;
use secret_structs::secret::secret::SecretBlockSafe;
use secret_macros::SecretBlockSafeDerive;
use serde::ser::{Serializer, SerializeStruct};
use serde::{Serialize, Deserialize, Deserializer};
use std::marker::PhantomData;
use keyboard_types::{Key, Modifiers};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SecKeyboardEvent {
    pub ke: KeyboardEvent
}

unsafe impl SecretBlockSafe for SecKeyboardEvent {}

#[derive(SecretBlockSafeDerive)]
pub struct PreDOMString {
    s: String
}
pub struct SecurePart {
    pub type_: PreDOMString, //this
    pub key: Key, //this
    pub code: PreDOMString, //this
    pub location: u32, //this
    pub repeat: bool, //this
    pub is_composing: bool, //this
    pub modifiers: Modifiers, //this
    pub char_code: u32, //this
    pub key_code: u32, //this
}
unsafe impl SecretBlockSafe for SecurePart {}