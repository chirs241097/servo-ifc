#![feature(negative_impls)]

use keyboard_types::KeyboardEvent;
use secret_structs::secret::secret::SecretBlockSafe;
use secret_macros::SecretBlockSafeDerive;
use serde::ser::{Serializer, SerializeStruct};
use serde::{Serialize, Deserialize, Deserializer};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SecKeyboardEvent {
    pub ke: KeyboardEvent
}

unsafe impl SecretBlockSafe for SecKeyboardEvent {}