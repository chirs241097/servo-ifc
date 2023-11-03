// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use keyboard_types::{Key, Modifiers};
use script::clipboard_provider::ClipboardProvider;
use script::test::DOMString;
use script::textinput::{
    Direction, Lines, Selection, SelectionDirection, TextInput, TextPoint, UTF16CodeUnits,
    UTF8Bytes,
};

use secret_structs::ternary_lattice as sec_lat;
use secret_structs::integrity_lattice as int_lat;
use secret_structs::*;
use secret_structs::secret::*;
use keyboard_wrapper::*;

pub struct DummyClipboardContext {
    content: String,
}

impl DummyClipboardContext {
    pub fn new(s: &str) -> DummyClipboardContext {
        DummyClipboardContext {
            content: s.to_owned(),
        }
    }
}

impl ClipboardProvider for DummyClipboardContext {
    fn clipboard_contents(&mut self) -> String {
        self.content.clone()
    }
    fn set_clipboard_contents(&mut self, s: String) {
        self.content = s;
    }
}

fn text_input(lines: Lines, s: &str) -> TextInput<DummyClipboardContext> {
    TextInput::new(
        lines,
        DOMString::from(s),
        DummyClipboardContext::new(""),
        None,
        None,
        SelectionDirection::None,
    )
}

#[test]
fn test_set_content_ignores_max_length() {
    let mut textinput = TextInput::new(
        Lines::Single,
        DOMString::from(""),
        DummyClipboardContext::new(""),
        Some(UTF16CodeUnits::one()),
        None,
        SelectionDirection::None,
    );

    //Carapace: Changed function call to reflect Carapace API
    textinput.set_content(info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), { wrap_secret(DOMString::from_str("mozilla rocks"))}));
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, DOMString::from("mozilla rocks"));
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, DOMString::from("mozilla rocks"));
}

#[test]
fn test_textinput_when_inserting_multiple_lines_over_a_selection_respects_max_length() {
    let mut textinput = TextInput::new(
        Lines::Multiple,
        DOMString::from("hello\nworld"),
        DummyClipboardContext::new(""),
        //Carapace: Change UTF16CodeUnits to have named fields
        Some(UTF16CodeUnits{value: 17}),
        None,
        SelectionDirection::None,
    );

    textinput.adjust_horizontal(UTF8Bytes::one(), Direction::Forward, Selection::NotSelected);
    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 3}, Direction::Forward, Selection::Selected);
    textinput.adjust_vertical(1, Selection::Selected);

    // Selection is now "hello\n
    //                    ------
    //                   world"
    //                   ----

    textinput.insert_string("cruel\nterrible\nbad".to_string());

    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "hcruel\nterrible\nd");
}

#[test]
fn test_textinput_when_inserting_multiple_lines_still_respects_max_length() {
    let mut textinput = TextInput::new(
        Lines::Multiple,
        DOMString::from("hello\nworld"),
        DummyClipboardContext::new(""),
        //Carapace: Change UTF16CodeUnits to have named fields
        Some(UTF16CodeUnits{value: 17}),
        None,
        SelectionDirection::None,
    );

    textinput.adjust_vertical(1, Selection::NotSelected);
    textinput.insert_string("cruel\nterrible".to_string());

    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "hello\ncruel\nworld");
}

#[test]
fn test_textinput_when_content_is_already_longer_than_max_length_and_theres_no_selection_dont_insert_anything(
) {
    let mut textinput = TextInput::new(
        Lines::Single,
        DOMString::from("abc"),
        DummyClipboardContext::new(""),
        Some(UTF16CodeUnits::one()),
        None,
        SelectionDirection::None,
    );

    textinput.insert_char('a');

    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "abc");
}

#[test]
fn test_multi_line_textinput_with_maxlength_doesnt_allow_appending_characters_when_input_spans_lines(
) {
    let mut textinput = TextInput::new(
        Lines::Multiple,
        DOMString::from("abc\nd"),
        DummyClipboardContext::new(""),
        //Carapace: Change UTF16CodeUnits to have named fields
        Some(UTF16CodeUnits{value: 5}),
        None,
        SelectionDirection::None,
    );

    textinput.insert_char('a');

    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "abc\nd");
}

#[test]
fn test_single_line_textinput_with_max_length_doesnt_allow_appending_characters_when_replacing_a_selection(
) {
    let mut textinput = TextInput::new(
        Lines::Single,
        DOMString::from("abcde"),
        DummyClipboardContext::new(""),
        //Carapace: Change UTF16CodeUnits to have named fields
        Some(UTF16CodeUnits{value: 5}),
        None,
        SelectionDirection::None,
    );

    textinput.adjust_horizontal(UTF8Bytes::one(), Direction::Forward, Selection::NotSelected);
    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 3}, Direction::Forward, Selection::Selected);

    // Selection is now "abcde"
    //                    ---

    //Carapace: Changed function call to reflect Carapace API
    textinput.replace_selection({let ds = DOMString::from_str("too long"); info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(ds)})});

    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "atooe");
}

#[test]
fn test_single_line_textinput_with_max_length_allows_deletion_when_replacing_a_selection() {
    let mut textinput = TextInput::new(
        Lines::Single,
        DOMString::from("abcde"),
        DummyClipboardContext::new(""),
        //Carapace: Change UTF16CodeUnits to have named fields
        Some(UTF16CodeUnits{value: 1}),
        None,
        SelectionDirection::None,
    );

    textinput.adjust_horizontal(UTF8Bytes::one(), Direction::Forward, Selection::NotSelected);
    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 2}, Direction::Forward, Selection::Selected);

    // Selection is now "abcde"
    //                    --

    //Carapace: Changed function call to reflect Carapace API
    textinput.replace_selection({let ds = DOMString::from_str("only deletion should be applied"); info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(ds)})});

    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "ade");
}

#[test]
fn test_single_line_textinput_with_max_length_multibyte() {
    let mut textinput = TextInput::new(
        Lines::Single,
        DOMString::from(""),
        DummyClipboardContext::new(""),
        //Carapace: Change UTF16CodeUnits to have named fields
        Some(UTF16CodeUnits{value: 2}),
        None,
        SelectionDirection::None,
    );

    textinput.insert_char('á');
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "á");
    textinput.insert_char('é');
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "áé");
    textinput.insert_char('i');
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "áé");
}

#[test]
fn test_single_line_textinput_with_max_length_multi_code_unit() {
    let mut textinput = TextInput::new(
        Lines::Single,
        DOMString::from(""),
        DummyClipboardContext::new(""),
        //Carapace: Change UTF16CodeUnits to have named fields
        Some(UTF16CodeUnits{value: 3}),
        None,
        SelectionDirection::None,
    );

    textinput.insert_char('\u{10437}');
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "\u{10437}");
    textinput.insert_char('\u{10437}');
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "\u{10437}");
    textinput.insert_char('x');
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "\u{10437}x");
    textinput.insert_char('x');
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "\u{10437}x");
}

#[test]
fn test_single_line_textinput_with_max_length_inside_char() {
    let mut textinput = TextInput::new(
        Lines::Single,
        DOMString::from("\u{10437}"),
        DummyClipboardContext::new(""),
        Some(UTF16CodeUnits::one()),
        None,
        SelectionDirection::None,
    );

    textinput.insert_char('x');
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "\u{10437}");
}

#[test]
fn test_single_line_textinput_with_max_length_doesnt_allow_appending_characters_after_max_length_is_reached(
) {
    let mut textinput = TextInput::new(
        Lines::Single,
        DOMString::from("a"),
        DummyClipboardContext::new(""),
        Some(UTF16CodeUnits::one()),
        None,
        SelectionDirection::None,
    );

    textinput.insert_char('b');
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "a");
}

#[test]
fn test_textinput_delete_char() {
    let mut textinput = text_input(Lines::Single, "abcdefg");
    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 2}, Direction::Forward, Selection::NotSelected);
    textinput.delete_char(Direction::Backward);
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "acdefg");

    textinput.delete_char(Direction::Forward);
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "adefg");

    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 2}, Direction::Forward, Selection::Selected);
    textinput.delete_char(Direction::Forward);
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "afg");

    let mut textinput = text_input(Lines::Single, "a🌠b");
    // Same as "Right" key
    textinput.adjust_horizontal_by_one(Direction::Forward, Selection::NotSelected);
    textinput.delete_char(Direction::Forward);
    // Not splitting surrogate pairs.
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "ab");

    let mut textinput = text_input(Lines::Single, "abcdefg");
    textinput.set_selection_range(2, 2, SelectionDirection::None);
    textinput.delete_char(Direction::Backward);
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "acdefg");
}

#[test]
fn test_textinput_insert_char() {
    let mut textinput = text_input(Lines::Single, "abcdefg");
    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 2}, Direction::Forward, Selection::NotSelected);
    textinput.insert_char('a');
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "abacdefg");

    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 2}, Direction::Forward, Selection::Selected);
    textinput.insert_char('b');
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "ababefg");

    let mut textinput = text_input(Lines::Single, "a🌠c");
    // Same as "Right" key
    textinput.adjust_horizontal_by_one(Direction::Forward, Selection::NotSelected);
    textinput.adjust_horizontal_by_one(Direction::Forward, Selection::NotSelected);
    textinput.insert_char('b');
    // Not splitting surrogate pairs.
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "a🌠bc");
}

#[test]
fn test_textinput_get_sorted_selection() {
    let mut textinput = text_input(Lines::Single, "abcdefg");
    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 2}, Direction::Forward, Selection::NotSelected);
    textinput.adjust_horizontal(UTF8Bytes{value: 2}, Direction::Forward, Selection::Selected);
    let (start, end) = textinput.sorted_selection_bounds();
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(start.index, UTF8Bytes{value: 2});
    assert_eq!(end.index, UTF8Bytes{value: 4});

    textinput.clear_selection();

    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 2}, Direction::Backward, Selection::Selected);
    let (start, end) = textinput.sorted_selection_bounds();
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(start.index, UTF8Bytes{value: 2});
    assert_eq!(end.index, UTF8Bytes{value: 4});
}

#[test]
fn test_textinput_replace_selection() {
    let mut textinput = text_input(Lines::Single, "abcdefg");
    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 2}, Direction::Forward, Selection::NotSelected);
    textinput.adjust_horizontal(UTF8Bytes{value: 2}, Direction::Forward, Selection::Selected);

    //Carapace: Changed function call to reflect Carapace API
    textinput.replace_selection({let ds = DOMString::from_str("xyz"); info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(ds)})});
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "abxyzefg");
}

#[test]
fn test_textinput_replace_selection_multibyte_char() {
    let mut textinput = text_input(Lines::Single, "é");
    textinput.adjust_horizontal_by_one(Direction::Forward, Selection::Selected);

    //Carapace: Changed function call to reflect Carapace API
    textinput.replace_selection({let ds = DOMString::from_str("e"); info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(ds)})});
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "e");
}

#[test]
fn test_textinput_current_line_length() {
    let mut textinput = text_input(Lines::Multiple, "abc\nde\nf");
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.current_line_length(), UTF8Bytes{value: 3});

    textinput.adjust_vertical(1, Selection::NotSelected);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.current_line_length(), UTF8Bytes{value: 2});

    textinput.adjust_vertical(1, Selection::NotSelected);
    assert_eq!(textinput.current_line_length(), UTF8Bytes::one());
}

#[test]
fn test_textinput_adjust_vertical() {
    let mut textinput = text_input(Lines::Multiple, "abc\nde\nf");
    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 3}, Direction::Forward, Selection::NotSelected);
    textinput.adjust_vertical(1, Selection::NotSelected);
    assert_eq!(textinput.edit_point().line, 1);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 2});

    textinput.adjust_vertical(-1, Selection::NotSelected);
    assert_eq!(textinput.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 2});

    textinput.adjust_vertical(2, Selection::NotSelected);
    assert_eq!(textinput.edit_point().line, 2);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 1});

    textinput.adjust_vertical(-1, Selection::Selected);
    assert_eq!(textinput.edit_point().line, 1);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 1});
}

#[test]
fn test_textinput_adjust_vertical_multibyte() {
    let mut textinput = text_input(Lines::Multiple, "áé\nae");

    textinput.adjust_horizontal_by_one(Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 2});

    textinput.adjust_vertical(1, Selection::NotSelected);
    assert_eq!(textinput.edit_point().line, 1);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 1});
}

#[test]
fn test_textinput_adjust_horizontal() {
    let mut textinput = text_input(Lines::Multiple, "abc\nde\nf");
    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 4}, Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput.edit_point().line, 1);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());

    textinput.adjust_horizontal(UTF8Bytes::one(), Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput.edit_point().line, 1);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 1});

    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 2}, Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput.edit_point().line, 2);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());

    textinput.adjust_horizontal(
        UTF8Bytes::one(),
        Direction::Backward,
        Selection::NotSelected,
    );
    assert_eq!(textinput.edit_point().line, 1);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 2});
}

#[test]
fn test_textinput_adjust_horizontal_by_word() {
    // Test basic case of movement word by word based on UAX#29 rules
    let mut textinput = text_input(Lines::Single, "abc def");
    textinput.adjust_horizontal_by_word(Direction::Forward, Selection::NotSelected);
    textinput.adjust_horizontal_by_word(Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 7});
    textinput.adjust_horizontal_by_word(Direction::Backward, Selection::NotSelected);
    assert_eq!(textinput.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 4});
    textinput.adjust_horizontal_by_word(Direction::Backward, Selection::NotSelected);
    assert_eq!(textinput.edit_point().line, 0);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());

    // Test new line case of movement word by word based on UAX#29 rules
    let mut textinput_2 = text_input(Lines::Multiple, "abc\ndef");
    textinput_2.adjust_horizontal_by_word(Direction::Forward, Selection::NotSelected);
    textinput_2.adjust_horizontal_by_word(Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput_2.edit_point().line, 1);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput_2.edit_point().index, UTF8Bytes{value: 3});
    textinput_2.adjust_horizontal_by_word(Direction::Backward, Selection::NotSelected);
    assert_eq!(textinput_2.edit_point().line, 1);
    assert_eq!(textinput_2.edit_point().index, UTF8Bytes::zero());
    textinput_2.adjust_horizontal_by_word(Direction::Backward, Selection::NotSelected);
    assert_eq!(textinput_2.edit_point().line, 0);
    assert_eq!(textinput_2.edit_point().index, UTF8Bytes::zero());

    // Test non-standard sized characters case of movement word by word based on UAX#29 rules
    let mut textinput_3 = text_input(Lines::Single, "áéc d🌠bc");
    textinput_3.adjust_horizontal_by_word(Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput_3.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput_3.edit_point().index, UTF8Bytes{value: 5});
    textinput_3.adjust_horizontal_by_word(Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput_3.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput_3.edit_point().index, UTF8Bytes{value: 7});
    textinput_3.adjust_horizontal_by_word(Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput_3.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput_3.edit_point().index, UTF8Bytes{value: 13});
    textinput_3.adjust_horizontal_by_word(Direction::Backward, Selection::NotSelected);
    assert_eq!(textinput_3.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput_3.edit_point().index, UTF8Bytes{value: 11});
    textinput_3.adjust_horizontal_by_word(Direction::Backward, Selection::NotSelected);
    assert_eq!(textinput_3.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput_3.edit_point().index, UTF8Bytes{value: 6});
}

#[test]
fn test_textinput_adjust_horizontal_to_line_end() {
    // Test standard case of movement to end based on UAX#29 rules
    let mut textinput = text_input(Lines::Single, "abc def");
    textinput.adjust_horizontal_to_line_end(Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 7});

    // Test new line case of movement to end based on UAX#29 rules
    let mut textinput_2 = text_input(Lines::Multiple, "abc\ndef");
    textinput_2.adjust_horizontal_to_line_end(Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput_2.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput_2.edit_point().index, UTF8Bytes{value: 3});
    textinput_2.adjust_horizontal_to_line_end(Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput_2.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput_2.edit_point().index, UTF8Bytes{value: 3});
    textinput_2.adjust_horizontal_to_line_end(Direction::Backward, Selection::NotSelected);
    assert_eq!(textinput_2.edit_point().line, 0);
    assert_eq!(textinput_2.edit_point().index, UTF8Bytes::zero());

    // Test non-standard sized characters case of movement to end based on UAX#29 rules
    let mut textinput_3 = text_input(Lines::Single, "áéc d🌠bc");
    textinput_3.adjust_horizontal_to_line_end(Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput_3.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput_3.edit_point().index, UTF8Bytes{value: 13});
    textinput_3.adjust_horizontal_to_line_end(Direction::Backward, Selection::NotSelected);
    assert_eq!(textinput_3.edit_point().line, 0);
    assert_eq!(textinput_3.edit_point().index, UTF8Bytes::zero());
}

#[test]
fn test_navigation_keyboard_shortcuts() {
    let mut textinput = text_input(Lines::Multiple, "hello áéc");

    // Test that CMD + Right moves to the end of the current line.
    //Carapace: Changed function call to reflect Carapace API
    textinput.handle_keydown_aux({let k = KeyWrapper{k: Key::ArrowRight }; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(k)})}, {let m = ModifiersWrapper{m: Modifiers::META}; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(m)})}, true);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 11});
    // Test that CMD + Right moves to the beginning of the current line.
    //Carapace: Changed function call to reflect Carapace API
    textinput.handle_keydown_aux({let k = KeyWrapper{k: Key::ArrowLeft }; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(k)})}, {let m = ModifiersWrapper{m: Modifiers::META}; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(m)})}, true);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());
    // Test that CTRL + ALT + E moves to the end of the current line also.
    //Carapace: Changed function call to reflect Carapace API
    textinput.handle_keydown_aux({let k = KeyWrapper{k: Key::Character("e".to_owned()) }; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(k)})}, {let m = ModifiersWrapper{m: Modifiers::CONTROL | Modifiers::ALT}; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(m)})}, true);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 11});
    // Test that CTRL + ALT + A moves to the beginning of the current line also.
    //Carapace: Changed function call to reflect Carapace API
    textinput.handle_keydown_aux({let k = KeyWrapper{k: Key::Character("a".to_owned()) }; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(k)})}, {let m = ModifiersWrapper{m: Modifiers::CONTROL | Modifiers::ALT}; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(m)})}, true);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());

    // Test that ALT + Right moves to the end of the word.
    //Carapace: Changed function call to reflect Carapace API
    textinput.handle_keydown_aux({let k = KeyWrapper{k: Key::ArrowRight }; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(k)})}, {let m = ModifiersWrapper{m: Modifiers::ALT}; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(m)})}, true);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 5});
    // Test that CTRL + ALT + F moves to the end of the word also.
    //Carapace: Changed function call to reflect Carapace API
    textinput.handle_keydown_aux({let k = KeyWrapper{k: Key::Character("f".to_owned()) }; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(k)})}, {let m = ModifiersWrapper{m: Modifiers::CONTROL | Modifiers::ALT}; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(m)})}, true);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 11});
    // Test that ALT + Left moves to the end of the word.
    //Carapace: Changed function call to reflect Carapace API
    textinput.handle_keydown_aux({let k = KeyWrapper{k: Key::ArrowLeft }; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(k)})}, {let m = ModifiersWrapper{m: Modifiers::ALT}; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(m)})}, true);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 6});
    // Test that CTRL + ALT + B moves to the end of the word also.
    //Carapace: Changed function call to reflect Carapace API
    textinput.handle_keydown_aux({let k = KeyWrapper{k: Key::Character("b".to_owned()) }; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(k)})}, {let m = ModifiersWrapper{m: Modifiers::CONTROL | Modifiers::ALT}; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(m)})}, true);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());
}

#[test]
fn test_textinput_handle_return() {
    let mut single_line_textinput = text_input(Lines::Single, "abcdef");
    single_line_textinput.adjust_horizontal(
        //Carapace: Change UTF8Bytes to have named fields
        UTF8Bytes{value: 3},
        Direction::Forward,
        Selection::NotSelected,
    );
    single_line_textinput.handle_return();
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = single_line_textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "abcdef");

    let mut multi_line_textinput = text_input(Lines::Multiple, "abcdef");
    multi_line_textinput.adjust_horizontal(
        //Carapace: Change UTF8Bytes to have named fields
        UTF8Bytes{value: 3},
        Direction::Forward,
        Selection::NotSelected,
    );
    multi_line_textinput.handle_return();
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = multi_line_textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "abc\ndef");
}

#[test]
fn test_textinput_select_all() {
    let mut textinput = text_input(Lines::Multiple, "abc\nde\nf");
    assert_eq!(textinput.edit_point().line, 0);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());

    textinput.select_all();
    assert_eq!(textinput.edit_point().line, 2);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 1});
}

#[test]
fn test_textinput_get_content() {
    let single_line_textinput = text_input(Lines::Single, "abcdefg");
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = single_line_textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "abcdefg");

    let multi_line_textinput = text_input(Lines::Multiple, "abc\nde\nf");
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = multi_line_textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "abc\nde\nf");
}

#[test]
fn test_textinput_set_content() {
    let mut textinput = text_input(Lines::Multiple, "abc\nde\nf");
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "abc\nde\nf");

    //Carapace: Changed function call to reflect Carapace API
    textinput.set_content(info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), { wrap_secret(DOMString::from_str("abc\nf"))}));
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "abc\nf");

    assert_eq!(textinput.edit_point().line, 0);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());

    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 3}, Direction::Forward, Selection::Selected);
    assert_eq!(textinput.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 3});
    //Carapace: Changed function call to reflect Carapace API
    textinput.set_content(info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), { wrap_secret(DOMString::from_str("de"))}));
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "de");
    assert_eq!(textinput.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 2});
}

#[test]
fn test_clipboard_paste() {
    #[cfg(target_os = "macos")]
    const MODIFIERS: Modifiers = Modifiers::META;
    #[cfg(not(target_os = "macos"))]
    const MODIFIERS: Modifiers = Modifiers::CONTROL;

    let mut textinput = TextInput::new(
        Lines::Single,
        DOMString::from("defg"),
        DummyClipboardContext::new("abc"),
        None,
        None,
        SelectionDirection::None,
    );
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "defg");
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());
    //Carapace: Changed function call to reflect Carapace API
    textinput.handle_keydown_aux({let k = KeyWrapper{k: Key::Character("v".to_owned()) }; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(k)})}, {let m = ModifiersWrapper{m: MODIFIERS}; info_flow_block_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, new_dynamic_secret_label(vec![]), new_dynamic_integrity_label(vec![]), {wrap_secret(m)})}, false);
    //Carapace: Changed test to reflect Carapace API
    assert_eq!({let result = textinput.get_content(); info_flow_block_declassify_dynamic_all!(sec_lat::Label_Empty, int_lat::Label_All, result.get_dynamic_secret_label_clone(), result.get_dynamic_integrity_label_clone(), {unwrap_secret(result)})}, "abcdefg");
}

#[test]
fn test_textinput_cursor_position_correct_after_clearing_selection() {
    let mut textinput = text_input(Lines::Single, "abcdef");

    // Single line - Forward
    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 3}, Direction::Forward, Selection::Selected);
    textinput.adjust_horizontal(UTF8Bytes::one(), Direction::Forward, Selection::NotSelected);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 3});

    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 3}, Direction::Backward, Selection::NotSelected);
    textinput.adjust_horizontal(UTF8Bytes{value: 3}, Direction::Forward, Selection::Selected);
    textinput.adjust_horizontal_by_one(Direction::Forward, Selection::NotSelected);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 3});

    // Single line - Backward
    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 3}, Direction::Backward, Selection::NotSelected);
    textinput.adjust_horizontal(UTF8Bytes{value: 3}, Direction::Forward, Selection::Selected);
    textinput.adjust_horizontal(
        UTF8Bytes::one(),
        Direction::Backward,
        Selection::NotSelected,
    );
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());

    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 3}, Direction::Backward, Selection::NotSelected);
    textinput.adjust_horizontal(UTF8Bytes{value: 3}, Direction::Forward, Selection::Selected);
    textinput.adjust_horizontal_by_one(Direction::Backward, Selection::NotSelected);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());

    let mut textinput = text_input(Lines::Multiple, "abc\nde\nf");

    // Multiline - Forward
    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 4}, Direction::Forward, Selection::Selected);
    textinput.adjust_horizontal(UTF8Bytes::one(), Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());
    assert_eq!(textinput.edit_point().line, 1);

    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 4}, Direction::Backward, Selection::NotSelected);
    textinput.adjust_horizontal(UTF8Bytes{value: 4}, Direction::Forward, Selection::Selected);
    textinput.adjust_horizontal_by_one(Direction::Forward, Selection::NotSelected);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());
    assert_eq!(textinput.edit_point().line, 1);

    // Multiline - Backward
    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 4}, Direction::Backward, Selection::NotSelected);
    textinput.adjust_horizontal(UTF8Bytes{value: 4}, Direction::Forward, Selection::Selected);
    textinput.adjust_horizontal(
        UTF8Bytes::one(),
        Direction::Backward,
        Selection::NotSelected,
    );
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());
    assert_eq!(textinput.edit_point().line, 0);

    //Carapace: Change UTF8Bytes to have named fields
    textinput.adjust_horizontal(UTF8Bytes{value: 4}, Direction::Backward, Selection::NotSelected);
    textinput.adjust_horizontal(UTF8Bytes{value: 4}, Direction::Forward, Selection::Selected);
    textinput.adjust_horizontal_by_one(Direction::Backward, Selection::NotSelected);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());
    assert_eq!(textinput.edit_point().line, 0);
}

#[test]
fn test_textinput_set_selection_with_direction() {
    let mut textinput = text_input(Lines::Single, "abcdef");
    textinput.set_selection_range(2, 6, SelectionDirection::Forward);
    assert_eq!(textinput.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 6});
    assert_eq!(textinput.selection_direction(), SelectionDirection::Forward);

    assert!(textinput.selection_origin().is_some());
    assert_eq!(textinput.selection_origin().unwrap().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.selection_origin().unwrap().index, UTF8Bytes{value: 2});

    textinput.set_selection_range(2, 6, SelectionDirection::Backward);
    assert_eq!(textinput.edit_point().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 2});
    assert_eq!(
        textinput.selection_direction(),
        SelectionDirection::Backward
    );

    assert!(textinput.selection_origin().is_some());
    assert_eq!(textinput.selection_origin().unwrap().line, 0);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.selection_origin().unwrap().index, UTF8Bytes{value: 6});

    textinput = text_input(Lines::Multiple, "\n\n");
    textinput.set_selection_range(0, 1, SelectionDirection::Forward);
    assert_eq!(textinput.edit_point().line, 1);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());
    assert_eq!(textinput.selection_direction(), SelectionDirection::Forward);

    assert!(textinput.selection_origin().is_some());
    assert_eq!(textinput.selection_origin().unwrap().line, 0);
    assert_eq!(
        textinput.selection_origin().unwrap().index,
        UTF8Bytes::zero()
    );

    textinput = text_input(Lines::Multiple, "\n");
    textinput.set_selection_range(0, 1, SelectionDirection::Forward);
    assert_eq!(textinput.edit_point().line, 1);
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());
    assert_eq!(textinput.selection_direction(), SelectionDirection::Forward);

    assert!(textinput.selection_origin().is_some());
    assert_eq!(textinput.selection_origin().unwrap().line, 0);
    assert_eq!(
        textinput.selection_origin().unwrap().index,
        UTF8Bytes::zero()
    );
}

#[test]
fn test_textinput_unicode_handling() {
    let mut textinput = text_input(Lines::Single, "éèùµ$£");
    assert_eq!(textinput.edit_point().index, UTF8Bytes::zero());
    textinput.set_edit_point_index(1);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 2});
    textinput.set_edit_point_index(4);
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(textinput.edit_point().index, UTF8Bytes{value: 8});
}

#[test]
fn test_selection_bounds() {
    let mut textinput = text_input(Lines::Single, "abcdef");

    assert_eq!(
        TextPoint {
            line: 0,
            index: UTF8Bytes::zero()
        },
        textinput.selection_origin_or_edit_point()
    );
    assert_eq!(
        TextPoint {
            line: 0,
            index: UTF8Bytes::zero()
        },
        textinput.selection_start()
    );
    assert_eq!(
        TextPoint {
            line: 0,
            index: UTF8Bytes::zero()
        },
        textinput.selection_end()
    );

    textinput.set_selection_range(2, 5, SelectionDirection::Forward);
    assert_eq!(
        TextPoint {
            line: 0,
            //Carapace: Change UTF8Bytes to have named fields
            index: UTF8Bytes{value: 2}
        },
        textinput.selection_origin_or_edit_point()
    );
    assert_eq!(
        TextPoint {
            line: 0,
            //Carapace: Change UTF8Bytes to have named fields
            index: UTF8Bytes{value: 2}
        },
        textinput.selection_start()
    );
    assert_eq!(
        TextPoint {
            line: 0,
            //Carapace: Change UTF8Bytes to have named fields
            index: UTF8Bytes{value: 5}
        },
        textinput.selection_end()
    );
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(UTF8Bytes{value: 2}, textinput.selection_start_offset());
    assert_eq!(UTF8Bytes{value: 5}, textinput.selection_end_offset());

    textinput.set_selection_range(3, 6, SelectionDirection::Backward);
    assert_eq!(
        TextPoint {
            line: 0,
            //Carapace: Change UTF8Bytes to have named fields
            index: UTF8Bytes{value: 6}
        },
        textinput.selection_origin_or_edit_point()
    );
    assert_eq!(
        TextPoint {
            line: 0,
            //Carapace: Change UTF8Bytes to have named fields
            index: UTF8Bytes{value: 3}
        },
        textinput.selection_start()
    );
    assert_eq!(
        TextPoint {
            line: 0,
            //Carapace: Change UTF8Bytes to have named fields
            index: UTF8Bytes{value: 6}
        },
        textinput.selection_end()
    );
    //Carapace: Change UTF8Bytes to have named fields
    assert_eq!(UTF8Bytes{value: 3}, textinput.selection_start_offset());
    assert_eq!(UTF8Bytes{value: 6}, textinput.selection_end_offset());

    textinput = text_input(Lines::Multiple, "\n\n");
    textinput.set_selection_range(0, 1, SelectionDirection::Forward);
    assert_eq!(
        TextPoint {
            line: 0,
            index: UTF8Bytes::zero()
        },
        textinput.selection_origin_or_edit_point()
    );
    assert_eq!(
        TextPoint {
            line: 0,
            index: UTF8Bytes::zero()
        },
        textinput.selection_start()
    );
    assert_eq!(
        TextPoint {
            line: 1,
            index: UTF8Bytes::zero()
        },
        textinput.selection_end()
    );
}

#[test]
fn test_select_all() {
    let mut textinput = text_input(Lines::Single, "abc");
    textinput.set_selection_range(2, 3, SelectionDirection::Backward);
    textinput.select_all();
    assert_eq!(textinput.selection_direction(), SelectionDirection::Forward);
    assert_eq!(
        TextPoint {
            line: 0,
            index: UTF8Bytes::zero()
        },
        textinput.selection_start()
    );
    assert_eq!(
        TextPoint {
            line: 0,
            //Carapace: Change UTF8Bytes to have named fields
            index: UTF8Bytes{value: 3}
        },
        textinput.selection_end()
    );
}
