
use keyboard_types::KeyboardEvent;
use secret_structs::secret::secret::SecretBlockSafe;

#[derive(Clone)]
pub struct SecKeyboardEvent {
    pub ke: KeyboardEvent
}

unsafe impl SecretBlockSafe for SecKeyboardEvent {}