#![feature(negative_impls)]

use keyboard_types::KeyboardEvent;
use secret_structs::secret::secret::SecretBlockSafe;
use secret_structs::secret::secret::*;
use secret_structs::lattice::integrity_lattice as int_lat;
use secret_structs::lattice::ternary_lattice as sec_lat;
use secret_macros::SecretBlockSafeDerive;
use serde::ser::{Serializer, SerializeStruct};
use serde::{Serialize, Deserialize, Deserializer};
use std::marker::PhantomData;
use keyboard_types::{Key, Modifiers, KeyState, Code, Location};
use malloc_size_of_derive::MallocSizeOf;
use malloc_size_of::MallocSizeOf;
use malloc_size_of::MallocSizeOfOps;

//#[derive(Clone, Default, Serialize, Deserialize)]
//pub struct SecKeyboardEvent {
//    pub ke: KeyboardEvent
//}

//unsafe impl SecretBlockSafe for SecKeyboardEvent {}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SecKeyboardEvent {
    /// Whether the key is pressed or released.
    pub state: StaticDynamicAll<KeyStateWrapper,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>,
    /// Logical key value.
    pub key: StaticDynamicAll<KeyWrapper,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>,
    /// Physical key position.
    pub code: StaticDynamicAll<CodeWrapper,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>,
    /// Location for keys with multiple instances on common keyboards.
    pub location: StaticDynamicAll<LocationWrapper,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>,
    /// Flags for pressed modifier keys.
    pub modifiers: StaticDynamicAll<ModifiersWrapper,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>,
    /// True if the key is currently auto-repeated.
    pub repeat: StaticDynamicAll<bool,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>,
    /// Events with this flag should be ignored in a text editor
    /// and instead composition events should be used.
    pub is_composing: StaticDynamicAll<bool,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct KeyStateWrapper {
    pub k: KeyState
}

unsafe impl SecretBlockSafe for KeyStateWrapper {}

#[derive(Clone, Default, Serialize, Deserialize, MallocSizeOf)]
pub struct KeyWrapper {
    pub k: Key
}

unsafe impl SecretBlockSafe for KeyWrapper {}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LocationWrapper {
    pub l: Location
}

unsafe impl SecretBlockSafe for LocationWrapper {}

#[derive(Clone, Default, Serialize, Deserialize, MallocSizeOf)]
pub struct ModifiersWrapper {
    pub m: Modifiers
}

unsafe impl SecretBlockSafe for ModifiersWrapper {}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CodeWrapper {
    pub c: Code
}

unsafe impl SecretBlockSafe for CodeWrapper {}

#[derive(Clone, SecretBlockSafeDerive, MallocSizeOf)]
pub struct PreDOMString {
    pub s: String
}

pub struct CellWrapper<T> {
    pub c: std::cell::Cell<T>
}

impl<T: MallocSizeOf + Copy + SecretValueSafe, L1: sec_lat::Label, L2: sec_lat::Label> MallocSizeOf for CellWrapper<InfoFlowStruct<T, L1, L2, DynamicSecretLabel, DynamicIntegrityLabel>> {
    fn size_of(&self, ops: &mut MallocSizeOfOps) -> usize {
        self.c.get().unwrap_unsafe_dynamic_all::<L1, L2>().size_of(ops)
    }
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