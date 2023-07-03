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
use std::cell::Cell;

//Vincent: Added imports
use keyboard_wrapper::*;
use secret_structs::lattice::ternary_lattice as sec_lat;
use secret_structs::lattice::integrity_lattice as int_lat;
use secret_structs::info_flow_block_dynamic_all;
use secret_structs::info_flow_block_no_return_dynamic_all;
use secret_structs::secret::secret::SecretBlockSafe;
use secret_structs::secret::secret::{StaticDynamicAll,DynamicSecretLabel, DynamicIntegrityLabel, *};
use secret_macros::info_leak_free_full;
use secret_macros::SecretBlockSafeDerive;

unsafe_no_jsmanaged_fields!(Key);
unsafe_no_jsmanaged_fields!(Modifiers);

#[dom_struct]
pub struct KeyboardEvent {
    uievent: UIEvent,
    key: DomRefCell<StaticDynamicAll<PreDOMString,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>>,
    typed_key: DomRefCell<StaticDynamicAll<KeyWrapper,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>>,
    code: DomRefCell<StaticDynamicAll<PreDOMString,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>>,
    location: Cell<StaticDynamicAll<u32,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>>,
    modifiers: Cell<StaticDynamicAll<ModifiersWrapper,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>>,
    repeat: Cell<StaticDynamicAll<bool,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>>,
    is_composing: Cell<StaticDynamicAll<bool,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>>,
    char_code: Cell<StaticDynamicAll<u32,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>>,
    key_code: Cell<StaticDynamicAll<u32,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>>,
}

#[derive(SecretBlockSafeDerive)]
pub struct Secure2 {
    type_arg: PreDOMString,
    key_arg: PreDOMString,
    location_arg: u32,
    repeat: bool
}

impl KeyboardEvent {
    fn new_inherited() -> KeyboardEvent {
        KeyboardEvent {
            uievent: UIEvent::new_inherited(),
            key: DomRefCell::new(StaticDynamicAll::<PreDOMString,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>::new_info_flow_struct(PreDOMString{s: String::new()}, DynamicSecretLabel{policies: vec![]}, DynamicIntegrityLabel{policies: vec![]})),
            typed_key: DomRefCell::new(StaticDynamicAll::<KeyWrapper,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>::new_info_flow_struct(KeyWrapper{k: Key::Unidentified}, DynamicSecretLabel{policies: vec![]}, DynamicIntegrityLabel{policies: vec![]})),
            code: DomRefCell::new(StaticDynamicAll::<PreDOMString,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>::new_info_flow_struct(PreDOMString{s: String::new()}, DynamicSecretLabel{policies: vec![]}, DynamicIntegrityLabel{policies: vec![]})),
            location: Cell::new(StaticDynamicAll::<u32,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>::new_info_flow_struct(0, DynamicSecretLabel{policies: vec![]}, DynamicIntegrityLabel{policies: vec![]})),
            modifiers: Cell::new(StaticDynamicAll::<ModifiersWrapper,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>::new_info_flow_struct(ModifiersWrapper{m: Modifiers::empty()}, DynamicSecretLabel{policies: vec![]}, DynamicIntegrityLabel{policies: vec![]})),
            repeat: Cell::new(StaticDynamicAll::<bool,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>::new_info_flow_struct(false, DynamicSecretLabel{policies: vec![]}, DynamicIntegrityLabel{policies: vec![]})),
            is_composing: Cell::new(StaticDynamicAll::<bool,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>::new_info_flow_struct(false, DynamicSecretLabel{policies: vec![]}, DynamicIntegrityLabel{policies: vec![]})),
            char_code: Cell::new(StaticDynamicAll::<u32,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>::new_info_flow_struct(0, DynamicSecretLabel{policies: vec![]}, DynamicIntegrityLabel{policies: vec![]})),
            key_code: Cell::new(StaticDynamicAll::<u32,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>::new_info_flow_struct(0, DynamicSecretLabel{policies: vec![]}, DynamicIntegrityLabel{policies: vec![]})),
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
        secure: StaticDynamicAll<SecurePart,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>
        //keyboard_event_2: StaticDynamicAll<SecurePart,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>
    ) -> DomRoot<KeyboardEvent> { 
        let secure_2 = info_flow_block_dynamic_all!(sec_lat::None, int_lat::All {
            let unwrapped = u(&secure);
            let result = Secure2 {
                type_arg: unwrapped.type_,
                key_arg: PreDOMString{s: unwrapped.key.to_string()},
                location_arg: unwrapped.location,
                repeat: unwrapped.repeat
            };
            sec(result);
        });
        let ev = KeyboardEvent::new_uninitialized(window);
        ev.InitKeyboardEvent2(
            //type_, //this
            can_bubble,
            cancelable,
            view,
            //DOMString::from(key.to_string()), //this
            //location, //this
            DOMString::new(),
            //repeat, //this
            DOMString::new(),
            secure_2
        );
        *ev.typed_key.borrow_mut() = key;
        *ev.code.borrow_mut() = code;
        ev.modifiers.set(modifiers);
        ev.is_composing.set(is_composing);
        ev.char_code.set(char_code);
        ev.key_code.set(key_code);
        ev.secure_part = keyboard_event_2;
        ev
    }

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
        let secure_1 = info_flow_block_dynamic_all!(sec_lat::None, int_lat::All {
            let result = SecurePart{
                type_: PreDOMString{s: String::from(type_)},
                key: Key::Unidentified,
                code: PreDOMString{s: String::from(init.code)},
                location: init.location,
                repeat: init.repeat,
                is_composing: init.isComposing,
                modifiers: modifiers,
                char_code: 0,
                key_code: 0
            };
            sec(result);
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
        *event.key.borrow_mut() = init.key.clone();
        Ok(event)
    }
}

impl KeyboardEvent {
    pub fn key(&self) -> Key {
        self.typed_key.borrow().clone()
    }

    pub fn modifiers(&self) -> Modifiers {
        self.modifiers.get()
    }
    //Added secondary impl to get around the bindings problem.
    fn InitKeyboardEvent2(
        &self,
        //type_arg: DOMString,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        //key_arg: DOMString,
        //location_arg: u32,
        _modifiers_list_arg: DOMString,
        //repeat: bool,
        _locale: DOMString,
        secure2: StaticDynamicAll<Secure2,sec_lat::None,int_lat::All,DynamicSecretLabel,DynamicIntegrityLabel>
    ) {
        if self.upcast::<Event>().dispatching() {
            return;
        }
        let type_arg = info_flow_block_dynamic_all!(sec_lat::None, int_lat::All {
            let unwrapped = u(&secure2);
            sec(unwrapped.type_arg)
        });

        self.upcast::<UIEvent>()
            .InitUIEvent2(type_arg, can_bubble_arg, cancelable_arg, view_arg, 0);
        *self.key.borrow_mut() = key_arg;
        self.location.set(location_arg);
        self.repeat.set(repeat);
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

        self.upcast::<UIEvent>()
            .InitUIEvent(type_arg, can_bubble_arg, cancelable_arg, view_arg, 0);
        *self.key.borrow_mut() = key_arg;
        self.location.set(location_arg);
        self.repeat.set(repeat);
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-key
    fn Key(&self) -> DOMString {
        self.key.borrow().clone()
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-code
    fn Code(&self) -> DOMString {
        self.code.borrow().clone()
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-location
    fn Location(&self) -> u32 {
        self.location.get()
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-ctrlKey
    fn CtrlKey(&self) -> bool {
        self.modifiers.get().contains(Modifiers::CONTROL)
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-shiftKey
    fn ShiftKey(&self) -> bool {
        self.modifiers.get().contains(Modifiers::SHIFT)
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-altKey
    fn AltKey(&self) -> bool {
        self.modifiers.get().contains(Modifiers::ALT)
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-metaKey
    fn MetaKey(&self) -> bool {
        self.modifiers.get().contains(Modifiers::META)
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-repeat
    fn Repeat(&self) -> bool {
        self.repeat.get()
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-isComposing
    fn IsComposing(&self) -> bool {
        self.is_composing.get()
    }

    // https://w3c.github.io/uievents/#dom-keyboardevent-getmodifierstate
    fn GetModifierState(&self, key_arg: DOMString) -> bool {
        self.modifiers.get().contains(match &*key_arg {
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
        })
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-charCode
    fn CharCode(&self) -> u32 {
        self.char_code.get()
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-keyCode
    fn KeyCode(&self) -> u32 {
        self.key_code.get()
    }

    // https://w3c.github.io/uievents/#widl-KeyboardEvent-which
    fn Which(&self) -> u32 {
        if self.char_code.get() != 0 {
            self.char_code.get()
        } else {
            self.key_code.get()
        }
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.uievent.IsTrusted()
    }
}
