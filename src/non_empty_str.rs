use {
    crate::*,
    miniunchecked::*,
    std::{
        borrow::{Cow, ToOwned},
        cmp::PartialEq,
        fmt::{Display, Formatter},
        num::NonZeroUsize,
        ops::Deref,
    },
};

/// A non-empty [`string slice`](str).
///
/// This is the borrowed version, [`NonEmptyString`] is the owned version.
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NonEmptyStr(str);

impl NonEmptyStr {
    /// Tries to create a [`NonEmptyStr`] from the string slice `s`.
    ///
    /// Returns `None` if the string `s` is empty.
    pub fn new(s: &str) -> Option<&Self> {
        if s.is_empty() {
            None
        } else {
            Some(unsafe { Self::new_unchecked(s) })
        }
    }

    /// Creates a [`NonEmptyStr`] from the string slice `s`
    /// without checking if it is empty.
    ///
    /// # Safety
    ///
    /// The caller guarantees the string `s` is not empty.
    /// Passing an empty string slice is undefined behaviour.
    ///
    /// # Panics
    ///
    /// In debug configuration only, panics if `s` is empty.
    pub unsafe fn new_unchecked(s: &str) -> &Self {
        debug_assert!(
            !s.is_empty(),
            "tried to create a non-empty string slice from an empty string"
        );
        unsafe { &*(s as *const str as *const _) }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn len_nonzero(&self) -> NonZeroUsize {
        unsafe {
            NonZeroUsize::new(self.0.len())
                .unwrap_unchecked_dbg_msg("non-empty strings have non-zero length")
        }
    }
}

impl Deref for NonEmptyStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for &NonEmptyStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<NonEmptyStr> for &NonEmptyStr {
    fn as_ref(&self) -> &NonEmptyStr {
        self
    }
}

impl ToOwned for NonEmptyStr {
    type Owned = NonEmptyString;

    fn to_owned(&self) -> Self::Owned {
        self.into()
    }
}

// Fallible conversions from string slices and owned strings.
////////////////////////////////////////////////////////////
impl<'s> TryFrom<&'s str> for &'s NonEmptyStr {
    type Error = ();

    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        NonEmptyStr::new(s).ok_or(())
    }
}

impl<'s> TryFrom<&'s String> for &'s NonEmptyStr {
    type Error = ();

    fn try_from(s: &'s String) -> Result<Self, Self::Error> {
        NonEmptyStr::new(s).ok_or(())
    }
}
////////////////////////////////////////////////////////////

// Infallible conversion from a non-empty owned string.
////////////////////////////////////////////////////////////
impl<'s> From<&'s NonEmptyString> for &'s NonEmptyStr {
    fn from(s: &'s NonEmptyString) -> Self {
        s.as_ne_str()
    }
}
////////////////////////////////////////////////////////////

// Infallible conversions into string slices and owned strings.
// Conversion into a non-empty owned string is implemented by a `From` on it.
////////////////////////////////////////////////////////////
impl<'s> From<&'s NonEmptyStr> for &'s str {
    fn from(val: &'s NonEmptyStr) -> Self {
        val.as_str()
    }
}

impl<'s> From<&'s NonEmptyStr> for String {
    fn from(val: &'s NonEmptyStr) -> Self {
        val.as_str().to_owned()
    }
}

impl<'s> From<&'s NonEmptyStr> for Cow<'s, str> {
    fn from(val: &'s NonEmptyStr) -> Self {
        Cow::Borrowed(val.as_str())
    }
}

impl<'s> From<&'s NonEmptyStr> for Cow<'s, NonEmptyStr> {
    fn from(val: &'s NonEmptyStr) -> Self {
        Cow::Borrowed(val)
    }
}
////////////////////////////////////////////////////////////

// Comparsions.

// <NonEmptyStr>
////////////////////////////////////////////////////////////
impl PartialEq<&NonEmptyStr> for NonEmptyStr {
    fn eq(&self, other: &&NonEmptyStr) -> bool {
        PartialEq::eq(self, *other)
    }
}

impl PartialEq<NonEmptyStr> for &NonEmptyStr {
    fn eq(&self, other: &NonEmptyStr) -> bool {
        PartialEq::eq(*self, other)
    }
}
////////////////////////////////////////////////////////////

// <str>
////////////////////////////////////////////////////////////

// Direct

impl PartialEq<str> for NonEmptyStr {
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_str(), other)
    }
}

impl PartialEq<&str> for NonEmptyStr {
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_str(), *other)
    }
}

impl PartialEq<str> for &NonEmptyStr {
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_str(), other)
    }
}

// Reverse

impl PartialEq<NonEmptyStr> for str {
    fn eq(&self, other: &NonEmptyStr) -> bool {
        PartialEq::eq(self, other.as_str())
    }
}

impl PartialEq<&NonEmptyStr> for str {
    fn eq(&self, other: &&NonEmptyStr) -> bool {
        PartialEq::eq(self, other.as_str())
    }
}

impl PartialEq<NonEmptyStr> for &str {
    fn eq(&self, other: &NonEmptyStr) -> bool {
        PartialEq::eq(*self, other.as_str())
    }
}
////////////////////////////////////////////////////////////

// <String>
////////////////////////////////////////////////////////////

// Direct

impl PartialEq<String> for NonEmptyStr {
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl PartialEq<&String> for NonEmptyStr {
    fn eq(&self, other: &&String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl PartialEq<String> for &NonEmptyStr {
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

// Reverse

impl PartialEq<NonEmptyStr> for String {
    fn eq(&self, other: &NonEmptyStr) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl PartialEq<&NonEmptyStr> for String {
    fn eq(&self, other: &&NonEmptyStr) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl PartialEq<NonEmptyStr> for &String {
    fn eq(&self, other: &NonEmptyStr) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}
////////////////////////////////////////////////////////////

// <NonEmptyString>
////////////////////////////////////////////////////////////
impl PartialEq<NonEmptyString> for NonEmptyStr {
    fn eq(&self, other: &NonEmptyString) -> bool {
        PartialEq::eq(self, other.as_ne_str())
    }
}

impl PartialEq<&NonEmptyString> for NonEmptyStr {
    fn eq(&self, other: &&NonEmptyString) -> bool {
        PartialEq::eq(self, other.as_ne_str())
    }
}

impl PartialEq<NonEmptyString> for &NonEmptyStr {
    fn eq(&self, other: &NonEmptyString) -> bool {
        PartialEq::eq(self, other.as_ne_str())
    }
}
////////////////////////////////////////////////////////////

impl Display for &NonEmptyStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::redundant_slicing)]
    fn cmp(nes: &NonEmptyStr, s: &str) {
        assert_eq!(nes, s);
        assert_eq!(s, nes);
        assert_eq!(&nes[..], s);
        assert_eq!(&s[..], nes);
        assert_eq!(<&NonEmptyStr as AsRef<str>>::as_ref(&nes), s);
        assert_eq!(s, <&NonEmptyStr as AsRef<str>>::as_ref(&nes));
        assert_eq!(nes.as_str(), s);
        assert_eq!(s, nes.as_str());
        assert_eq!(nes.deref(), s);
        assert_eq!(s, nes.deref());
    }

    #[test]
    fn non_empty_str() {
        let foo = "foo";
        let foo_str = foo.to_owned();
        let ne_foo_str = NonEmptyString::new(foo_str.clone()).unwrap();

        // `new()` from non-empty slice
        let ne_foo = NonEmptyStr::new(foo).unwrap();
        cmp(ne_foo, foo);

        assert_eq!(ne_foo, ne_foo);
        assert_eq!(ne_foo, &ne_foo);
        assert_eq!(&ne_foo, ne_foo);
        assert_eq!(&ne_foo, &ne_foo);
        assert_eq!(ne_foo, *ne_foo);
        assert_eq!(*ne_foo, ne_foo);
        assert_eq!(*ne_foo, *ne_foo);

        // `new()` from empty slice
        let empty = "";
        assert!(NonEmptyStr::new(empty).is_none());

        // `unchecked_new()` from non-empty slice
        {
            let ne_foo = unsafe { NonEmptyStr::new_unchecked(foo) };
            cmp(ne_foo, foo);
        }

        // from `NonEmptyString`.
        {
            let ne_foo: &NonEmptyStr = (&ne_foo_str).into();
            cmp(ne_foo, foo);

            let ne_foo = <&NonEmptyStr as From<&NonEmptyString>>::from(&ne_foo_str);
            cmp(ne_foo, foo);
        }

        // try from non-empty slice
        {
            use std::convert::TryInto;

            let ne_foo: &NonEmptyStr = foo.try_into().unwrap();
            cmp(ne_foo, foo);

            let ne_foo = <&NonEmptyStr as TryFrom<&str>>::try_from(foo).unwrap();
            cmp(ne_foo, foo);
        }

        // try from empty slice
        {
            use std::convert::TryInto;

            let ne_foo: Result<&NonEmptyStr, _> = "".try_into();
            assert!(ne_foo.is_err());

            let ne_foo = <&NonEmptyStr as TryFrom<&str>>::try_from("");
            assert!(ne_foo.is_err());
        }

        // try from non-empty `String`
        {
            use std::convert::TryInto;

            let ne_foo: &NonEmptyStr = (&foo_str).try_into().unwrap();
            cmp(ne_foo, foo);

            let ne_foo = <&NonEmptyStr as TryFrom<&String>>::try_from(&foo_str).unwrap();
            cmp(ne_foo, foo);
        }

        // try from empty `String`
        {
            use std::convert::TryInto;

            let empty_str = "".to_owned();

            let ne_foo: Result<&NonEmptyStr, _> = (&empty_str).try_into();
            assert!(ne_foo.is_err());

            let ne_foo = <&NonEmptyStr as TryFrom<&String>>::try_from(&empty_str);
            assert!(ne_foo.is_err());
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "tried to create a non-empty string slice from an empty string")]
    fn new_unchecked_panic() {
        let _ = unsafe { NonEmptyStr::new_unchecked("") };
    }
}
