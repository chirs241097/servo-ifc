/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Common handling of keyboard input and state management for text input controls

use crate::clipboard_provider::ClipboardProvider;
use crate::dom::bindings::str::*;
use crate::dom::compositionevent::CompositionEvent;
use crate::dom::keyboardevent::KeyboardEvent;
use keyboard_types::{Key, KeyState, Modifiers, ShortcutMatcher};
use std::borrow::ToOwned;
use std::cmp::min;
use std::default::Default;
use std::ops::{Add, AddAssign, Range};
use std::usize;
use unicode_segmentation::UnicodeSegmentation;


use keyboard_wrapper::*;
use secret_structs::info_flow_block_declassify_dynamic_all;
use secret_structs::info_flow_block_dynamic_all;
use secret_structs::info_flow_block_no_return_dynamic_all;
use secret_structs::secret::*;
use secret_structs::integrity_lattice as int_lat;
use secret_structs::ternary_lattice as sec_lat;
use secret_macros::InvisibleSideEffectFreeDerive;
use secret_macros::side_effect_free_attr_full;

#[derive(Clone, Copy, PartialEq)]
pub enum Selection {
    Selected,
    NotSelected,
}

#[derive(Clone, Copy, Debug, JSTraceable, MallocSizeOf, PartialEq)]
pub enum SelectionDirection {
    Forward,
    Backward,
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, JSTraceable, MallocSizeOf, Ord, PartialEq, PartialOrd)]
pub struct UTF8Bytes{pub value: usize}
//Vincent: impl secretblocksafe for type
unsafe impl InvisibleSideEffectFree for UTF8Bytes {}
impl UTF8Bytes {
    #[side_effect_free_attr_full(method)]
    pub fn zero() -> UTF8Bytes {
        UTF8Bytes{value: 0}
    }

    #[side_effect_free_attr_full(method)]
    pub fn one() -> UTF8Bytes {
        UTF8Bytes{value: 1}
    }

    pub fn unwrap_range(byte_range: Range<UTF8Bytes>) -> Range<usize> {
        byte_range.start.value..byte_range.end.value
    }

    pub fn saturating_sub(self, other: UTF8Bytes) -> UTF8Bytes {
        /*let conditional = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, other.get_dynamic_secret_label_clone(), other.get_dynamic_integrity_label_clone(), {
            let unwrapped = u(&other);
            self > unwrapped
        });*/
        if self > other /*conditional*/ {
            /*info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, other.get_dynamic_secret_label_clone(), other.get_dynamic_integrity_label_clone(), {
                let result = self.value - u(&other).value;
                wrap_secret(UTF8Bytes{value: result})
            })*/
            UTF8Bytes{value: self.value - other.value}
        } else {
            /*info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, other.get_dynamic_secret_label_clone(), other.get_dynamic_integrity_label_clone(), {
                wrap_secret(UTF8Bytes::zero())
            })*/
            UTF8Bytes::zero()
        }
    }
}

#[side_effect_free_attr_full]
pub fn add_utf8(first: UTF8Bytes, second: UTF8Bytes) -> UTF8Bytes {
    UTF8Bytes{value: first.value + second.value}
}

#[side_effect_free_attr_full]
pub fn greater_utf8(first: &UTF8Bytes, second: &UTF8Bytes) -> bool {
    first.value > second.value
}

impl Add for UTF8Bytes {
    type Output = UTF8Bytes;

    fn add(self, other: UTF8Bytes) -> UTF8Bytes {
        UTF8Bytes{value: self.value + other.value}
    }
}

impl AddAssign for UTF8Bytes {
    fn add_assign(&mut self, other: UTF8Bytes) {
        *self = UTF8Bytes{value: self.value + other.value}
    }
}

trait StrExt {
    fn len_utf8(&self) -> UTF8Bytes;
}
impl StrExt for str {
    fn len_utf8(&self) -> UTF8Bytes {
        UTF8Bytes{value: self.len()}
    }
}

#[side_effect_free_attr_full]
fn len_utf8_str(a: &str) -> UTF8Bytes {
    UTF8Bytes{value: core::primitive::str::len(a)}
}

#[derive(Clone, Copy, Debug, Default, JSTraceable, MallocSizeOf, PartialEq, PartialOrd)]
pub struct UTF16CodeUnits{pub value: usize}
//Vincent: impl secretblocksafe for utf16
unsafe impl InvisibleSideEffectFree for UTF16CodeUnits {}

impl UTF16CodeUnits {
    #[side_effect_free_attr_full(method)]
    pub fn zero() -> UTF16CodeUnits {
        UTF16CodeUnits{value: 0}
    }

    pub fn one() -> UTF16CodeUnits {
        UTF16CodeUnits{value: 1}
    }

    pub fn saturating_sub(self, other: UTF16CodeUnits) -> UTF16CodeUnits {
        /*let conditional = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, other.get_dynamic_secret_label_clone(), other.get_dynamic_integrity_label_clone(), {
            let unwrapped = u(&other);
            self > unwrapped
        });*/
        if self > other /*conditional*/ {
            /*info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, other.get_dynamic_secret_label_clone(), other.get_dynamic_integrity_label_clone(), {
                let result = self.value - u(&other).value;
                wrap_secret(UTF16CodeUnits{value: result})
            })*/
            UTF16CodeUnits{value: self.value - other.value}
        } else {
            /*info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, other.get_dynamic_secret_label_clone(), other.get_dynamic_integrity_label_clone(), {
                wrap_secret(UTF16CodeUnits::zero())
            })*/
            UTF16CodeUnits::zero()
        }
    }
}

#[side_effect_free_attr_full]
pub fn add_utf16(first: UTF16CodeUnits, second: UTF16CodeUnits) -> UTF16CodeUnits {
    UTF16CodeUnits{value: first.value + second.value}
}

#[side_effect_free_attr_full]
pub fn greater_utf16(first: &UTF16CodeUnits, second: &UTF16CodeUnits) -> bool {
    first.value > second.value
}

impl Add for UTF16CodeUnits {
    type Output = UTF16CodeUnits;

    fn add(self, other: UTF16CodeUnits) -> UTF16CodeUnits {
        UTF16CodeUnits{value: self.value + other.value}
    }
}

impl AddAssign for UTF16CodeUnits {
    fn add_assign(&mut self, other: UTF16CodeUnits) {
        *self = UTF16CodeUnits{value: self.value + other.value}
    }
}

impl From<DOMString> for SelectionDirection {
    fn from(direction: DOMString) -> SelectionDirection {
        match direction.as_ref() {
            "forward" => SelectionDirection::Forward,
            "backward" => SelectionDirection::Backward,
            _ => SelectionDirection::None,
        }
    }
}

impl From<SelectionDirection> for DOMString {
    fn from(direction: SelectionDirection) -> DOMString {
        match direction {
            SelectionDirection::Forward => DOMString::from("forward"),
            SelectionDirection::Backward => DOMString::from("backward"),
            SelectionDirection::None => DOMString::from("none"),
        }
    }
}

#[derive(Clone, Copy, Debug, JSTraceable, MallocSizeOf, PartialEq, PartialOrd, InvisibleSideEffectFreeDerive)]
pub struct TextPoint {
    /// 0-based line number
    pub line: usize,
    /// 0-based column number in bytes
    pub index: UTF8Bytes,
}

impl TextPoint {
    /// Returns a TextPoint constrained to be a valid location within lines
    fn constrain_to(&self, lines: &[InfoFlowStruct<DOMString, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>]) -> TextPoint {
        let line = min(self.line, lines.len() - 1);

        TextPoint {
            line,
            index: min(self.index, info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, lines[line].get_dynamic_secret_label_clone(), lines[line].get_dynamic_integrity_label_clone(), {
                let unwrapped = unwrap_secret_ref(&lines[line]);
                len_utf8_str(&DOMString::to_str_ref(unwrapped))
            })/*lines[line].len_utf8()*/),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct SelectionState {
    start: TextPoint,
    end: TextPoint,
    direction: SelectionDirection,
}

/// Encapsulated state for handling keyboard input in a single or multiline text input control.
#[derive(JSTraceable, MallocSizeOf)]
pub struct TextInput<T: ClipboardProvider> {
    /// Current text input content, split across lines without trailing '\n'
    lines: Vec<ServoSecureDynamic<DOMString>>,

    /// Current cursor input point
    edit_point: TextPoint,

    /// The current selection goes from the selection_origin until the edit_point. Note that the
    /// selection_origin may be after the edit_point, in the case of a backward selection.
    selection_origin: Option<TextPoint>,
    selection_direction: SelectionDirection,

    /// Is this a multiline input?
    multiline: bool,

    #[ignore_malloc_size_of = "Can't easily measure this generic type"]
    clipboard_provider: T,

    /// The maximum number of UTF-16 code units this text input is allowed to hold.
    ///
    /// <https://html.spec.whatwg.org/multipage/#attr-fe-maxlength>
    max_length: Option<UTF16CodeUnits>,
    min_length: Option<UTF16CodeUnits>,

    /// Was last change made by set_content?
    was_last_change_by_set_content: bool,
}

/// Resulting action to be taken by the owner of a text input that is handling an event.
pub enum KeyReaction {
    TriggerDefaultAction,
    DispatchInput,
    RedrawSelection,
    Nothing,
}

impl Default for TextPoint {
    fn default() -> TextPoint {
        TextPoint {
            line: 0,
            index: UTF8Bytes::zero(),
        }
    }
}

/// Control whether this control should allow multiple lines.
#[derive(Eq, PartialEq)]
pub enum Lines {
    Single,
    Multiple,
}

/// The direction in which to delete a character.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
}

// Some shortcuts use Cmd on Mac and Control on other systems.
#[cfg(target_os = "macos")]
pub const CMD_OR_CONTROL: Modifiers = Modifiers::META;
#[cfg(not(target_os = "macos"))]
pub const CMD_OR_CONTROL: Modifiers = Modifiers::CONTROL;

/// The length in bytes of the first n characters in a UTF-8 string.
///
/// If the string has fewer than n characters, returns the length of the whole string.
/// If n is 0, returns 0
fn len_of_first_n_chars(text: &InfoFlowStruct<DOMString, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>, n: InfoFlowStruct<usize, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>) -> UTF8Bytes {
    info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, text.get_dynamic_secret_label_clone(), text.get_dynamic_integrity_label_clone(), {
        let unwrapped = core::primitive::str::char_indices(&unwrap_secret_ref(text));
        let unwrapped_n = unwrap_secret_ref(&n);


        match custom_last(custom_take_charindices(unwrapped, *unwrapped_n)) {
            Some((index, ch)) => UTF8Bytes{value: index + core::primitive::char::len_utf8(ch)},
            None => UTF8Bytes::zero(),
        }
    })
    /*match text.char_indices().take(n).last() {
        Some((index, ch)) => UTF8Bytes(index + ch.len_utf8()),
        None => UTF8Bytes::zero(),
    }*/
}

/// The length in bytes of the first n code units in a string when encoded in UTF-16. 
///
/// If the string is fewer than n code units, returns the length of the whole string.
fn len_of_first_n_code_units(text: &ServoSecureDynamic<DOMString> /*&str*/, n: UTF16CodeUnits) -> UTF8Bytes {
    /*let mut utf8_len: InfoFlowStruct<UTF8Bytes, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel> = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, text.get_dynamic_secret_label_clone(), text.get_dynamic_integrity_label_clone(), {
        wrap_secret(UTF8Bytes::zero())
    });
    let mut utf16_len = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, text.get_dynamic_secret_label_clone(), text.get_dynamic_integrity_label_clone(), {
        wrap_secret(UTF16CodeUnits::zero())
    });*/
    let utf8_len = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, text.get_dynamic_secret_label_clone(), text.get_dynamic_integrity_label_clone(), {
        let unwrapped_str = unwrap_secret_ref(&text);
        //let mut unwrapped_utf16 = unwrap_secret_mut_ref(&mut utf16_len);
        //let mut unwrapped_utf8 = unwrap_secret_mut_ref(&mut utf8_len);
        let mut utf16_val: usize = 0;
        let mut utf8_val: usize = 0;
        for c in core::primitive::str::chars(std::string::String::as_str(DOMString::to_string_ref(unwrapped_str))) {
            utf16_val += core::primitive::char::len_utf16(c);
            if utf16_val > n.value {
                break;
            }
            utf8_val += core::primitive::char::len_utf8(c);
        }
        wrap_secret(UTF8Bytes{value: utf8_val})
    });
    info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, text.get_dynamic_secret_label_clone(), text.get_dynamic_integrity_label_clone(), {
        unwrap_secret(utf8_len)
    })
    /*for c in text.chars() {
        utf16_len += UTF16CodeUnits{value: c.len_utf16()};
        if utf16_len > n {
            break;
        }
        utf8_len += UTF8Bytes{value: c.len_utf8()};
    }
    utf8_len*/
}

impl<T: ClipboardProvider> TextInput<T> {
    /// Instantiate a new text input control
    pub fn new(
        lines: Lines,
        initial: DOMString,
        clipboard_provider: T,
        max_length: Option<UTF16CodeUnits>,
        min_length: Option<UTF16CodeUnits>,
        selection_direction: SelectionDirection,
    ) -> TextInput<T> {
        //Vincent: TODO: replace every instance of new_dynamic_secret_label and new_dynamic_integrity_label
        let mut i = TextInput {
            lines: vec![],
            edit_point: Default::default(),
            selection_origin: None,
            multiline: lines == Lines::Multiple,
            clipboard_provider: clipboard_provider,
            max_length: max_length,
            min_length: min_length,
            selection_direction: selection_direction,
            was_last_change_by_set_content: true,
        };
        i.set_content(info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(initial)}));
        i
    }

    pub fn edit_point(&self) -> TextPoint {
        self.edit_point
    }

    pub fn selection_origin(&self) -> Option<TextPoint> {
        self.selection_origin
    }

    /// The selection origin, or the edit point if there is no selection. Note that the selection
    /// origin may be after the edit point, in the case of a backward selection.
    pub fn selection_origin_or_edit_point(&self) -> TextPoint {
        self.selection_origin.unwrap_or(self.edit_point)
    }

    pub fn selection_direction(&self) -> SelectionDirection {
        self.selection_direction
    }

    pub fn set_max_length(&mut self, length: Option<UTF16CodeUnits>) {
        self.max_length = length;
    }

    pub fn set_min_length(&mut self, length: Option<UTF16CodeUnits>) {
        self.min_length = length;
    }

    /// Was last edit made by set_content?
    pub fn was_last_change_by_set_content(&self) -> bool {
        self.was_last_change_by_set_content
    }

    /// Remove a character at the current editing point
    pub fn delete_char(&mut self, dir: Direction) {
        if self.selection_origin.is_none() || self.selection_origin == Some(self.edit_point) {
            self.adjust_horizontal_by_one(dir, Selection::Selected);
        }
        //Vincent: FIX LABEL
        self.replace_selection( info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {
            wrap_secret(DOMString::from_string(std::string::String::from("")))
        }) /*DOMString::new()*/ );
    }

    /// Insert a character at the current editing point
    pub fn insert_char(&mut self, ch: char) {
        self.insert_string(ch.to_string());
    }

    /// Insert a string at the current editing point
    pub fn insert_string<S: Into<String>>(&mut self, s: S) {
        if self.selection_origin.is_none() {
            self.selection_origin = Some(self.edit_point);
        }
        //Vincent: FIX LABEL  
        let s_new: String = s.into();
        self.replace_selection(info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {
            wrap_secret(DOMString::from_string(s_new))
        }) /*DOMString::from(s.into())*/);
    }

    //Vincent: added new function
    pub fn insert_secret_string<S: Into<String> + SecretValueSafe + SideEffectFree>(&mut self, s: ServoSecureDynamic<S>) {
        if self.selection_origin.is_none() {
            self.selection_origin = Some(self.edit_point);
        }
        self.replace_selection(info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, s.get_dynamic_secret_label_clone(), s.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret(s);
            wrap_secret(DOMString::from_string(unchecked_operation(std::convert::Into::<String>::into(unwrapped)/*.into()*/)))
        }) /*DOMString::from(s.into())*/);
    }

    /// The start of the selection (or the edit point, if there is no selection). Always less than
    /// or equal to selection_end(), regardless of the selection direction.
    pub fn selection_start(&self) -> TextPoint {
        match self.selection_direction {
            SelectionDirection::None | SelectionDirection::Forward => {
                self.selection_origin_or_edit_point()
            },
            SelectionDirection::Backward => self.edit_point,
        }
    }

    /// The byte offset of the selection_start()
    pub fn selection_start_offset(&self) -> UTF8Bytes {
        self.text_point_to_offset(&self.selection_start())
    }

    /// The end of the selection (or the edit point, if there is no selection). Always greater
    /// than or equal to selection_start(), regardless of the selection direction.
    pub fn selection_end(&self) -> TextPoint {
        match self.selection_direction {
            SelectionDirection::None | SelectionDirection::Forward => self.edit_point,
            SelectionDirection::Backward => self.selection_origin_or_edit_point(),
        }
    }

    /// The byte offset of the selection_end()
    pub fn selection_end_offset(&self) -> UTF8Bytes {
        self.text_point_to_offset(&self.selection_end())
    }

    /// Whether or not there is an active selection (the selection may be zero-length)
    #[inline]
    pub fn has_selection(&self) -> bool {
        self.selection_origin.is_some()
    }

    /// Returns a tuple of (start, end) giving the bounds of the current selection. start is always
    /// less than or equal to end.
    pub fn sorted_selection_bounds(&self) -> (TextPoint, TextPoint) {
        (self.selection_start(), self.selection_end())
    }

    /// Return the selection range as byte offsets from the start of the content.
    ///
    /// If there is no selection, returns an empty range at the edit point.
    pub fn sorted_selection_offsets_range(&self) -> Range<UTF8Bytes> {
        self.selection_start_offset()..self.selection_end_offset()
    }

    /// The state of the current selection. Can be used to compare whether selection state has changed.
    pub fn selection_state(&self) -> SelectionState {
        SelectionState {
            start: self.selection_start(),
            end: self.selection_end(),
            direction: self.selection_direction,
        }
    }

    // Check that the selection is valid.
    fn assert_ok_selection(&self) {
        debug!(
            "edit_point: {:?}, selection_origin: {:?}, direction: {:?}",
            self.edit_point, self.selection_origin, self.selection_direction
        );
        if let Some(begin) = self.selection_origin {
            debug_assert!(begin.line < self.lines.len());
            let lines_ref = &self.lines;
            let classified_len_utf8 = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, lines_ref[begin.line].get_dynamic_secret_label_clone(), lines_ref[begin.line].get_dynamic_integrity_label_clone(), {
                let unwrapped = unwrap_secret_ref(&lines_ref[begin.line]);
                wrap_secret(len_utf8_str(&DOMString::to_str_ref(unwrapped)))
            });
            debug_assert!(begin.index <= info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, classified_len_utf8.get_dynamic_secret_label_clone(), classified_len_utf8.get_dynamic_integrity_label_clone(), {
                unwrap_secret(classified_len_utf8)
            }) /*self.lines[begin.line].len_utf8()*/ );

            match self.selection_direction {
                SelectionDirection::None | SelectionDirection::Forward => {
                    debug_assert!(begin <= self.edit_point)
                },

                SelectionDirection::Backward => debug_assert!(self.edit_point <= begin),
            }
        }

        debug_assert!(self.edit_point.line < self.lines.len());
        let lines_ref = &self.lines;
        let line_usize = self.edit_point.line;
        let classified_len_utf8 = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, self.lines[self.edit_point.line].get_dynamic_secret_label_clone(), self.lines[self.edit_point.line].get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&lines_ref[line_usize]);
            wrap_secret(len_utf8_str(&DOMString::to_str_ref(unwrapped)))
        });
        debug_assert!(self.edit_point.index <= info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, classified_len_utf8.get_dynamic_secret_label_clone(), classified_len_utf8.get_dynamic_integrity_label_clone(), {
            unwrap_secret(classified_len_utf8)
        }) /*self.lines[self.edit_point.line].len_utf8()*/ );
    }

    pub fn get_selection_text(&self) -> Option<InfoFlowStruct<String, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>> {
        let new = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {
            wrap_secret(std::string::String::new())
        });

        let text = self.fold_selection_slices(new, |s, slice| {
            info_flow_block_no_return_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, s.get_dynamic_secret_label_clone().dynamic_union(slice.get_dynamic_secret_label_reference()), s.get_dynamic_integrity_label_clone().dynamic_intersection(slice.get_dynamic_integrity_label_reference()), {
                let mut unwrapped_mut = unwrap_secret_mut_ref(s);
                let unwrapped = unwrap_secret_ref(&slice);
                std::string::String::push_str(unwrapped_mut, unwrapped);
            });
        }/*s.push_str(slice)*/);
        let bool_check = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, text.get_dynamic_secret_label_clone(), text.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&text);
            std::string::String::is_empty(unwrapped)
        });
        if bool_check /*text.is_empty()*/ {
            return None;
        }
        Some(text)
    }

    /// The length of the selected text in UTF-16 code units.
    fn selection_utf16_len(&self) -> UTF16CodeUnits {
        let new_acc = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {
            wrap_secret(UTF16CodeUnits::zero())
        });
        //Vincent: changed function
        let result = self.fold_selection_slices(new_acc, |len, slice| {
            info_flow_block_no_return_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, len.get_dynamic_secret_label_clone().dynamic_union(slice.get_dynamic_secret_label_reference()), len.get_dynamic_integrity_label_clone().dynamic_intersection(slice.get_dynamic_integrity_label_reference()), {
                let mut unwrapped_mut = unwrap_secret_mut_ref(len);
                let unwrapped = unwrap_secret_ref(&slice);
                let added1 = core::primitive::str::chars(unwrapped);
                let mapped = std::str::Chars::map(added1, core::primitive::char::len_utf16);
                let added2 = unchecked_operation(mapped.sum::<usize>());
                unchecked_operation(*unwrapped_mut += UTF16CodeUnits{value: added2});
                //map(char::len_utf16)
                //str::chars(unwrapped)
            });
            /* *len += UTF16CodeUnits(slice.chars().map(char::len_utf16).sum::<usize>())*/
        });
        info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {
            unwrap_secret(result)
        })
    }

    /// Run the callback on a series of slices that, concatenated, make up the selected text.
    ///
    /// The accumulator `acc` can be mutated by the callback, and will be returned at the end.
    fn fold_selection_slices<B: SecretValueSafe, F: FnMut(&mut InfoFlowStruct<B, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>, InfoFlowStruct<&str, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>)>(&self, mut acc: InfoFlowStruct<B, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>, mut f: F) -> InfoFlowStruct<B, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel> {
        if self.has_selection() {
            let (start, end) = self.sorted_selection_bounds();
            let start_offset = start.index.value;
            let end_offset = end.index.value;
            //let UTF8Bytes(start_offset) = start.index;
            //let UTF8Bytes(end_offset) = end.index;

            if start.line == end.line {
                let lines_ref = &self.lines;
                let a: InfoFlowStruct<&str, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel> = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, self.lines[start.line].get_dynamic_secret_label_clone(), self.lines[start.line].get_dynamic_integrity_label_clone(), {
                    let a: &DOMString = unwrap_secret_ref(&lines_ref[start.line]);
                    let b: &str = DOMString::to_str_ref(a);
                    wrap_secret(&b[start_offset..end_offset])
                }); //Vincent: EXPERIMENTAL RETURN REFERENCE TO SECRET
                f(&mut acc, a/*&self.lines[start.line][start_offset..end_offset]*/)
            } else {
                let lines_ref = &self.lines;
                let a = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, self.lines[start.line].get_dynamic_secret_label_clone(), self.lines[start.line].get_dynamic_integrity_label_clone(), {
                    let a = unwrap_secret_ref(&lines_ref[start.line]);
                    let b = &DOMString::to_str_ref(a)[start_offset..];
                    wrap_secret(b)
                }); //Vincent: EXPERIMENTAL RETURN REFERENCE TO SECRET
                f(&mut acc, a/*&self.lines[start.line][start_offset..]*/);
                for line in &self.lines[start.line + 1..end.line] {
                    let a = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {
                        wrap_secret("\n")
                    });
                    let b = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, self.lines[start.line].get_dynamic_secret_label_clone(), self.lines[start.line].get_dynamic_integrity_label_clone(), {
                        let a = unwrap_secret_ref(&line);
                        wrap_secret(DOMString::to_str_ref(&a))
                    }); //Vincent: EXPERIMENTAL RETURN REFERENCE TO SECRET
                    f(&mut acc, a/*"\n"*/);
                    f(&mut acc, b/*line*/);
                }
                let a = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {
                    wrap_secret("\n")
                });
                f(&mut acc, a/*"\n"*/);
                let lines_ref = &self.lines;
                let a = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, self.lines[end.line].get_dynamic_secret_label_clone(), self.lines[end.line].get_dynamic_integrity_label_clone(), {
                    let a = unwrap_secret_ref(&lines_ref[end.line]);
                    let b = &DOMString::to_str_ref(a)[..end_offset];
                    wrap_secret(b)
                }); //Vincent: EXPERIMENTAL RETURN REFERENCE TO SECRET
                f(&mut acc, a/*&self.lines[end.line][..end_offset]*/)
            }
        }

        acc
    }

    pub fn replace_selection(&mut self, insert: /*DOMString*/ ServoSecureDynamic<DOMString>) {
        if !self.has_selection() {
            return;
        }

        let allowed_to_insert_count = if let Some(max_length) = self.max_length {
            let len_after_selection_replaced =
                self.utf16_len().saturating_sub(self.selection_utf16_len());
            max_length.saturating_sub(len_after_selection_replaced)
        } else {
            UTF16CodeUnits{value: usize::MAX}
        };

        let last_char_index =
            len_of_first_n_code_units(&/***/insert, allowed_to_insert_count).value;
            //usize 
        //let to_insert = &insert[..last_char_index];
        let to_insert = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, insert.get_dynamic_secret_label_clone(), insert.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&insert);
            wrap_secret(&((DOMString::to_str_ref(&unwrapped))[..last_char_index]))
        });

        let (start, end) = self.sorted_selection_bounds();
        let start_offset = start.index.value;
        let end_offset = end.index.value;
        //let UTF8Bytes(start_offset) = start.index;
        //let UTF8Bytes(end_offset) = end.index;

        let new_lines = {
            //let prefix = &l[start.line][..start_offset];
            //let suffix  = &self.lines[end.line][end_offset..];
            let lines_prefix = &self.lines[..start.line];
            let lines_suffix = &self.lines[end.line + 1..];

            let mut initial_insert_lines = if self.multiline {
                info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, to_insert.get_dynamic_secret_label_clone(), to_insert.get_dynamic_integrity_label_clone(), {
                    let unwrapped = unwrap_secret_ref(&to_insert);
                    let splits = core::primitive::str::split(unwrapped, |c| c == '\n');
                    let mapped = custom_map(splits, |s| DOMString::from_string(std::string::String::from(s)));
                    let collected = custom_collect_unwrapped(mapped);
                    wrap_secret(collected)
                })
                //to_insert.split('\n').map(|s| DOMString::from(s)).collect()
            } else {
                info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, to_insert.get_dynamic_secret_label_clone(), to_insert.get_dynamic_integrity_label_clone(), {
                    let unwrapped = unwrap_secret_ref(&to_insert);
                    let mut v = std::vec::Vec::new();
                    std::vec::Vec::push(&mut v, DOMString::from_string(std::string::String::from(*unwrapped)));
                    wrap_secret(v)
                    //vec![DOMString::from(to_insert)]
                })
            };

            let mut insert_lines = {
                let mut returned = vec![];
                let length = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, initial_insert_lines.get_dynamic_secret_label_clone(), initial_insert_lines.get_dynamic_integrity_label_clone(), {
                    let unwrapped = unwrap_secret_ref(&initial_insert_lines);
                    std::vec::Vec::len(unwrapped)
                });
                for i in 0..length {
                    returned.insert(0, info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, initial_insert_lines.get_dynamic_secret_label_clone(), initial_insert_lines.get_dynamic_integrity_label_clone(), {
                            let mut vec = unwrap_secret_mut_ref(&mut initial_insert_lines);
                            let popped = std::vec::Vec::pop(vec);
                            wrap_secret(std::option::Option::unwrap(popped))
                            //wrap_secret(popped.expect("Should be an actual element"))
                        })
                    );
                }
                returned
            };

            // FIXME(ajeffrey): effecient append for DOMStrings
            //Vincent: deleted prefix and new_line initializations to instead have one initialization for secret new_line
            let lines_ref = &self.lines;
            let merged_sec_label = self.lines[start.line].get_dynamic_secret_label_clone().dynamic_union(insert_lines[0].get_dynamic_secret_label_reference());
            let merged_int_label = self.lines[start.line].get_dynamic_integrity_label_clone().dynamic_intersection(insert_lines[0].get_dynamic_integrity_label_reference());
            let mut new_line = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, merged_sec_label.clone(), merged_int_label.clone(), {
                let unwrapped = unwrap_secret_ref(&lines_ref[start.line]);
                let pre_string = core::primitive::str::as_bytes(&DOMString::to_str_ref(unwrapped)[..start_offset]);
                let owned = <[u8]>::to_owned(&pre_string);
                wrap_secret(unsafe { std::string::String::from_utf8_unchecked(owned) })
            });
            //let mut new_line = prefix.to_owned();

            let insert_zero_ref = &insert_lines[0];
            info_flow_block_no_return_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, merged_sec_label.clone(), merged_int_label.clone(), {
                let mut mut_unwrapped = unwrap_secret_mut_ref(&mut new_line);
                let unwrapped = unwrap_secret_ref(insert_zero_ref);
                std::string::String::push_str(mut_unwrapped, std::string::String::as_str(DOMString::to_string_ref(unwrapped)));
            });
            //new_line.push_str(&insert_lines[0]);
            //insert_lines[0] = DOMString::from(new_line);
            insert_lines[0] = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, merged_sec_label.clone(), merged_int_label.clone(), {
                let unwrapped = unwrap_secret(new_line);
                wrap_secret(DOMString::from_string(unwrapped))
            });

            let last_insert_lines_index = insert_lines.len() - 1;
            let last_insert_lines_ref = &insert_lines[insert_lines.len() - 1];
            self.edit_point.index = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, insert_lines[last_insert_lines_index].get_dynamic_secret_label_clone(), insert_lines[last_insert_lines_index].get_dynamic_integrity_label_clone(), {
                let unwrapped = unwrap_secret_ref(last_insert_lines_ref);
                len_utf8_str(&DOMString::to_str_ref(unwrapped))
            });
            //self.edit_point.index = insert_lines[last_insert_lines_index].len_utf8();
            self.edit_point.line = start.line + last_insert_lines_index;

            // FIXME(ajeffrey): effecient append for DOMStrings
            let lines_ref = &self.lines;
            let to_unwrap = &mut insert_lines[last_insert_lines_index];
            info_flow_block_no_return_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, to_unwrap.get_dynamic_secret_label_clone().dynamic_union(self.lines[end.line].get_dynamic_secret_label_reference()), to_unwrap.get_dynamic_integrity_label_clone().dynamic_intersection(self.lines[end.line].get_dynamic_integrity_label_reference()), {
                let mut mut_unwrapped = unwrap_secret_mut_ref(to_unwrap);
                let mut mut_string = DOMString::to_mut_string_ref(mut_unwrapped);
                let unwrapped = unwrap_secret_ref(&lines_ref[end.line]);
                let pre_string = &DOMString::to_str_ref(unwrapped)[end_offset..];
                std::string::String::push_str(mut_string, pre_string);
            });
            //insert_lines[last_insert_lines_index].push_str(suffix);

            let mut new_lines = vec![];
            new_lines.extend_from_slice(lines_prefix);
            new_lines.extend_from_slice(&*insert_lines);
            new_lines.extend_from_slice(lines_suffix);
            new_lines
        };

        self.lines = new_lines;
        self.was_last_change_by_set_content = false;
        self.clear_selection();
        self.assert_ok_selection();
    }

    /// Return the length in bytes of the current line under the editing point.
    pub fn current_line_length(&self) -> UTF8Bytes {
        let lines_ref = &self.lines;
        let edit_point_line = self.edit_point.line;
        info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, self.lines[self.edit_point.line].get_dynamic_secret_label_clone(), self.lines[self.edit_point.line].get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&lines_ref[edit_point_line]);
            len_utf8_str(&DOMString::to_str_ref(unwrapped))
        })
    }

    /// Adjust the editing point position by a given number of lines. The resulting column is
    /// as close to the original column position as possible.
    pub fn adjust_vertical(&mut self, adjust: isize, select: Selection) {
        if !self.multiline {
            return;
        }

        if select == Selection::Selected {
            if self.selection_origin.is_none() {
                self.selection_origin = Some(self.edit_point);
            }
        } else {
            self.clear_selection();
        }

        assert!(self.edit_point.line < self.lines.len());

        let target_line: isize = self.edit_point.line as isize + adjust;

        if target_line < 0 {
            self.edit_point.line = 0;
            self.edit_point.index = UTF8Bytes::zero();
            if self.selection_origin.is_some() &&
                (self.selection_direction == SelectionDirection::None ||
                    self.selection_direction == SelectionDirection::Forward)
            {
                self.selection_origin = Some(TextPoint {
                    line: 0,
                    index: UTF8Bytes::zero(),
                });
            }
            return;
        } else if target_line as usize >= self.lines.len() {
            self.edit_point.line = self.lines.len() - 1;
            self.edit_point.index = self.current_line_length();
            if self.selection_origin.is_some() &&
                (self.selection_direction == SelectionDirection::Backward)
            {
                self.selection_origin = Some(self.edit_point);
            }
            return;
        }

        let edit_index = self.edit_point.index.value;
        //let UTF8Bytes(edit_index) = self.edit_point.index;
        let lines_ref = &self.lines;
        let edit_line = self.edit_point.line;
        let col = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, self.lines[self.edit_point.line].get_dynamic_secret_label_clone(), self.lines[self.edit_point.line].get_dynamic_integrity_label_clone(), {
            let unwrapped_u = unwrap_secret_ref(&lines_ref[edit_line]);
            let chs = core::primitive::str::chars(&DOMString::to_str_ref(unwrapped_u)[..edit_index]);
            wrap_secret(std::str::Chars::count(chs))
        });
        /*let col = self.lines[self.edit_point.line][..edit_index]
            .chars()
            .count();*/
        self.edit_point.line = target_line as usize;
        // NOTE: this adjusts to the nearest complete Unicode codepoint, rather than grapheme cluster
        self.edit_point.index = len_of_first_n_chars(&self.lines[self.edit_point.line], col);
        if let Some(origin) = self.selection_origin {
            if ((self.selection_direction == SelectionDirection::None ||
                self.selection_direction == SelectionDirection::Forward) &&
                self.edit_point <= origin) ||
                (self.selection_direction == SelectionDirection::Backward &&
                    origin <= self.edit_point)
            {
                self.selection_origin = Some(self.edit_point);
            }
        }
        self.assert_ok_selection();
    }

    /// Adjust the editing point position by a given number of bytes. If the adjustment
    /// requested is larger than is available in the current line, the editing point is
    /// adjusted vertically and the process repeats with the remaining adjustment requested.
    pub fn adjust_horizontal(
        &mut self,
        adjust: UTF8Bytes,
        direction: Direction,
        select: Selection,
    ) {
        if self.adjust_selection_for_horizontal_change(direction, select) {
            return;
        }
        self.perform_horizontal_adjustment(adjust, direction, select);
    }

    /// Adjust the editing point position by exactly one grapheme cluster. If the edit point
    /// is at the beginning of the line and the direction is "Backward" or the edit point is at
    /// the end of the line and the direction is "Forward", a vertical adjustment is made
    pub fn adjust_horizontal_by_one(&mut self, direction: Direction, select: Selection) {
        if self.adjust_selection_for_horizontal_change(direction, select) {
            return;
        }
        let adjust = {
            let current_line = &self.lines[self.edit_point.line];
            let current_offset = self.edit_point.index.value;
            //let UTF8Bytes(current_offset) = self.edit_point.index;
            let next_ch = match direction {
                Direction::Forward => {
                    info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, current_line.get_dynamic_secret_label_clone(), current_line.get_dynamic_integrity_label_clone(),{
                        let unwrapped = unwrap_secret_ref(&current_line);
                        let mut gs = custom_graphemes(&DOMString::to_str_ref(unwrapped)[current_offset..], true);
                        wrap_secret(custom_next_back(&mut gs))
                    })
                },
                //Direction::Forward => current_line[current_offset..].graphemes(true).next(),
                Direction::Backward => {
                    info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, current_line.get_dynamic_secret_label_clone(), current_line.get_dynamic_integrity_label_clone(),{
                        let unwrapped = unwrap_secret_ref(&current_line);
                        let mut gs = custom_graphemes(&DOMString::to_str_ref(unwrapped)[..current_offset], true);
                        wrap_secret(custom_next_back(&mut gs))
                    })
                },
                //Direction::Backward => current_line[..current_offset].graphemes(true).next_back(),
            };
            let result = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, next_ch.get_dynamic_secret_label_clone(), next_ch.get_dynamic_integrity_label_clone(), {
                let unwrapped = unwrap_secret_ref(&next_ch);
                match *unwrapped {
                    None => wrap_secret(UTF8Bytes::one()),
                    Some(c) => wrap_secret(UTF8Bytes{value: core::primitive::str::len(c) as usize})
                }
            });
            info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {
                unwrap_secret(result)
            })
            /*
            match next_ch {
                Some(c) => UTF8Bytes{value: c.len() as usize},
                None => UTF8Bytes::one(), // Going to the next line is a "one byte" offset
            }*/
        };
        self.perform_horizontal_adjustment(adjust, direction, select);
    }

    /// Return whether to cancel the caret move
    fn adjust_selection_for_horizontal_change(
        &mut self,
        adjust: Direction,
        select: Selection,
    ) -> bool {
        if select == Selection::Selected {
            if self.selection_origin.is_none() {
                self.selection_origin = Some(self.edit_point);
            }
        } else {
            if self.has_selection() {
                self.edit_point = match adjust {
                    Direction::Backward => self.selection_start(),
                    Direction::Forward => self.selection_end(),
                };
                self.clear_selection();
                return true;
            }
        }
        false
    }

    /// Update the field selection_direction.
    ///
    /// When the edit_point (or focus) is before the selection_origin (or anchor)
    /// you have a backward selection. Otherwise you have a forward selection.
    fn update_selection_direction(&mut self) {
        debug!(
            "edit_point: {:?}, selection_origin: {:?}",
            self.edit_point, self.selection_origin
        );
        self.selection_direction = if Some(self.edit_point) < self.selection_origin {
            SelectionDirection::Backward
        } else {
            SelectionDirection::Forward
        }
    }

    fn perform_horizontal_adjustment(
        &mut self,
        adjust: UTF8Bytes,
        direction: Direction,
        select: Selection,
    ) {
        match direction {
            Direction::Backward => {
                let remaining = self.edit_point.index;
                if adjust > remaining && self.edit_point.line > 0 {
                    self.adjust_vertical(-1, select);
                    self.edit_point.index = self.current_line_length();
                    // one shift is consumed by the change of line, hence the -1
                    self.adjust_horizontal(
                        adjust.saturating_sub(remaining + UTF8Bytes::one()),
                        direction,
                        select,
                    );
                } else {
                    self.edit_point.index = remaining.saturating_sub(adjust);
                }
            },
            Direction::Forward => {
                let remaining = self
                    .current_line_length()
                    .saturating_sub(self.edit_point.index);
                if adjust > remaining && self.lines.len() > self.edit_point.line + 1 {
                    self.adjust_vertical(1, select);
                    self.edit_point.index = UTF8Bytes::zero();
                    // one shift is consumed by the change of line, hence the -1
                    self.adjust_horizontal(
                        adjust.saturating_sub(remaining + UTF8Bytes::one()),
                        direction,
                        select,
                    );
                } else {
                    self.edit_point.index =
                        min(self.current_line_length(), self.edit_point.index + adjust);
                }
            },
        };
        self.update_selection_direction();
        self.assert_ok_selection();
    }

    /// Deal with a newline input.
    pub fn handle_return(&mut self) -> KeyReaction {
        if !self.multiline {
            KeyReaction::TriggerDefaultAction
        } else {
            self.insert_char('\n');
            KeyReaction::DispatchInput
        }
    }

    /// Select all text in the input control.
    pub fn select_all(&mut self) {
        self.selection_origin = Some(TextPoint {
            line: 0,
            index: UTF8Bytes::zero(),
        });
        let last_line = self.lines.len() - 1;
        self.edit_point.line = last_line;
        let lines_ref = &self.lines;
        self.edit_point.index = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, self.lines[last_line].get_dynamic_secret_label_clone(), self.lines[last_line].get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&lines_ref[last_line]);
            len_utf8_str(&DOMString::to_str_ref(unwrapped))
        })/*self.lines[last_line].len_utf8()*/;
        self.selection_direction = SelectionDirection::Forward;
        self.assert_ok_selection();
    }

    /// Remove the current selection.
    pub fn clear_selection(&mut self) {
        self.selection_origin = None;
        self.selection_direction = SelectionDirection::None;
    }

    /// Remove the current selection and set the edit point to the end of the content.
    pub fn clear_selection_to_limit(&mut self, direction: Direction) {
        self.clear_selection();
        self.adjust_horizontal_to_limit(direction, Selection::NotSelected);
    }

    pub fn adjust_horizontal_by_word(&mut self, direction: Direction, select: Selection) {
        if self.adjust_selection_for_horizontal_change(direction, select) {
            return;
        }
        let shift_increment: UTF8Bytes = {
            let current_index = self.edit_point.index;
            let current_line = self.edit_point.line;
            let mut newline_adjustment = UTF8Bytes::zero();
            let mut shift_temp = UTF8Bytes::zero();
            match direction {
                Direction::Backward => {
                    let input: InfoFlowStruct<&str, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>;
                    if current_index == UTF8Bytes::zero() && current_line > 0 {
                        let lines_ref = &self.lines;
                        input = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, self.lines[current_line - 1].get_dynamic_secret_label_clone(), self.lines[current_line - 1].get_dynamic_integrity_label_clone(), {
                            let u = &**unwrap_secret_ref(&lines_ref[current_line - 1]);
                            wrap_secret(u)
                        })/*&self.lines[current_line - 1]*/;
                        newline_adjustment = UTF8Bytes::one();
                    } else {
                        let remaining = current_index.value;
                        //let UTF8Bytes(remaining) = current_index;
                        let lines_ref = &self.lines;
                        input = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, self.lines[current_line].get_dynamic_secret_label_clone(), self.lines[current_line - 1].get_dynamic_integrity_label_clone(), {
                            let u = DOMString::to_str_ref(unwrap_secret_ref(&lines_ref[current_line]));
                            wrap_secret(&u[..remaining])
                        })/*&self.lines[current_line][..remaining]*/;
                    }

                    let new_utf8 = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, input.get_dynamic_secret_label_clone(), input.get_dynamic_integrity_label_clone(), {
                        let unwrapped = unwrap_secret_ref(&input);
                        let mut iter = custom_split_word_bounds(unwrapped);
                        let mut iter2 = custom_rev(iter);
                        let mut result = UTF8Bytes::zero();
                        loop {
                            match custom_next_rev(&mut iter2) {
                                None => break,
                                Some(x) => {
                                    unchecked_operation(result += UTF8Bytes{value: x.len() as usize});
                                    let mut chars = core::primitive::str::chars(x);
                                    if std::str::Chars::any(&mut chars, |x| core::primitive::char::is_alphabetic(x) || core::primitive::char::is_numeric(x)) {
                                        break;
                                    }
                                }
                            }
                        }
                        wrap_secret(result)
                    });
                    shift_temp = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_utf8.get_dynamic_secret_label_clone(), new_utf8.get_dynamic_integrity_label_clone(), {
                        unwrap_secret(new_utf8)
                    });
                    /*let mut iter = input.split_word_bounds().rev();
                    loop {
                        match iter.next() {
                            None => break,
                            Some(x) => {
                                shift_temp += UTF8Bytes{value: x.len() as usize};
                                if x.chars().any(|x| x.is_alphabetic() || x.is_numeric()) {
                                    break;
                                }
                            },
                        }
                    }*/
                },
                Direction::Forward => {
                    let input: InfoFlowStruct<&str, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>;
                    let remaining = self.current_line_length().saturating_sub(current_index);
                    if remaining == UTF8Bytes::zero() && self.lines.len() > self.edit_point.line + 1
                    {
                        let lines_ref = &self.lines;
                        input = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, self.lines[current_line + 1].get_dynamic_secret_label_clone(), self.lines[current_line + 1].get_dynamic_integrity_label_clone(), {
                            let u = &**unwrap_secret_ref(&lines_ref[current_line + 1]);
                            wrap_secret(u)
                        })/*&self.lines[current_line + 1]*/;
                        ;
                        newline_adjustment = UTF8Bytes::one();
                    } else {
                        let current_offset = current_index.value;
                        //let UTF8Bytes(current_offset) = current_index;
                        let lines_ref = &self.lines;
                        input = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, self.lines[current_line].get_dynamic_secret_label_clone(), self.lines[current_line - 1].get_dynamic_integrity_label_clone(), {
                            let u = DOMString::to_str_ref(unwrap_secret_ref(&lines_ref[current_line]));
                            wrap_secret(&u[current_offset..])
                        })/*&self.lines[current_line][current_offset..]*/;
                    }

                    let new_utf8 = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, input.get_dynamic_secret_label_clone(), input.get_dynamic_integrity_label_clone(), {
                        let unwrapped = unwrap_secret_ref(&input);
                        let mut iter = custom_split_word_bounds(unwrapped);
                        let mut result = UTF8Bytes::zero();
                        loop {
                            match custom_next_uwordbounds(&mut iter) {
                                None => break,
                                Some(x) => {
                                    unchecked_operation(result += UTF8Bytes{value: x.len() as usize});
                                    let mut chars = core::primitive::str::chars(x);
                                    if std::str::Chars::any(&mut chars, |x| core::primitive::char::is_alphabetic(x) || core::primitive::char::is_numeric(x)) {
                                        break;
                                    }
                                }
                            }
                        }
                        wrap_secret(result)
                    });
                    shift_temp = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_utf8.get_dynamic_secret_label_clone(), new_utf8.get_dynamic_integrity_label_clone(), {
                        unwrap_secret(new_utf8)
                    });
                    /*let mut iter = input.split_word_bounds();
                    loop {
                        match iter.next() {
                            None => break,
                            Some(x) => {
                                shift_temp += UTF8Bytes{value: x.len() as usize};
                                if x.chars().any(|x| x.is_alphabetic() || x.is_numeric()) {
                                    break;
                                }
                            },
                        }
                    }*/
                },
            };

            shift_temp + newline_adjustment
        };

        self.adjust_horizontal(shift_increment, direction, select);
    }

    pub fn adjust_horizontal_to_line_end(&mut self, direction: Direction, select: Selection) {
        if self.adjust_selection_for_horizontal_change(direction, select) {
            return;
        }
        let shift: usize = {
            let current_line = &self.lines[self.edit_point.line];
            let current_offset = self.edit_point.index.value;
            //let UTF8Bytes(current_offset) = self.edit_point.index;
            match direction {
                Direction::Backward => {
                    info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, current_line.get_dynamic_secret_label_clone(), current_line.get_dynamic_integrity_label_clone(), {
                        let unwrapped = unwrap_secret_ref(&current_line);
                        core::primitive::str::len(&DOMString::to_str_ref(unwrapped)[..current_offset])
                    })
                } /*current_line[..current_offset].len()*/,
                Direction::Forward =>  {
                    info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, current_line.get_dynamic_secret_label_clone(), current_line.get_dynamic_integrity_label_clone(), {
                        let unwrapped = unwrap_secret_ref(&current_line);
                        core::primitive::str::len(&DOMString::to_str_ref(unwrapped)[current_offset..])
                    })
                } /*current_line[current_offset..].len()*/,
            }
        };
        self.perform_horizontal_adjustment(UTF8Bytes{value: shift}, direction, select);
    }

    pub fn adjust_horizontal_to_limit(&mut self, direction: Direction, select: Selection) {
        if self.adjust_selection_for_horizontal_change(direction, select) {
            return;
        }
        match direction {
            Direction::Backward => {
                self.edit_point.line = 0;
                self.edit_point.index = UTF8Bytes::zero();
            },
            Direction::Forward => {
                self.edit_point.line = &self.lines.len() - 1;
                let lines_ref = &self.lines;
                let lines_ref_index = &self.lines.len() - 1;
                self.edit_point.index = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, self.lines[&self.lines.len() - 1].get_dynamic_secret_label_clone(), self.lines[&self.lines.len() - 1].get_dynamic_integrity_label_clone(), {
                    let unwrapped = unwrap_secret_ref(&lines_ref[lines_ref_index]);
                    len_utf8_str(&DOMString::to_str_ref(unwrapped))
                }); /*(&self.lines[&self.lines.len() - 1]).len_utf8();*/
            },
        }
    }

    /// Process a given `KeyboardEvent` and return an action for the caller to execute.
    //Vincent: Changed function to preserve secrecy
    pub fn handle_keydown(&mut self, event: &KeyboardEvent) -> KeyReaction {
        //let key_stage_1 = event.get_typed_key();
        //let key_wrapper: KeyWrapper = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, key_stage_1.get_dynamic_secret_label_clone(), key_stage_1.get_dynamic_integrity_label_clone(), {
        //    unwrap_secret(key_stage_1)
        //});
        //let key = key_wrapper.k;
        let key = event.get_typed_key();
        //let mods_stage_1 = event.get_modifiers();
        //let mods_wrapper: ModifiersWrapper = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, mods_stage_1.get_dynamic_secret_label_clone(), mods_stage_1.get_dynamic_integrity_label_clone(), {
        //    unwrap_secret(mods_stage_1)
        //});
        //let mods = mods_wrapper.m;
        let mods = event.get_modifiers();
        //let key = event.key(); 
        //let mods = event.modifiers();
        self.handle_keydown_aux(key, mods, cfg!(target_os = "macos"))
    }

    // This function exists for easy unit testing.
    // To test Mac OS shortcuts on other systems a flag is passed.
    //Vincent: Modified function to preserve secrecy.
    pub fn handle_keydown_aux(
        &mut self,
        key: ServoSecureDynamic<KeyWrapper>,
        mut mods: ServoSecureDynamic<ModifiersWrapper>,
        macos: bool,
    ) -> KeyReaction {
        let shift_wrapper = ModifiersWrapper{m: Modifiers::SHIFT};
        let shift_wrapper2 = ModifiersWrapper{m: Modifiers::SHIFT};
        let mods_cond_classified = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, mods.get_dynamic_secret_label_clone(), mods.get_dynamic_integrity_label_clone(), {
            let m = unwrap_secret_ref(&mods);
            wrap_secret(custom_contains(m, shift_wrapper))
        });
        let mods_cond_declassified = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, mods_cond_classified.get_dynamic_secret_label_clone(), mods_cond_classified.get_dynamic_integrity_label_clone(), {
            unwrap_secret(mods_cond_classified)
        });
        let maybe_select = if mods_cond_declassified /*mods.contains(Modifiers::SHIFT)*/ {
            Selection::Selected
        } else {
            Selection::NotSelected
        };
        info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, mods.get_dynamic_secret_label_clone(), mods.get_dynamic_integrity_label_clone(), {
            let mut m = unwrap_secret_mut_ref(&mut mods);
            custom_remove(&mut m, shift_wrapper2)
        });
        //mods.remove(Modifiers::SHIFT);
        
        //Vincent: DECLASSIFY 
        let k = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, key.get_dynamic_secret_label_clone(), key.get_dynamic_integrity_label_clone(), {
            custom_clone_key_wrapper(unwrap_secret_ref(&key))
        }).k;
        //Vincent: DECLASSIFY
        let m: Modifiers = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, mods.get_dynamic_secret_label_clone(), mods.get_dynamic_integrity_label_clone(), {
            custom_clone_modifiers_wrapper(unwrap_secret_ref(&mods))
        }).m;
        ShortcutMatcher::new(KeyState::Down, /*key.clone()*/ k, /*mods*/ m)
            .shortcut(Modifiers::CONTROL | Modifiers::ALT, 'B', || {
                self.adjust_horizontal_by_word(Direction::Backward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .shortcut(Modifiers::CONTROL | Modifiers::ALT, 'F', || {
                self.adjust_horizontal_by_word(Direction::Forward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .shortcut(Modifiers::CONTROL | Modifiers::ALT, 'A', || {
                self.adjust_horizontal_to_line_end(Direction::Backward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .shortcut(Modifiers::CONTROL | Modifiers::ALT, 'E', || {
                self.adjust_horizontal_to_line_end(Direction::Forward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .optional_shortcut(macos, Modifiers::CONTROL, 'A', || {
                self.adjust_horizontal_to_line_end(Direction::Backward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .optional_shortcut(macos, Modifiers::CONTROL, 'E', || {
                self.adjust_horizontal_to_line_end(Direction::Forward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .shortcut(CMD_OR_CONTROL, 'A', || {
                self.select_all();
                KeyReaction::RedrawSelection
            })
            .shortcut(CMD_OR_CONTROL, 'X', || {
                if let Some(text) = self.get_selection_text() {
                    self.clipboard_provider.set_clipboard_contents(info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, text.get_dynamic_secret_label_clone(), text.get_dynamic_integrity_label_clone(), {
                        unwrap_secret(text)
                    })/*text*/);
                    self.delete_char(Direction::Backward);
                }
                KeyReaction::DispatchInput
            })
            .shortcut(CMD_OR_CONTROL, 'C', || {
                if let Some(text) = self.get_selection_text() {
                    self.clipboard_provider.set_clipboard_contents(info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, text.get_dynamic_secret_label_clone(), text.get_dynamic_integrity_label_clone(), {
                        unwrap_secret(text)
                    })/*text*/);
                }
                KeyReaction::DispatchInput
            })
            .shortcut(CMD_OR_CONTROL, 'V', || {
                let contents = self.clipboard_provider.clipboard_contents();
                self.insert_string(contents);
                KeyReaction::DispatchInput
            })
            .shortcut(Modifiers::empty(), Key::Delete, || {
                self.delete_char(Direction::Forward);
                KeyReaction::DispatchInput
            })
            .shortcut(Modifiers::empty(), Key::Backspace, || {
                self.delete_char(Direction::Backward);
                KeyReaction::DispatchInput
            })
            .optional_shortcut(macos, Modifiers::META, Key::ArrowLeft, || {
                self.adjust_horizontal_to_line_end(Direction::Backward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .optional_shortcut(macos, Modifiers::META, Key::ArrowRight, || {
                self.adjust_horizontal_to_line_end(Direction::Forward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .optional_shortcut(macos, Modifiers::META, Key::ArrowUp, || {
                self.adjust_horizontal_to_limit(Direction::Backward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .optional_shortcut(macos, Modifiers::META, Key::ArrowDown, || {
                self.adjust_horizontal_to_limit(Direction::Forward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .shortcut(Modifiers::ALT, Key::ArrowLeft, || {
                self.adjust_horizontal_by_word(Direction::Backward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .shortcut(Modifiers::ALT, Key::ArrowRight, || {
                self.adjust_horizontal_by_word(Direction::Forward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .shortcut(Modifiers::empty(), Key::ArrowLeft, || {
                self.adjust_horizontal_by_one(Direction::Backward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .shortcut(Modifiers::empty(), Key::ArrowRight, || {
                self.adjust_horizontal_by_one(Direction::Forward, maybe_select);
                KeyReaction::RedrawSelection
            })
            .shortcut(Modifiers::empty(), Key::ArrowUp, || {
                self.adjust_vertical(-1, maybe_select);
                KeyReaction::RedrawSelection
            })
            .shortcut(Modifiers::empty(), Key::ArrowDown, || {
                self.adjust_vertical(1, maybe_select);
                KeyReaction::RedrawSelection
            })
            .shortcut(Modifiers::empty(), Key::Enter, || self.handle_return())
            .optional_shortcut(macos, Modifiers::empty(), Key::Home, || {
                self.edit_point.index = UTF8Bytes::zero();
                KeyReaction::RedrawSelection
            })
            .optional_shortcut(macos, Modifiers::empty(), Key::End, || {
                self.edit_point.index = self.current_line_length();
                self.assert_ok_selection();
                KeyReaction::RedrawSelection
            })
            .shortcut(Modifiers::empty(), Key::PageUp, || {
                self.adjust_vertical(-28, maybe_select);
                KeyReaction::RedrawSelection
            })
            .shortcut(Modifiers::empty(), Key::PageDown, || {
                self.adjust_vertical(28, maybe_select);
                KeyReaction::RedrawSelection
            })
            .otherwise(|| {
                //Vincent: TODO UNDO
                let cond_wrapped = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, key.get_dynamic_secret_label_clone(), key.get_dynamic_integrity_label_clone(), {
                    let unwrapped = unwrap_secret_ref(&key);
                    let mut val = false;
                    unchecked_operation(if let Key::Character(ref c) = unwrapped.k {
                        val = true;
                    });
                    wrap_secret(val)
                });
                let cond_unwrapped = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, key.get_dynamic_secret_label_clone(), key.get_dynamic_integrity_label_clone(), {
                    unwrap_secret(cond_wrapped)
                });
                let string_wrapped = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, key.get_dynamic_secret_label_clone(), key.get_dynamic_integrity_label_clone(), {
                    let unwrapped = unwrap_secret_ref(&key);
                    let mut s = std::string::String::from("");
                    unchecked_operation(if let Key::Character(ref c) = unwrapped.k {
                        s = c.as_str().into();
                    });
                    wrap_secret(s)
                });
                if /*let Key::Character(ref c) = /*k*/ key*/ cond_unwrapped {
                    self.insert_secret_string(string_wrapped/*c.as_str()*/);
                    return KeyReaction::DispatchInput;
                }
                KeyReaction::Nothing
            })
            .unwrap()
    }

    pub fn handle_compositionend(&mut self, event: &CompositionEvent) -> KeyReaction {
        self.insert_string(event.data());
        KeyReaction::DispatchInput
    }

    /// Whether the content is empty.
    pub fn is_empty(&self) -> bool {
        self.lines.len() <= 1 && self.lines.get(0).map_or(true, |line| {
            info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, line.get_dynamic_secret_label_clone(), line.get_dynamic_integrity_label_clone(), {
                let unwrapped = unwrap_secret_ref(&line);
                std::string::String::is_empty(DOMString::to_string_ref(&unwrapped))
            })
        } /*line.is_empty()*/)
    }

    /// The length of the content in bytes.
    pub fn len_utf8(&self) -> UTF8Bytes {
        self.lines
            .iter()
            .fold(UTF8Bytes::zero(), |m, l| {
                let l_len_utf8 = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, l.get_dynamic_secret_label_clone(), l.get_dynamic_integrity_label_clone(), {
                    let unwrapped = unwrap_secret_ref(&l);
                    len_utf8_str(&DOMString::to_str_ref(unwrapped))
                });
                m + /*l.len_utf8()*/ l_len_utf8 + UTF8Bytes::one() // + 1 for the '\n'
            })
            .saturating_sub(UTF8Bytes::one())
    }

    /// The total number of code units required to encode the content in utf16.
    pub fn utf16_len(&self) -> UTF16CodeUnits {
        self.lines
            .iter()
            .fold(UTF16CodeUnits::zero(), |m, l| {
                let len_chars_map = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, l.get_dynamic_secret_label_clone(), l.get_dynamic_integrity_label_clone(), {
                    let unwrapped_chars = core::primitive::str::chars(std::string::String::as_str(DOMString::to_string_ref(unwrap_secret_ref(l))));
                    let mapped = std::str::Chars::map(unwrapped_chars, core::primitive::char::len_utf16);
                    unchecked_operation(mapped.sum::<usize>() + 1)
                });
                m + UTF16CodeUnits{value: len_chars_map /*l.chars().map(char::len_utf16).sum::<usize>() + 1*/}
                // + 1 for the '\n'
            })
            .saturating_sub(UTF16CodeUnits::one())
    }

    /// The length of the content in Unicode code points.
    pub fn char_count(&self) -> usize {
        self.lines.iter().fold(0, |m, l| {
            let l_chars_count = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, l.get_dynamic_secret_label_clone(), l.get_dynamic_integrity_label_clone(), {
                let unwrapped_chars = core::primitive::str::chars(std::string::String::as_str(DOMString::to_string_ref(unwrap_secret_ref(l))));
                std::str::Chars::count(unwrapped_chars)
            });
            m + l_chars_count /*l.chars().count()*/ + 1 // + 1 for the '\n'
        }) - 1
    }

    /// Get the current contents of the text input. Multiple lines are joined by \n.
    pub fn get_content(&self) -> ServoSecureDynamic<DOMString> {
        if (self.lines.is_empty())
        {
            //TODO: use domain information from element owning this textinput
            ServoSecureDynamic::<DOMString>::new_info_flow_struct(DOMString::from_string(String::from("")),
            new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]))
        }
        else
        {
            let sec_label = self.lines[0].get_dynamic_secret_label_clone();
            let int_label = self.lines[0].get_dynamic_integrity_label_clone();
            let lines_ref = &self.lines;
            info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, sec_label, int_label, {
                let mut content = std::string::String::from("");
                for (i, line) in unchecked_operation(std::iter::Iterator::enumerate(lines_ref.iter())) {
                    std::string::String::push_str(&mut content, DOMString::to_str_ref(unwrap_secret_ref(line)));
                    //content.push_str(unwrap_secret_ref(&line));
                    if i < std::vec::Vec::len(lines_ref) - 1 {
                        std::string::String::push(&mut content, '\n');
                        //content.push('\n');
                    }
                }
                wrap_secret(DOMString::from_string(content))
            })
        }
    }

    /// Get a reference to the contents of a single-line text input. Panics if self is a multiline input.
    pub fn single_line_content(&self) -> &InfoFlowStruct<DOMString, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel> {
        assert!(!self.multiline);
        &self.lines[0]
    }

    /// Set the current contents of the text input. If this is control supports multiple lines,
    /// any \n encountered will be stripped and force a new logical line.
    pub fn set_content(&mut self, content: InfoFlowStruct<DOMString, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>) {
        self.lines = if self.multiline {
            let result = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, content.get_dynamic_secret_label_clone(), content.get_dynamic_integrity_label_clone(), {
                let unwrapped = unwrap_secret_ref(&content);
                let replaced: String = core::primitive::str::replace(DOMString::to_str_ref(unwrapped), "\r\n", "\n");
                let split = core::primitive::str::split(&replaced, |c| c == '\n' || c == '\r');
                let mapped = custom_map(split, |s| secret_structs::info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, secret_structs::secret::InfoFlowStruct::get_dynamic_secret_label_clone(&content), secret_structs::secret::InfoFlowStruct::get_dynamic_integrity_label_clone(&content), { wrap_secret(DOMString::from_string(core::primitive::str::to_string(s))) }));
                let collected = custom_collect_wrapped(mapped);
                wrap_secret(collected)
            });
            info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {
                unwrap_secret(result)
            })
            // https://html.spec.whatwg.org/multipage/#textarea-line-break-normalisation-transformation
            /*content
                .replace("\r\n", "\n")
                .split(|c| c == '\n' || c == '\r')
                .map(DOMString::from)
                .collect()*/
        } else {
            vec![content]
        };

        self.was_last_change_by_set_content = true;
        self.edit_point = self.edit_point.constrain_to(&self.lines);

        if let Some(origin) = self.selection_origin {
            self.selection_origin = Some(origin.constrain_to(&self.lines));
        }
        self.assert_ok_selection();
    }

    /// Convert a TextPoint into a byte offset from the start of the content.
    fn text_point_to_offset(&self, text_point: &TextPoint) -> UTF8Bytes {
        self.lines
            .iter()
            .enumerate()
            .fold(UTF8Bytes::zero(), |acc, (i, val)| {
                if i < text_point.line {
                    acc + info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, val.get_dynamic_secret_label_clone(), val.get_dynamic_integrity_label_clone(), {let unwrapped = unwrap_secret_ref(&val); len_utf8_str(&DOMString::to_str_ref(unwrapped))}) + UTF8Bytes::one() // +1 for the \n
                } else {
                    acc
                }
            }) +
            text_point.index
    }

    /// Convert a byte offset from the start of the content into a TextPoint.
    fn offset_to_text_point(&self, abs_point: UTF8Bytes) -> TextPoint {
        let mut index = abs_point;
        let mut line = 0;
        let last_line_idx = self.lines.len() - 1;
        self.lines
            .iter()
            .enumerate()
            .fold(UTF8Bytes::zero(), |acc, (i, val)| {
                if i != last_line_idx {
                    let line_end = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, val.get_dynamic_secret_label_clone(), val.get_dynamic_integrity_label_clone(), {let unwrapped = unwrap_secret_ref(&val); len_utf8_str(&DOMString::to_str_ref(unwrapped))});
                    let new_acc = acc + line_end + UTF8Bytes::one();
                    if abs_point >= new_acc && index > line_end {
                        index = index.saturating_sub(line_end + UTF8Bytes::one());
                        line += 1;
                    }
                    new_acc
                } else {
                    acc
                }
            });

        TextPoint {
            line: line,
            index: index,
        }
    }

    pub fn set_selection_range(&mut self, start: u32, end: u32, direction: SelectionDirection) {
        let mut start = UTF8Bytes{value: start as usize};
        let mut end = UTF8Bytes{value: end as usize};
        let content = self.get_content();
        let text_end = info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, content.get_dynamic_secret_label_clone(), content.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(&content); 
            len_utf8_str(&DOMString::to_str_ref(unwrapped))
        })/*self.get_content().len_utf8()*/;

        if end > text_end {
            end = text_end;
        }
        if start > end {
            start = end;
        }

        self.selection_direction = direction;

        match direction {
            SelectionDirection::None | SelectionDirection::Forward => {
                self.selection_origin = Some(self.offset_to_text_point(start));
                self.edit_point = self.offset_to_text_point(end);
            },
            SelectionDirection::Backward => {
                self.selection_origin = Some(self.offset_to_text_point(end));
                self.edit_point = self.offset_to_text_point(start);
            },
        }
        self.assert_ok_selection();
    }

    /// Set the edit point index position based off of a given grapheme cluster offset
    pub fn set_edit_point_index(&mut self, index: usize) {
        let start = &self.lines[self.edit_point.line];
        let byte_offset = info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, start.get_dynamic_secret_label_clone(), start.get_dynamic_integrity_label_clone(), {
            let unwrapped = unwrap_secret_ref(start);
            let graphemes = custom_graphemes(DOMString::to_str_ref(unwrapped), true);
            let taken = custom_take_graphemes(graphemes, index);
            let folded = custom_fold(taken, 0, |acc, x| acc + len_utf8_str(x).value);
            wrap_secret(folded)
        });
        self.edit_point.index = UTF8Bytes{value: info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, byte_offset.get_dynamic_secret_label_clone(), byte_offset.get_dynamic_integrity_label_clone(), {
            unwrap_secret(byte_offset)
        })};

        /*let byte_offset = self.lines[self.edit_point.line]
            .graphemes(true)
            .take(index)
            .fold(UTF8Bytes::zero(), |acc, x| acc + x.len_utf8());
        self.edit_point.index = byte_offset;*/
    }
}
