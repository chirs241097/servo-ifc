/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::KeyboardEventBinding;
use crate::dom::bindings::codegen::Bindings::KeyboardEventBinding::KeyboardEventMethods;
use crate::dom::bindings::codegen::Bindings::UIEventBinding::UIEventMethods;
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::reflect_dom_object;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::event::Event;
use crate::dom::uievent::UIEvent;
use crate::dom::window::Window;
use dom_struct::dom_struct;
use keyboard_types::{Key, Modifiers};
//use std::cell::Cell;

//Carapace: Added imports
use keyboard_wrapper::*;
use secret_structs::ternary_lattice as sec_lat;
use secret_structs::integrity_lattice as int_lat;
use secret_structs::untrusted_secure_block_dynamic_all;
use secret_structs::secret::{StaticDynamicAll,DynLabel, *};
//use secret_macros::SecretBlockSafeDerive;

unsafe_no_jsmanaged_fields!(Key);
unsafe_no_jsmanaged_fields!(Modifiers);

#[dom_struct]
pub struct KeyboardEvent {
    //TODO: Make sure DomRefCell is the only option, and we can't in fact use Cell
    uievent: UIEvent,
    key: DomRefCell</*StaticDynamicAll<DOMString, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>*//**/ServoSecureDynamic<DOMString>/**/>,
    typed_key: DomRefCell</*StaticDynamicAll<KeyWrapper, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>*//**/ServoSecureDynamic<KeyWrapper>/**/>,
    code: DomRefCell</*StaticDynamicAll<DOMString, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>*//**/ServoSecureDynamic<DOMString>/**/>,
    location: DomRefCell</*StaticDynamicAll<u32, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>*//**/ServoSecureDynamic<u32>/**/>, //initially Cell
    modifiers: DomRefCell</*StaticDynamicAll<ModifiersWrapper, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>*//**/ServoSecureDynamic<ModifiersWrapper>/**/>, //initially Cell
    repeat: DomRefCell</*StaticDynamicAll<bool, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>*//**/ServoSecureDynamic<bool>/**/>, //initially Cell
    is_composing: DomRefCell</*StaticDynamicAll<bool, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>*//**/ServoSecureDynamic<bool>/**/>, //initially Cell
    char_code: DomRefCell</*StaticDynamicAll<u32, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>*//**/ServoSecureDynamic<u32>/**/>, //initially Cell
    key_code: DomRefCell</*StaticDynamicAll<u32, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>*//**/ServoSecureDynamic<u32>/**/>, //initially Cell
}

impl KeyboardEvent {
    pub fn get_modifiers(&self) -> ServoSecureDynamic<ModifiersWrapper> {
        self.modifiers.borrow().clone()
    }
    pub fn get_typed_key(&self) -> ServoSecureDynamic<KeyWrapper> {
        self.typed_key.borrow().clone()
    }
}

/*#[derive(SecretBlockSafeDerive)]
pub struct Secure2 {
    type_arg: DOMString,
    key_arg: DOMString,
    location_arg: u32,
    repeat: bool
}*/

impl KeyboardEvent {
    fn new_inherited() -> KeyboardEvent {
        KeyboardEvent {
            uievent: UIEvent::new_inherited(),
            key: DomRefCell::new(untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, DynField::<Sec>::generate_dynamic_label(&()), DynField::<Int>::generate_dynamic_label(&()), { wrap(DOMString::new()) })),
            typed_key: DomRefCell::new({
                let k2 = KeyWrapper{k: Key::Unidentified}; 
                untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, DynField::<Sec>::generate_dynamic_label(&()), DynField::<Int>::generate_dynamic_label(&()), { wrap(k2) })
            }),
            code: DomRefCell::new(untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, DynField::<Sec>::generate_dynamic_label(&()), DynField::<Int>::generate_dynamic_label(&()), { wrap(DOMString::new()) })),
            location: DomRefCell::new(untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, DynField::<Sec>::generate_dynamic_label(&()), DynField::<Int>::generate_dynamic_label(&()), { wrap(0) })),
            modifiers: DomRefCell::new({
                let m2 = ModifiersWrapper{m: Modifiers::empty()}; 
                untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, DynField::<Sec>::generate_dynamic_label(&()), DynField::<Int>::generate_dynamic_label(&()), { wrap(m2) })
            }),
            repeat: DomRefCell::new(untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, DynField::<Sec>::generate_dynamic_label(&()), DynField::<Int>::generate_dynamic_label(&()), { wrap(false) })),
            is_composing: DomRefCell::new(untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, DynField::<Sec>::generate_dynamic_label(&()), DynField::<Int>::generate_dynamic_label(&()), { wrap(false) })),
            char_code: DomRefCell::new(untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, DynField::<Sec>::generate_dynamic_label(&()), DynField::<Int>::generate_dynamic_label(&()), { wrap(0) })),
            key_code: DomRefCell::new(untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, DynField::<Sec>::generate_dynamic_label(&()), DynField::<Int>::generate_dynamic_label(&()), { wrap(0) }))
        }
    }

    pub fn new_uninitialized(window: &Window) -> DomRoot<KeyboardEvent> {
        reflect_dom_object(Box::new(KeyboardEvent::new_inherited()), window)
    }
    //#[info_leak_free_full]
    pub fn new(
        window: &Window,
        //type_: DOMString, //this
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        _detail: i32,
        //key: Key, //this
        //code: DOMString, //this
        //location: u32, //this
        //repeat: bool, //this
        //is_composing: bool, //this
        //modifiers: Modifiers, //this
        //char_code: u32, //this
        //key_code: u32, //this
        secure: StaticDynamicAll<SecurePart<DOMString>,sec_lat::Label_Empty,int_lat::Label_All,DynLabel<Sec>,DynLabel<Int>>
    ) -> DomRoot<KeyboardEvent> { 

        let type_ = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, secure.get_dyn_sec_label_ref(), secure.get_dyn_int_label_ref(), {
            let unwrapped = unwrap_ref(&secure);
            //DOMString::from_string(std::string::String::clone(DOMString::to_string_ref(&unwrapped.type)))
            wrap(DOMString::from_string(std::string::String::clone(DOMString::to_string_ref(&unwrapped.type_))))
            //wrap(std::clone::Clone::clone(&unwrapped.type_))
        });
        let key: ServoSecureDynamic<KeyWrapper> = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, secure.get_dyn_sec_label_ref(), secure.get_dyn_int_label_ref(), {
            let unwrapped: &SecurePart<DOMString> = unwrap_ref(&secure);
            wrap(custom_clone_key_wrapper(&unwrapped.key))
        });
        let code = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, secure.get_dyn_sec_label_ref(), secure.get_dyn_int_label_ref(), {
            let unwrapped = unwrap_ref(&secure);
            wrap(DOMString::from_string(std::string::String::clone(DOMString::to_string_ref(&unwrapped.code))))
        });
        let location = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, secure.get_dyn_sec_label_ref(), secure.get_dyn_int_label_ref(), {
            let unwrapped = unwrap_ref(&secure);
            wrap(unwrapped.location)
        });
        let repeat = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, secure.get_dyn_sec_label_ref(), secure.get_dyn_int_label_ref(), {
            let unwrapped = unwrap_ref(&secure);
            wrap(unwrapped.repeat)
        });
        let is_composing = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, secure.get_dyn_sec_label_ref(), secure.get_dyn_int_label_ref(), {
            let unwrapped = unwrap_ref(&secure);
            wrap(unwrapped.is_composing)
        });
        let modifiers = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, secure.get_dyn_sec_label_ref(), secure.get_dyn_int_label_ref(), {
            let unwrapped = unwrap_ref(&secure);
            wrap(custom_clone_modifiers_wrapper(&unwrapped.modifiers))
        });
        let char_code = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, secure.get_dyn_sec_label_ref(), secure.get_dyn_int_label_ref(), {
            let unwrapped = unwrap_ref(&secure);
            wrap(unwrapped.char_code)
        });
        let key_code = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, secure.get_dyn_sec_label_ref(), secure.get_dyn_int_label_ref(), {
            let unwrapped = unwrap_ref(&secure);
            wrap(unwrapped.key_code)
        });
        /*let secure_2 = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All {
            let unwrapped = unwrap_ref(&secure);
            let result = Secure2 {
                type_arg: unwrapped.type_,
                key_arg: DOMString{s: unwrapped.key.to_string()},
                location_arg: unwrapped.location,
                repeat: unwrapped.repeat
            };
            wrap(result);
        });*/
        let key_to_string: ServoSecureDynamic<DOMString> = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, secure.get_dyn_sec_label_ref(), secure.get_dyn_int_label_ref(), {
            let unwrapped_s: &SecurePart<DOMString> = unwrap_ref(&secure);
            let a = keyboard_wrapper::to_string(&unwrapped_s.key);
            wrap(DOMString::from_string(a))
        });
        let ev = KeyboardEvent::new_uninitialized(window);
        ev.InitKeyboardEvent2(
            type_, //this
            can_bubble,
            cancelable,
            view,
            key_to_string, //DOMString::from(key.to_string()), //this
            location, //this
            DOMString::new(),
            repeat, //this
            DOMString::new(),
        );
        *ev.typed_key.borrow_mut() = key;
        *ev.code.borrow_mut() = code;
        //Carapace: Changed below function calls to use DomRefCell API instead of Cell
        *ev.modifiers.borrow_mut() = modifiers;
        *ev.is_composing.borrow_mut() = is_composing;
        *ev.char_code.borrow_mut() = char_code;
        *ev.key_code.borrow_mut() = key_code;
        ev
    }


    //Carapace: Removed this function to see what other compile errors come up because this function interacts with no other code I could find
    
    #[allow(non_snake_case)]
    pub fn Constructor(
        window: &Window,
        type_: DOMString,
        init: &KeyboardEventBinding::KeyboardEventInit,
    ) -> Fallible<DomRoot<KeyboardEvent>> {
        let mut modifiers = Modifiers::empty();
        modifiers.set(Modifiers::CONTROL, init.parent.ctrlKey);
        modifiers.set(Modifiers::ALT, init.parent.altKey);
        modifiers.set(Modifiers::SHIFT, init.parent.shiftKey);
        modifiers.set(Modifiers::META, init.parent.metaKey);
        //Carapace: Created new SecurePart in order to compensate for the modified funciton signature.
        let result: SecurePart<DOMString> = SecurePart{
            type_: DOMString::from_string(std::string::String::from(type_)),
            key: KeyWrapper{k: Key::Unidentified},
            code: DOMString::from_string(std::string::String::from(std::clone::Clone::clone(&init.code))),
            location: init.location,
            repeat: init.repeat,
            is_composing: init.isComposing,
            modifiers: ModifiersWrapper{m: modifiers},
            char_code: 0,
            key_code: 0
        };
        let s: DynLabel<Sec> = DynLabel::<Sec>::new_default();
        let i: DynLabel<Int> = DynLabel::<Int>::new_default();
        let secure_1 = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, &s, &i,  {
            wrap(result)
        });
        let event = KeyboardEvent::new(
            window,
            //type_,
            init.parent.parent.parent.bubbles,
            init.parent.parent.parent.cancelable,
            init.parent.parent.view.as_deref(),
            init.parent.parent.detail,
            secure_1
            //Key::Unidentified,
            //init.code.clone(),
            //init.location,
            //init.repeat,
            //init.isComposing,
            //modifiers,
            //0,
            //0,
        );
        let s = DOMString::from_string(init.key.clone().to_string());
        *event.key.borrow_mut() = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, DynField::<Sec>::generate_dynamic_label(&()), DynField::<Int>::generate_dynamic_label(&()), { 
            wrap(s) 
        });
        Ok(event)
    }
    
}

//Carapace: Defined copy of function to get around binding limiting type signatures
impl KeyboardEvent {
    fn InitKeyboardEvent2(
        &self,
        type_arg: ServoSecureDynamic<DOMString>,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: ServoSecureDynamic<DOMString>,
        location_arg: ServoSecureDynamic::<u32>,
        _modifiers_list_arg: DOMString,
        repeat: ServoSecureDynamic<bool>,
        _locale: DOMString,
    ) {
        if self.upcast::<Event>().dispatching() {
            return;
        }

        self.upcast::<UIEvent>()
            .InitUIEvent2(type_arg, can_bubble_arg, cancelable_arg, view_arg, 0);
        *self.key.borrow_mut() = key_arg;
        *self.location.borrow_mut() = location_arg;
        *self.repeat.borrow_mut() = repeat;
    }
}

impl KeyboardEventMethods for KeyboardEvent {
    // https://w3c.github.io/uievents/#widl-KeyboardEvent-initKeyboardEvent
    fn InitKeyboardEvent(
        &self,
        type_arg: DOMString,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: DOMString,
        location_arg: u32,
        _modifiers_list_arg: DOMString,
        repeat: bool,
        _locale: DOMString,
    ) {
        if self.upcast::<Event>().dispatching() {
            return;
        }

        //Carapace: Modified function so it compiles, but this function shouldn't ever be called.
        panic!("Carapace: Can't call method InitKeyboardEvent");
        self.upcast::<UIEvent>()
        .InitUIEvent(type_arg, can_bubble_arg, cancelable_arg, view_arg, 0);
        let ka = DOMString::from_string(key_arg.to_string());
        *self.key.borrow_mut() = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, DynField::<Sec>::generate_dynamic_label(&()), DynField::<Int>::generate_dynamic_label(&()), { 
            wrap(ka) 
        });
        *self.location.borrow_mut() = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, DynField::<Sec>::generate_dynamic_label(&()), DynField::<Int>::generate_dynamic_label(&()), { 
            wrap(location_arg) 
        });
        *self.repeat.borrow_mut() = untrusted_secure_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, DynField::<Sec>::generate_dynamic_label(&()), DynField::<Int>::generate_dynamic_label(&()), { 
            wrap(repeat) 
        });
        //self.upcast::<UIEvent>()
        //    .InitUIEvent(type_arg, can_bubble_arg, cancelable_arg, view_arg, 0);
        //*self.key.borrow_mut() = key_arg;
        //self.location.set(location_arg);
        //self.repeat.set(repeat);
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-key
    fn Key(&self) -> DOMString {
        //Carapace: Don't allow this Javascript API to release secret data.
        panic!("Carapace: Can't call method Key");
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-code
    fn Code(&self) -> DOMString {
        //Carapace: Don't allow this Javascript API to release secret data.
        panic!("Carapace: Can't call method Code");
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-location
    fn Location(&self) -> u32 {
        //Carapace: Don't allow this Javascript API to release secret data.
        panic!("Carapace: Can't call method Location");
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-ctrlKey
    fn CtrlKey(&self) -> bool {
        //Carapace: Don't allow this Javascript API to release secret data.
        panic!("Carapace: Can't call method CtrlKey");
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-shiftKey
    fn ShiftKey(&self) -> bool {
        //Carapace: Don't allow this Javascript API to release secret data.
        panic!("Carapace: Can't call method ShiftKey");
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-altKey
    fn AltKey(&self) -> bool {
        //Carapace: Don't allow this Javascript API to release secret data.
        panic!("Carapace: Can't call method AltKey");
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-metaKey
    fn MetaKey(&self) -> bool {
        //Carapace: Don't allow this Javascript API to release secret data.
        panic!("Carapace: Can't call method MetaKey");
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-repeat
    fn Repeat(&self) -> bool {
        //Carapace: Don't allow this Javascript API to release secret data.
        panic!("Carapace: Can't call method Repeat");
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-isComposing
    fn IsComposing(&self) -> bool {
        //Carapace: Don't allow this Javascript API to release secret data.
        panic!("Carapace: Can't call method IsComposing");
    }

    // https://w3c.github.io/uievents/#dom-keyboardevent-getmodifierstate
    fn GetModifierState(&self, key_arg: DOMString) -> bool {
        //Carapace: Don't allow this Javascript API to release secret data.
        panic!("Carapace: Can't call method GetModifierState");
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-charCode
    fn CharCode(&self) -> u32 {
        //Carapace: Don't allow this Javascript API to release secret data.
        panic!("Carapace: Can't call method CharCode");
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-keyCode
    fn KeyCode(&self) -> u32 {
        //Carapace: Don't allow this Javascript API to release secret data.
        panic!("Carapace: Can't call method KeyCode");
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-which
    fn Which(&self) -> u32 {
        //Carapace: Don't allow this Javascript API to release secret data.
        panic!("Carapace: Can't call method Which");
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.uievent.IsTrusted()
    }
}
