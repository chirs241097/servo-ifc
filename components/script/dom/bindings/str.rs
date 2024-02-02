/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! The `ByteString` struct.
use chrono::prelude::{Utc, Weekday};
use chrono::{Datelike, TimeZone};
use cssparser::CowRcStr;
use html5ever::{LocalName, Namespace};
use regex::Regex;
use servo_atoms::Atom;
use std::borrow::{Borrow, Cow, ToOwned};
use std::default::Default;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops;
use std::ops::{Deref, DerefMut};
use std::str;
use std::str::FromStr;

//Carapace: Add imports
use secret_macros::*;
use secret_structs::secret::*;
use secret_structs::ternary_lattice as sec_lat;
use secret_structs::integrity_lattice as int_lat;

#[side_effect_free_attr_full]
//Map<Split<'_, Fn(char) -> bool>, FnMut(&str) -> DOMString>
pub fn custom_collect_unwrapped<F1: Fn(char) -> bool, F2: FnMut(&str) -> DOMString>(self_: std::iter::Map<std::str::Split<'_, F1>, F2>) -> Vec<DOMString> {
    unchecked_operation(self_.collect())
}

#[side_effect_free_attr_full]
pub fn custom_collect_wrapped<F1: Fn(char) -> bool, F2: FnMut(&str) -> InfoFlowStruct<DOMString, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>>(self_: std::iter::Map<std::str::Split<'_, F1>, F2>) -> Vec<InfoFlowStruct<DOMString, sec_lat::Label_Empty, int_lat::Label_All, DynamicSecretLabel, DynamicIntegrityLabel>> {
    unchecked_operation(self_.collect())
}

/// Encapsulates the IDL `ByteString` type.
#[derive(Clone, Debug, Default, Eq, JSTraceable, MallocSizeOf, PartialEq)]
pub struct ByteString(Vec<u8>);

impl ByteString {
    /// Creates a new `ByteString`.
    pub fn new(value: Vec<u8>) -> ByteString {
        ByteString(value)
    }

    /// Returns `self` as a string, if it encodes valid UTF-8, and `None`
    /// otherwise.
    pub fn as_str(&self) -> Option<&str> {
        str::from_utf8(&self.0).ok()
    }

    /// Returns the length.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `self` with A–Z replaced by a–z.
    pub fn to_lower(&self) -> ByteString {
        ByteString::new(self.0.to_ascii_lowercase())
    }
}

impl Into<Vec<u8>> for ByteString {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

impl Hash for ByteString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl FromStr for ByteString {
    type Err = ();
    fn from_str(s: &str) -> Result<ByteString, ()> {
        Ok(ByteString::new(s.to_owned().into_bytes()))
    }
}

impl ops::Deref for ByteString {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

/// A string that is constructed from a UCS-2 buffer by replacing invalid code
/// points with the replacement character.
#[derive(Clone, Default, Eq, Hash, MallocSizeOf, Ord, PartialEq, PartialOrd)]
pub struct USVString(pub String);

impl Borrow<str> for USVString {
    #[inline]
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Deref for USVString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl DerefMut for USVString {
    #[inline]
    fn deref_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl AsRef<str> for USVString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for USVString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl PartialEq<str> for USVString {
    fn eq(&self, other: &str) -> bool {
        &**self == other
    }
}

impl<'a> PartialEq<&'a str> for USVString {
    fn eq(&self, other: &&'a str) -> bool {
        &**self == *other
    }
}

impl From<String> for USVString {
    fn from(contents: String) -> USVString {
        USVString(contents)
    }
}

/// Returns whether `s` is a `token`, as defined by
/// [RFC 2616](http://tools.ietf.org/html/rfc2616#page-17).
pub fn is_token(s: &[u8]) -> bool {
    if s.is_empty() {
        return false; // A token must be at least a single character
    }
    s.iter().all(|&x| {
        // http://tools.ietf.org/html/rfc2616#section-2.2
        match x {
            0..=31 | 127 => false, // CTLs
            40 | 41 | 60 | 62 | 64 | 44 | 59 | 58 | 92 | 34 | 47 | 91 | 93 | 63 | 61 | 123 |
            125 | 32 => false, // separators
            x if x > 127 => false, // non-CHARs
            _ => true,
        }
    })
}

/// A DOMString.
///
/// This type corresponds to the [`DOMString`](idl) type in WebIDL.
///
/// [idl]: https://heycam.github.io/webidl/#idl-DOMString
///
/// Conceptually, a DOMString has the same value space as a JavaScript String,
/// i.e., an array of 16-bit *code units* representing UTF-16, potentially with
/// unpaired surrogates present (also sometimes called WTF-16).
///
/// Currently, this type stores a Rust `String`, in order to avoid issues when
/// integrating with the rest of the Rust ecosystem and even the rest of the
/// browser itself.
///
/// However, Rust `String`s are guaranteed to be valid UTF-8, and as such have
/// a *smaller value space* than WTF-16 (i.e., some JavaScript String values
/// can not be represented as a Rust `String`). This introduces the question of
/// what to do with values being passed from JavaScript to Rust that contain
/// unpaired surrogates.
///
/// The hypothesis is that it does not matter much how exactly those values are
/// transformed, because passing unpaired surrogates into the DOM is very rare.
/// In order to test this hypothesis, Servo will panic when encountering any
/// unpaired surrogates on conversion to `DOMString` by default. (The command
/// line option `-Z replace-surrogates` instead causes Servo to replace the
/// unpaired surrogate by a U+FFFD replacement character.)
///
/// Currently, the lack of crash reports about this issue provides some
/// evidence to support the hypothesis. This evidence will hopefully be used to
/// convince other browser vendors that it would be safe to replace unpaired
/// surrogates at the boundary between JavaScript and native code. (This would
/// unify the `DOMString` and `USVString` types, both in the WebIDL standard
/// and in Servo.)
///
/// This type is currently `!Send`, in order to help with an independent
/// experiment to store `JSString`s rather than Rust `String`s.
#[derive(Clone, Debug, Eq, Hash, MallocSizeOf, Ord, PartialEq, PartialOrd)]
//Carapace: Change DOMString to have named fields.
pub struct DOMString{s: String, p: PhantomData<*const ()>}
//Carapace: Impl InvisibleSideEffectFree for DOMString so it can be used in IFC blocks.
unsafe impl InvisibleSideEffectFree for DOMString{}

enum TimeParserState {
    HourHigh,
    HourLow09,
    HourLow03,
    MinuteColon,
    MinuteHigh,
    MinuteLow,
    SecondColon,
    SecondHigh,
    SecondLow,
    MilliStop,
    MilliHigh,
    MilliMiddle,
    MilliLow,
    Done,
    Error,
}
unsafe impl InvisibleSideEffectFree for TimeParserState{}

#[side_effect_free_attr_full]
fn next_time_parser_state(valid: bool, next: TimeParserState) -> TimeParserState {
    if valid {
        next
    } else {
        TimeParserState::Error
    }
}

impl DOMString {
    //Carapace: Add functions to replace Deref Coerction
    #[side_effect_free_attr_full(method)]
    pub fn to_str_ref(&self) -> &str {
        &self.s
    }
    #[side_effect_free_attr_full(method)]
    pub fn to_owned(self) -> String {
        self.s
    }
    #[side_effect_free_attr_full(method)]
    pub fn replace_content(&mut self, s: String) {
        self.s = s;
    }

    #[side_effect_free_attr_full(method)]
    pub fn to_string_ref(&self) -> &String {
        &self.s
    }

    #[side_effect_free_attr_full(method)]
    pub fn to_mut_string_ref(&mut self) -> &mut String {
        &mut self.s
    }

    /// Creates a new `DOMString`.
    //Carapace: Tag function as side_effect_free
    #[side_effect_free_attr_full(method)]
    pub fn new() -> DOMString {
        //Carapace: Use DOMString with named fields
        DOMString{s: std::string::String::new(), p: PhantomData}
    }

    /// Creates a new `DOMString` from a `String`.
    //Carapace: Tag function as side_effect_free
    #[side_effect_free_attr_full(method)]
    pub fn from_string(s: String) -> DOMString {
        //Carapace: Use DOMString with named fields
        DOMString{s: s, p: PhantomData}
    }

    /// Creates a new `DOMString` from a `&str`.
    //Carapace: Tag function as side_effect_free
    #[side_effect_free_attr_full(method)]
    pub fn from_str(contents: &str) -> DOMString {
        //Carapace: Use DOMString with named fields
        DOMString::from_string(std::string::String::from(contents))
    }

    /// Appends a given string slice onto the end of this String.
    #[side_effect_free_attr_full(method)]
    pub fn push_str(&mut self, string: &str) {
        std::string::String::push_str(&mut self.s, string)
    }

    /// Clears this `DOMString`, removing all contents.
    #[side_effect_free_attr_full(method)]
    pub fn clear(&mut self) {
        std::string::String::clear(&mut self.s)
    }

    /// Shortens this String to the specified length.
    pub fn truncate(&mut self, new_len: usize) {
        self.s.truncate(new_len);
    }

    /// Removes newline characters according to <https://infra.spec.whatwg.org/#strip-newlines>.
    #[side_effect_free_attr_full(method)]
    pub fn strip_newlines(&mut self) {
        std::string::String::retain(&mut self.s, |c| c != '\r' && c != '\n');
    }

    /// Removes leading and trailing ASCII whitespaces according to
    /// <https://infra.spec.whatwg.org/#strip-leading-and-trailing-ascii-whitespace>.
    #[side_effect_free_attr_full(method)]
    pub fn strip_leading_and_trailing_ascii_whitespace(&mut self) {
        if std::string::String::len(&self.s) == 0 {
            return;
        }

        let trailing_whitespace_len = core::primitive::str::len(
            core::primitive::str::trim_end_matches(&self.s, |ref c| core::primitive::char::is_ascii_whitespace(c))
        );
        std::string::String::truncate(&mut self.s, trailing_whitespace_len);
        if std::string::String::is_empty(&self.s) {
            return;
        }

        let first_non_whitespace = std::option::Option::unwrap(core::primitive::str::find(&self.s, |ref c| !core::primitive::char::is_ascii_whitespace(c)));
        let _ = std::string::String::replace_range(&mut self.s, 0..first_non_whitespace, "");
    }

    /// Validates this `DOMString` is a time string according to
    /// <https://html.spec.whatwg.org/multipage/#valid-time-string>.
    #[side_effect_free_attr_full(method)]
    pub fn is_valid_time_string(&self) -> bool {
        let state = std::str::Chars::fold(core::primitive::str::chars(&self.s), TimeParserState::HourHigh, |state, c| {
            match state {
                // Step 1 "HH"
                TimeParserState::HourHigh => match c {
                    '0' | '1' => TimeParserState::HourLow09,
                    '2' => TimeParserState::HourLow03,
                    _ => TimeParserState::Error,
                },
                TimeParserState::HourLow09 => next_time_parser_state(core::primitive::char::is_digit(c, 10), TimeParserState::MinuteColon),
                TimeParserState::HourLow03 => next_time_parser_state(core::primitive::char::is_digit(c, 4), TimeParserState::MinuteColon),

                // Step 2 ":"
                TimeParserState::MinuteColon => next_time_parser_state(c == ':', TimeParserState::MinuteHigh),

                // Step 3 "mm"
                TimeParserState::MinuteHigh => next_time_parser_state(core::primitive::char::is_digit(c, 6), TimeParserState::MinuteLow),
                TimeParserState::MinuteLow => next_time_parser_state(core::primitive::char::is_digit(c, 10), TimeParserState::SecondColon),

                // Step 4.1 ":"
                TimeParserState::SecondColon => next_time_parser_state(c == ':', TimeParserState::SecondHigh),
                // Step 4.2 "ss"
                TimeParserState::SecondHigh => next_time_parser_state(core::primitive::char::is_digit(c, 6), TimeParserState::SecondLow),
                TimeParserState::SecondLow => next_time_parser_state(core::primitive::char::is_digit(c, 10), TimeParserState::MilliStop),

                // Step 4.3.1 "."
                TimeParserState::MilliStop => next_time_parser_state(c == '.', TimeParserState::MilliHigh),
                // Step 4.3.2 "SSS"
                TimeParserState::MilliHigh => next_time_parser_state(core::primitive::char::is_digit(c, 10), TimeParserState::MilliMiddle),
                TimeParserState::MilliMiddle => next_time_parser_state(core::primitive::char::is_digit(c, 10), TimeParserState::MilliLow),
                TimeParserState::MilliLow => next_time_parser_state(core::primitive::char::is_digit(c, 10), TimeParserState::Done),

                _ => TimeParserState::Error,
            }
        });

        match state {
            TimeParserState::Done |
            // Step 4 (optional)
            TimeParserState::SecondColon |
            // Step 4.3 (optional)
            TimeParserState::MilliStop |
            // Step 4.3.2 (only 1 digit required)
            TimeParserState::MilliMiddle | TimeParserState::MilliLow => true,
            _ => false
        }
    }

    /// A valid date string should be "YYYY-MM-DD"
    /// YYYY must be four or more digits, MM and DD both must be two digits
    /// https://html.spec.whatwg.org/multipage/#valid-date-string
    #[side_effect_free_attr_full(method)]
    pub fn is_valid_date_string(&self) -> bool {
        match DOMString::parse_date_string(self) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    /// https://html.spec.whatwg.org/multipage/#parse-a-date-string
    #[side_effect_free_attr_full(method)]
    pub fn parse_date_string(&self) -> Result<(i32, u32, u32), ()> {
        // Step 1, 2, 3
        let (year_int, month_int, day_int) = parse_date_component(&self.s)?;

        // Step 4
        match str::Split::nth(&mut core::primitive::str::split(&self.s, '-'), 3) {
            Some(_) => std::result::Result::Err(()),
            // Step 5, 6
            _ => std::result::Result::Ok((year_int, month_int, day_int))
        }
    }

    /// https://html.spec.whatwg.org/multipage/#parse-a-time-string
    #[side_effect_free_attr_full(method)]
    pub fn parse_time_string(&self) -> Result<(u32, u32, f64), ()> {
        // Step 1, 2, 3
        let (hour_int, minute_int, second_float) = parse_time_component(&self.s)?;

        // Step 4
        match str::Split::nth(&mut core::primitive::str::split(&self.s, ':'), 3) {
            Some(_) => std::result::Result::Err(()),
            // Step 5, 6
            _ => std::result::Result::Ok((hour_int, minute_int, second_float))
        }
    }

    /// A valid month string should be "YYYY-MM"
    /// YYYY must be four or more digits, MM both must be two digits
    /// https://html.spec.whatwg.org/multipage/#valid-month-string
    #[side_effect_free_attr_full(method)]
    pub fn is_valid_month_string(&self) -> bool {
        match DOMString::parse_month_string(&self) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    /// https://html.spec.whatwg.org/multipage/#parse-a-month-string
    #[side_effect_free_attr_full(method)]
    pub fn parse_month_string(&self) -> Result<(i32, u32), ()> {
        // Step 1, 2, 3
        let (year_int, month_int) = parse_month_component(&self.s)?;

        // Step 4
        match str::Split::nth(&mut core::primitive::str::split(&self.s, '-'), 2) {
            Some(_) => return std::result::Result::Err(()),
            // Step 5
            _ => std::result::Result::Ok((year_int, month_int))
        }
    }

    /// A valid week string should be like {YYYY}-W{WW}, such as "2017-W52"
    /// YYYY must be four or more digits, WW both must be two digits
    /// https://html.spec.whatwg.org/multipage/#valid-week-string
    #[side_effect_free_attr_full(method)]
    pub fn is_valid_week_string(&self) -> bool {
        match DOMString::parse_week_string(&self) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    /// https://html.spec.whatwg.org/multipage/#parse-a-week-string
    #[side_effect_free_attr_full(method)]
    pub fn parse_week_string(&self) -> Result<(i32, u32), ()> {
        // Step 1, 2, 3
        let mut iterator = core::primitive::str::split(&self.s, '-');
        let year = std::option::Option::ok_or(str::Split::next(&mut iterator), ())?;

        // Step 4
        let year_int = std::result::Result::map_err(core::primitive::str::parse::<i32>(&year), |_|())?;
        if core::primitive::str::len(&year) < 4 || year_int == 0 {
            return std::result::Result::Err(());
        }

        // Step 5, 6
        let week = std::option::Option::ok_or(str::Split::next(&mut iterator), ())?;
        let (week_first, week_last) = core::primitive::str::split_at(&week, 1);
        //if *week_first != "W"  {
        if secret_structs::secret::SafePartialEq::safe_ne(week_first, "W") {
            return std::result::Result::Err(());
        }

        // Step 7
        let week_int = std::result::Result::map_err(core::primitive::str::parse::<u32>(&week_last), |_|())?;
        if core::primitive::str::len(&week_last) != 2 {
            return std::result::Result::Err(());
        }

        // Step 8
        let max_week = max_week_in_year(year_int);

        // Step 9
        if week_int < 1 || week_int > max_week {
            return std::result::Result::Err(());
        }

        // Step 10
        match str::Split::next(&mut iterator) {
            Some(_) => std::result::Result::Err(()),
            None =>
                // Step 11
                std::result::Result::Ok((year_int, week_int))
        }
    }

    /// https://html.spec.whatwg.org/multipage/#valid-floating-point-number
    #[side_effect_free_attr_full(method)]
    pub fn is_valid_floating_point_number_string(&self) -> bool {
        unchecked_operation(DOMString::is_valid_floating_point_number_string_internals(&self))
    }

    fn is_valid_floating_point_number_string_internals(&self) -> bool {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^-?(?:\d+\.\d+|\d+|\.\d+)(?:(e|E)(\+|\-)?\d+)?$").unwrap();
        }
        RE.is_match(&self.s) && self.parse_floating_point_number().is_ok()
    }

    /// https://html.spec.whatwg.org/multipage/#rules-for-parsing-floating-point-number-values
    #[side_effect_free_attr_full(method)]
    pub fn parse_floating_point_number(&self) -> Result<f64, ()> {
        // Steps 15-16 are telling us things about IEEE rounding modes
        // for floating-point significands; this code assumes the Rust
        // compiler already matches them in any cases where
        // that actually matters. They are not
        // related to f64::round(), which is for rounding to integers.
        match core::primitive::str::parse::<f64>(core::primitive::str::trim(&self.s)) {
            Ok(val)
                if !(
                    // A valid number is the same as what rust considers to be valid,
                    // except for +1., NaN, and Infinity.
                    core::primitive::f64::is_infinite(val) ||
                        core::primitive::f64::is_nan(val) ||
                        core::primitive::str::ends_with(&self.s, ".") ||
                        core::primitive::str::starts_with(&self.s, "+")
                ) =>
            {
                std::result::Result::Ok(val)
            },
            _ => std::result::Result::Err(()),
        }
    }

    /// https://html.spec.whatwg.org/multipage/#best-representation-of-the-number-as-a-floating-point-number
    pub fn set_best_representation_of_the_floating_point_number(&mut self) {
        if let Ok(val) = self.parse_floating_point_number() {
            self.s = val.to_string();
        }
    }

    /// A valid normalized local date and time string should be "{date}T{time}"
    /// where date and time are both valid, and the time string must be as short as possible
    /// https://html.spec.whatwg.org/multipage/#valid-normalised-local-date-and-time-string
    #[side_effect_free_attr_full(method)]
    pub fn convert_valid_normalized_local_date_and_time_string(&mut self) -> Result<(), ()> {
        let ((year, month, day), (hour, minute, second)) =
            DOMString::parse_local_date_and_time_string(&self)?;
        if second == 0.0 {
            self.s = unchecked_operation(format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}",
                year, month, day, hour, minute
            ));
        } else if second < 10.0 {
            // we need exactly one leading zero on the seconds,
            // whatever their total string length might be
            self.s = unchecked_operation(format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:0{}",
                year, month, day, hour, minute, second
            ));
        } else {
            // we need no leading zeroes on the seconds
            self.s = unchecked_operation(format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{}",
                year, month, day, hour, minute, second
            ));
        }
        std::result::Result::Ok(())
    }

    /// https://html.spec.whatwg.org/multipage/#parse-a-local-date-and-time-string
    #[side_effect_free_attr_full(method)]
    pub fn parse_local_date_and_time_string(
        &self,
    ) -> Result<((i32, u32, u32), (u32, u32, f64)), ()> {
        // Step 1, 2, 4
        let mut iterator = if core::primitive::str::contains(&self.s, 'T') {
            core::primitive::str::split(&self.s, 'T')
        } else {
            core::primitive::str::split(&self.s, ' ')
        };

        // Step 3
        let date = std::option::Option::ok_or(str::Split::next(&mut iterator),())?;
        let date_tuple = parse_date_component(date)?;

        // Step 5
        let time = std::option::Option::ok_or(str::Split::next(&mut iterator),())?;
        let time_tuple = parse_time_component(time)?;

        // Step 6
        match str::Split::next(&mut iterator) {
            Some(_) => std::result::Result::Err(()),
            None =>
                // Step 7, 8, 9
                std::result::Result::Ok((date_tuple, time_tuple))
        }
    }

    /// https://html.spec.whatwg.org/multipage/#valid-e-mail-address
    pub fn is_valid_email_address_string(&self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(concat!(
                r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?",
                r"(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
            ))
            .unwrap();
        }
        RE.is_match(&self.s)
    }

    /// https://html.spec.whatwg.org/multipage/#valid-simple-colour
    #[side_effect_free_attr_full(method)]
    pub fn is_valid_simple_color_string(&self) -> bool {
        let mut chars = core::primitive::str::chars(&self.s);
        if core::primitive::str::len(&self.s) == 7 && match std::str::Chars::next(&mut chars) {
            Some('#') => true,
            _ => false
        } {
            std::str::Chars::all(&mut chars, |c| core::primitive::char::is_digit(c, 16))
        } else {
            false
        }
    }
}

impl Borrow<str> for DOMString {
    #[inline]
    fn borrow(&self) -> &str {
        &self.s
    }
}

impl Default for DOMString {
    fn default() -> Self {
        DOMString{s: String::new(), p: PhantomData}
    }
}

impl Deref for DOMString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.s
    }
}

impl DerefMut for DOMString {
    #[inline]
    fn deref_mut(&mut self) -> &mut str {
        &mut self.s
    }
}

impl AsRef<str> for DOMString {
    fn as_ref(&self) -> &str {
        &self.s
    }
}

impl fmt::Display for DOMString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl PartialEq<str> for DOMString {
    fn eq(&self, other: &str) -> bool {
        &**self == other
    }
}

impl<'a> PartialEq<&'a str> for DOMString {
    fn eq(&self, other: &&'a str) -> bool {
        &**self == *other
    }
}

impl From<String> for DOMString {
    fn from(contents: String) -> DOMString {
        DOMString{s: contents, p: PhantomData}
    }
}

impl<'a> From<&'a str> for DOMString {
    fn from(contents: &str) -> DOMString {
        DOMString::from(String::from(contents))
    }
}

impl<'a> From<Cow<'a, str>> for DOMString {
    fn from(contents: Cow<'a, str>) -> DOMString {
        match contents {
            Cow::Owned(s) => DOMString::from(s),
            Cow::Borrowed(s) => DOMString::from(s),
        }
    }
}

impl From<DOMString> for LocalName {
    fn from(contents: DOMString) -> LocalName {
        LocalName::from(contents.s)
    }
}

impl From<DOMString> for Namespace {
    fn from(contents: DOMString) -> Namespace {
        Namespace::from(contents.s)
    }
}

impl From<DOMString> for Atom {
    fn from(contents: DOMString) -> Atom {
        Atom::from(contents.s)
    }
}

impl From<DOMString> for String {
    fn from(contents: DOMString) -> String {
        contents.s
    }
}

impl Into<Vec<u8>> for DOMString {
    fn into(self) -> Vec<u8> {
        self.s.into()
    }
}

impl<'a> Into<Cow<'a, str>> for DOMString {
    fn into(self) -> Cow<'a, str> {
        self.s.into()
    }
}

impl<'a> Into<CowRcStr<'a>> for DOMString {
    fn into(self) -> CowRcStr<'a> {
        self.s.into()
    }
}

impl Extend<char> for DOMString {
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = char>,
    {
        self.s.extend(iterable)
    }
}

/// https://html.spec.whatwg.org/multipage/#parse-a-month-component
#[side_effect_free_attr_full]
fn parse_month_component(value: &str) -> Result<(i32, u32), ()> {
    // Step 3
    let mut iterator = core::primitive::str::split(&value, '-');
    let year = std::option::Option::ok_or(str::Split::next(&mut iterator), ())?;
    let month = std::option::Option::ok_or(str::Split::next(&mut iterator), ())?;

    // Step 1, 2
    let year_int = std::result::Result::map_err(core::primitive::str::parse::<i32>(&year), |_| ())?;
    if core::primitive::str::len(&year) < 4 || year_int == 0 {
        return std::result::Result::Err(());
    }

    // Step 4, 5
    let month_int = std::result::Result::map_err(core::primitive::str::parse::<u32>(&month), |_| ())?;
    if core::primitive::str::len(&month) != 2 || month_int > 12 || month_int < 1 {
        return std::result::Result::Err(());
    }

    // Step 6
    std::result::Result::Ok((year_int, month_int))
}

/// https://html.spec.whatwg.org/multipage/#parse-a-date-component
#[side_effect_free_attr_full]
fn parse_date_component(value: &str) -> Result<(i32, u32, u32), ()> {
    // Step 1
    let (year_int, month_int) = parse_month_component(value)?;

    // Step 3, 4
    let day = std::option::Option::ok_or(str::Split::nth(&mut core::primitive::str::split(&value, '-'), 2), ())?;
    let day_int = std::result::Result::map_err(core::primitive::str::parse::<u32>(&day), |_| ())?;
    if core::primitive::str::len(&day) != 2 {
        return std::result::Result::Err(());
    }

    // Step 2, 5
    let max_day = max_day_in_month(year_int, month_int)?;
    if day_int == 0 || day_int > max_day {
        return std::result::Result::Err(());
    }

    // Step 6
    std::result::Result::Ok((year_int, month_int, day_int))
}

/// https://html.spec.whatwg.org/multipage/#parse-a-time-component
#[side_effect_free_attr_full]
fn parse_time_component(value: &str) -> Result<(u32, u32, f64), ()> {
    // Step 1
    let mut iterator = core::primitive::str::split(&value, ':');
    let hour = std::option::Option::ok_or(str::Split::next(&mut iterator), ())?;
    if core::primitive::str::len(&hour) != 2 {
        return std::result::Result::Err(());
    }
    let hour_int = std::result::Result::map_err(core::primitive::str::parse::<u32>(&hour), |_| ())?;

    // Step 2
    if hour_int > 23 {
        return std::result::Result::Err(());
    }

    // Step 3, 4
    let minute = std::option::Option::ok_or(str::Split::next(&mut iterator), ())?;
    if core::primitive::str::len(&minute) != 2 {
        return std::result::Result::Err(());
    }
    let minute_int = std::result::Result::map_err(core::primitive::str::parse::<u32>(&minute), |_| ())?;

    // Step 5
    if minute_int > 59 {
        return std::result::Result::Err(());
    }

    // Step 6, 7
    let second_float = match str::Split::next(&mut iterator) {
        Some(second) => {
            let mut second_iterator = core::primitive::str::split(&second, '.');
            if core::primitive::str::len(std::option::Option::ok_or(str::Split::next(&mut second_iterator), ())?) != 2 {
                return std::result::Result::Err(());
            }
            match str::Split::next(&mut second_iterator) {
                Some(second_last) => {
                    if core::primitive::str::len(&second_last) > 3 {
                        return std::result::Result::Err(());
                    }
                },
                None => {},
            }

            std::result::Result::map_err(core::primitive::str::parse::<f64>(&second), |_| ())?
        },
        None => 0.0,
    };

    // Step 8
    std::result::Result::Ok((hour_int, minute_int, second_float))
}

#[side_effect_free_attr_full]
fn max_day_in_month(year_num: i32, month_num: u32) -> Result<u32, ()> {
    match month_num {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => std::result::Result::Ok(31),
        4 | 6 | 9 | 11 => std::result::Result::Ok(30),
        2 => {
            if is_leap_year(year_num) {
                std::result::Result::Ok(29)
            } else {
                std::result::Result::Ok(28)
            }
        },
        _ => std::result::Result::Err(()),
    }
}

/// https://html.spec.whatwg.org/multipage/#week-number-of-the-last-day
#[side_effect_free_attr_full]
fn max_week_in_year(year: i32) -> u32 {
    match unchecked_operation(Utc.ymd(year as i32, 1, 1).weekday()) {
        Weekday::Thu => 53,
        Weekday::Wed if is_leap_year(year) => 53,
        _ => 52,
    }
}

#[inline]
#[side_effect_free_attr_full]
fn is_leap_year(year: i32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}
