
use keyboard_types::KeyboardEvent;
use secret_structs::secret::secret::SecretBlockSafe;
use serde::ser::{Serializer, SerializeStruct};
use serde::{Serialize, Deserialize, Deserializer};

#[derive(Clone, Serialize, Deserialize)]
pub struct SecKeyboardEvent {
    pub ke: KeyboardEvent
}

unsafe impl SecretBlockSafe for SecKeyboardEvent {}