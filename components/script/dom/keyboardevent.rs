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

//Vincent: Added imports
use keyboard_wrapper::*;
use secret_structs::ternary_lattice as sec_lat;
use secret_structs::integrity_lattice as int_lat;
use secret_structs::info_flow_block_dynamic_all;
use secret_structs::secret::{StaticDynamicAll,DynamicSecretLabel, DynamicIntegrityLabel, *};
//use secret_macros::SecretBlockSafeDerive;

unsafe_no_jsmanaged_fields!(Key);
unsafe_no_jsmanaged_fields!(Modifiers);

#[dom_struct]
pub struct KeyboardEvent {
    //TODO: Make sure DomRefCell is the only option, and we can't in fact use Cell
    uievent: UIEvent,
    key: DomRefCell</*StaticDynamicAll<PreDOMString, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>*//**/ServoSecureDynamic<PreDOMString>/**/>,
    typed_key: DomRefCell</*StaticDynamicAll<KeyWrapper, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>*//**/ServoSecureDynamic<KeyWrapper>/**/>,
    code: DomRefCell</*StaticDynamicAll<PreDOMString, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>*//**/ServoSecureDynamic<PreDOMString>/**/>,
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
    type_arg: PreDOMString,
    key_arg: PreDOMString,
    location_arg: u32,
    repeat: bool
}*/

impl KeyboardEvent {
    fn new_inherited() -> KeyboardEvent {
        KeyboardEvent {
            uievent: UIEvent::new_inherited(),
            key: DomRefCell::new(ServoSecureDynamic::<PreDOMString>::new_info_flow_struct(PreDOMString{s: String::new()}, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]))),
            typed_key: DomRefCell::new(ServoSecureDynamic::<KeyWrapper>::new_info_flow_struct(KeyWrapper{k: Key::Unidentified}, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]))),
            code: DomRefCell::new(ServoSecureDynamic::<PreDOMString>::new_info_flow_struct(PreDOMString{s: String::new()}, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]))),
            location: DomRefCell::new(ServoSecureDynamic::<u32>::new_info_flow_struct(0, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]))),
            modifiers: DomRefCell::new(ServoSecureDynamic::<ModifiersWrapper>::new_info_flow_struct(ModifiersWrapper{m: Modifiers::empty()}, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]))),
            repeat: DomRefCell::new(ServoSecureDynamic::<bool>::new_info_flow_struct(false, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]))),
            is_composing: DomRefCell::new(ServoSecureDynamic::<bool>::new_info_flow_struct(false, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]))),
            char_code: DomRefCell::new(ServoSecureDynamic::<u32>::new_info_flow_struct(0, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]))),
            key_code: DomRefCell::new(ServoSecureDynamic::<u32>::new_info_flow_struct(0, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]))),
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
        secure: StaticDynamicAll<SecurePart,sec_lat::Label_A,int_lat::Label_All,DynamicSecretLabel,DynamicIntegrityLabel>
    ) -> DomRoot<KeyboardEvent> { 

        let type_ = info_flow_block_dynamic_all!(sec_lat::Label_A, int_lat::Label_All, secure.get_dynamic_secret_label_clone(), secure.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&secure);
            wrap_secret(std::clone::Clone::clone(&unwrapped.type_))
        });
        let key: ServoSecureDynamic<KeyWrapper> = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, secure.get_dynamic_secret_label_clone(), secure.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&secure);
            wrap_secret(std::clone::Clone::clone(&unwrapped.key))
        });
        let code = info_flow_block_dynamic_all!(sec_lat::Label_A, int_lat::Label_All, secure.get_dynamic_secret_label_clone(), secure.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&secure);
            wrap_secret(std::clone::Clone::clone(&unwrapped.code))
        });
        let location = info_flow_block_dynamic_all!(sec_lat::Label_A, int_lat::Label_All, secure.get_dynamic_secret_label_clone(), secure.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&secure);
            wrap_secret(unwrapped.location)
        });
        let repeat = info_flow_block_dynamic_all!(sec_lat::Label_A, int_lat::Label_All, secure.get_dynamic_secret_label_clone(), secure.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&secure);
            wrap_secret(unwrapped.repeat)
        });
        let is_composing = info_flow_block_dynamic_all!(sec_lat::Label_A, int_lat::Label_All, secure.get_dynamic_secret_label_clone(), secure.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&secure);
            wrap_secret(unwrapped.is_composing)
        });
        let modifiers = info_flow_block_dynamic_all!(sec_lat::Label_A, int_lat::Label_All, secure.get_dynamic_secret_label_clone(), secure.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&secure);
            wrap_secret(std::clone::Clone::clone(&unwrapped.modifiers))
        });
        let char_code = info_flow_block_dynamic_all!(sec_lat::Label_A, int_lat::Label_All, secure.get_dynamic_secret_label_clone(), secure.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&secure);
            wrap_secret(unwrapped.char_code)
        });
        let key_code = info_flow_block_dynamic_all!(sec_lat::Label_A, int_lat::Label_All, secure.get_dynamic_secret_label_clone(), secure.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&secure);
            wrap_secret(unwrapped.key_code)
        });
        /*let secure_2 = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All {
            let unwrapped = unwrap_secret_ref(&secure);
            let result = Secure2 {
                type_arg: unwrapped.type_,
                key_arg: PreDOMString{s: unwrapped.key.to_string()},
                location_arg: unwrapped.location,
                repeat: unwrapped.repeat
            };
            wrap_secret(result);
        });*/
        let key_to_string: ServoSecureDynamic<PreDOMString> = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, secure.get_dynamic_secret_label_clone(), secure.get_dynamic_integrity_label_clone(), {
            let unwrapped_s = unwrap_secret_ref(&secure);
            let a = keyboard_wrapper::to_string(&unwrapped_s.key);
            wrap_secret(PreDOMString{s: a})
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
        //Vincent: Changed below function calls to use DomRefCell API instead of Cell
        *ev.modifiers.borrow_mut() = modifiers;
        *ev.is_composing.borrow_mut() = is_composing;
        *ev.char_code.borrow_mut() = char_code;
        *ev.key_code.borrow_mut() = key_code;
        ev
    }


    //Vincent: Removed this function to see what other compile errors come up because this function interacts with no other code I could find
    
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
        //Vincent: Created new SecurePart in order to compensate for the modified funciton signature.
        let result = SecurePart{
            type_: PreDOMString{s: std::string::String::from(type_)},
            key: KeyWrapper{k: Key::Unidentified},
            code: PreDOMString{s: std::string::String::from(std::clone::Clone::clone(&init.code))},
            location: init.location,
            repeat: init.repeat,
            is_composing: init.isComposing,
            modifiers: ModifiersWrapper{m: modifiers},
            char_code: 0,
            key_code: 0
        };
        let s: DynamicSecretLabel = new_dynamic_secret_label(vec![]);
        let i: DynamicIntegrityLabel = new_dynamic_integrity_label(vec![]);
        let secure_1 = info_flow_block_dynamic_all!(sec_lat::Label_A, int_lat::Label_All, s, i,  {
            wrap_secret(result)
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
        *event.key.borrow_mut() = ServoSecureDynamic::<PreDOMString>::new_info_flow_struct(PreDOMString{s: init.key.clone().to_string()}, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]));
        Ok(event)
    }
    
}

//Vincent: Defined copy of function to get around binding limiting type signatures
impl KeyboardEvent {
    fn InitKeyboardEvent2(
        &self,
        type_arg: ServoSecureDynamic<PreDOMString>,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: ServoSecureDynamic<PreDOMString>,
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

        //Vincent: Modified function so it compiles, but this function shouldn't ever be called.
        panic!("Vincent: Can't call method InitKeyboardEvent");
        self.upcast::<UIEvent>()
        .InitUIEvent(type_arg, can_bubble_arg, cancelable_arg, view_arg, 0);
        *self.key.borrow_mut() = ServoSecureDynamic::<PreDOMString>::new_info_flow_struct(PreDOMString{s: key_arg.to_string()}, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]));
        *self.location.borrow_mut() = ServoSecureDynamic::<u32>::new_info_flow_struct(location_arg, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]));
        *self.repeat.borrow_mut() = ServoSecureDynamic::<bool>::new_info_flow_struct(repeat, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]));
        //self.upcast::<UIEvent>()
        //    .InitUIEvent(type_arg, can_bubble_arg, cancelable_arg, view_arg, 0);
        //*self.key.borrow_mut() = key_arg;
        //self.location.set(location_arg);
        //self.repeat.set(repeat);
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-key
    fn Key(&self) -> DOMString {
        //Vincent: Replaced with a default value since it's secret.
        panic!("Vincent: Can't call method Key");
        /*self.key.borrow().clone()*/
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-code
    fn Code(&self) -> DOMString {
        //Vincent: Replaced with a default value since it's secret.
        panic!("Vincent: Can't call method Code");
        /*self.code.borrow().clone()*/
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-location
    fn Location(&self) -> u32 {
        //Vincent: Replaced with a default value since it's secret.
        panic!("Vincent: Can't call method Location");
        /*self.location.get()*/
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-ctrlKey
    fn CtrlKey(&self) -> bool {
        //Vincent: Replaced with a default value since it's secret.
        panic!("Vincent: Can't call method CtrlKey");
        /*self.modifiers.get().contains(Modifiers::CONTROL)*/
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-shiftKey
    fn ShiftKey(&self) -> bool {
        //Vincent: Replaced with a default value since it's secret.
        panic!("Vincent: Can't call method ShiftKey");
        /*self.modifiers.get().contains(Modifiers::SHIFT)*/
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-altKey
    fn AltKey(&self) -> bool {
        //Vincent: Replaced with a default value since it's secret.
        panic!("Vincent: Can't call method AltKey");
        /*self.modifiers.get().contains(Modifiers::ALT)*/
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-metaKey
    fn MetaKey(&self) -> bool {
        //Vincent: Replaced with a default value since it's secret.
        panic!("Vincent: Can't call method MetaKey");
        /*self.modifiers.get().contains(Modifiers::META)*/
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-repeat
    fn Repeat(&self) -> bool {
        //Vincent: Replaced with a default value since it's secret.
        panic!("Vincent: Can't call method Repeat");
        /*self.repeat.get()*/
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-isComposing
    fn IsComposing(&self) -> bool {
        //Vincent: Replaced with a default value since it's secret.
        panic!("Vincent: Can't call method IsComposing");
        /*self.is_composing.get()*/
    }

    // https://w3c.github.io/uievents/#dom-keyboardevent-getmodifierstate
    fn GetModifierState(&self, key_arg: DOMString) -> bool {
        //Vincent: Replaced with a default value since it's secret.
        panic!("Vincent: Can't call method GetModifierState");
        /*self.modifiers.get().contains(match &*key_arg {
            "Alt" => Modifiers::ALT,
            "AltGraph" => Modifiers::ALT_GRAPH,
            "CapsLock" => Modifiers::CAPS_LOCK,
            "Control" => Modifiers::CONTROL,
            "Fn" => Modifiers::FN,
            "FnLock" => Modifiers::FN_LOCK,
            "Meta" => Modifiers::META,
            "NumLock" => Modifiers::NUM_LOCK,
            "ScrollLock" => Modifiers::SCROLL_LOCK,
            "Shift" => Modifiers::SHIFT,
            "Symbol" => Modifiers::SYMBOL,
            "SymbolLock" => Modifiers::SYMBOL_LOCK,
            _ => return false,
        })*/
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-charCode
    fn CharCode(&self) -> u32 {
        //Vincent: Replaced with a default value since it's secret.
        panic!("Vincent: Can't call method CharCode");
        /*self.char_code.get()*/
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-keyCode
    fn KeyCode(&self) -> u32 {
        //Vincent: Replaced with a default value since it's secret.
        panic!("Vincent: Can't call method KeyCode");
        /*self.key_code.get()*/
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-which
    fn Which(&self) -> u32 {
        //Vincent: Replaced with a default value since it's secret.
        panic!("Vincent: Can't call method Which");
        /*if self.char_code.get() != 0 {
            self.char_code.get()
        } else {
            self.key_code.get()
        }*/
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.uievent.IsTrusted()
    }
}
