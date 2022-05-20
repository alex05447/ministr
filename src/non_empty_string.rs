use {
    crate::*,
    std::{
        borrow::{Borrow, Cow},
        cmp::PartialEq,
        fmt::{Display, Formatter},
        ops::Deref,
    },
};

/// A non-empty [`String`].
///
/// This is the owned version, [`NonEmptyStr`] is the borrowed version.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    /// Tries to create a [`NonEmptyString`] from the string `s`.
    /// Returns `None` if the string `s` is empty.
    pub fn new(s: String) -> Option<Self> {
        if s.is_empty() {
            None
        } else {
            Some(Self(s))
        }
    }

    /// Creates a [`NonEmptyString`] from the string `s`
    /// without checking if it is empty.
    ///
    /// # Safety
    /// The caller guarantees the string `s` is not empty.
    /// Passing an empty string is undefined behaviour.
    ///
    /// # Panics
    /// In debug configuration only, panics if `s` is empty.
    pub unsafe fn new_unchecked(s: String) -> Self {
        debug_assert!(
            !s.is_empty(),
            "tried to create a non-empty string from an empty source"
        );
        Self(s)
    }

    /// Creates a [`NonEmptyString`] from the [`non-empty string slice`](NonEmptyStr) `s`.
    pub fn from(s: &NonEmptyStr) -> Self {
        unsafe { NonEmptyString::new_unchecked(s.inner().to_owned()) }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn as_ne_str(&self) -> &NonEmptyStr {
        unsafe { NonEmptyStr::new_unchecked(&self.0) }
    }

    pub fn inner(&self) -> &String {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for NonEmptyString {
    type Target = NonEmptyStr;

    fn deref(&self) -> &Self::Target {
        self.as_ne_str()
    }
}

impl AsRef<NonEmptyStr> for NonEmptyString {
    fn as_ref(&self) -> &NonEmptyStr {
        self.as_ne_str()
    }
}

impl AsRef<String> for NonEmptyString {
    fn as_ref(&self) -> &String {
        self.inner()
    }
}

impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<NonEmptyStr> for NonEmptyString {
    fn borrow(&self) -> &NonEmptyStr {
        self.as_ne_str()
    }
}

// Fallible conversions from string slices and owned strings.
////////////////////////////////////////////////////////////
impl<'s> TryFrom<&'s str> for NonEmptyString {
    type Error = ();

    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        Self::new(s.to_owned()).ok_or(())
    }
}

impl TryFrom<String> for NonEmptyString {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s).ok_or(())
    }
}

impl<'s> TryFrom<Cow<'s, str>> for NonEmptyString {
    type Error = ();

    fn try_from(s: Cow<'s, str>) -> Result<Self, Self::Error> {
        match s {
            Cow::Borrowed(s) => s.try_into(),
            Cow::Owned(s) => s.try_into(),
        }
    }
}
////////////////////////////////////////////////////////////

// Infallible conversion from a non-empty string slice.
////////////////////////////////////////////////////////////
impl<'s> From<&'s NonEmptyStr> for NonEmptyString {
    fn from(s: &'s NonEmptyStr) -> Self {
        Self::from(s)
    }
}
////////////////////////////////////////////////////////////

// Conversions into string slices and owned strings.
////////////////////////////////////////////////////////////
impl<'s> Into<&'s str> for &'s NonEmptyString {
    fn into(self) -> &'s str {
        self.as_str()
    }
}

impl Into<String> for NonEmptyString {
    fn into(self) -> String {
        self.into_inner()
    }
}

impl<'s> Into<Cow<'s, str>> for NonEmptyString {
    fn into(self) -> Cow<'s, str> {
        Cow::Owned(self.into_inner())
    }
}

impl<'s> Into<Cow<'s, str>> for &'s NonEmptyString {
    fn into(self) -> Cow<'s, str> {
        Cow::Borrowed(self.as_str())
    }
}
////////////////////////////////////////////////////////////

// Comparsions.

// <NonEmptyString>
////////////////////////////////////////////////////////////
impl PartialEq<&NonEmptyString> for NonEmptyString {
    fn eq(&self, other: &&NonEmptyString) -> bool {
        PartialEq::eq(self, *other)
    }

    fn ne(&self, other: &&NonEmptyString) -> bool {
        PartialEq::ne(self, *other)
    }
}

impl PartialEq<NonEmptyString> for &NonEmptyString {
    fn eq(&self, other: &NonEmptyString) -> bool {
        PartialEq::eq(*self, other)
    }

    fn ne(&self, other: &NonEmptyString) -> bool {
        PartialEq::ne(*self, other)
    }
}
////////////////////////////////////////////////////////////

/// <str>
////////////////////////////////////////////////////////////
impl PartialEq<str> for NonEmptyString {
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_str(), other)
    }

    fn ne(&self, other: &str) -> bool {
        PartialEq::ne(self.as_str(), other)
    }
}

impl PartialEq<&str> for NonEmptyString {
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_str(), *other)
    }

    fn ne(&self, other: &&str) -> bool {
        PartialEq::ne(self.as_str(), *other)
    }
}

impl PartialEq<str> for &NonEmptyString {
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_str(), other)
    }

    fn ne(&self, other: &str) -> bool {
        PartialEq::ne(self.as_str(), other)
    }
}
////////////////////////////////////////////////////////////

/// <String>
////////////////////////////////////////////////////////////
impl PartialEq<String> for NonEmptyString {
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }

    fn ne(&self, other: &String) -> bool {
        PartialEq::ne(self.as_str(), other.as_str())
    }
}

impl PartialEq<&String> for NonEmptyString {
    fn eq(&self, other: &&String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }

    fn ne(&self, other: &&String) -> bool {
        PartialEq::ne(self.as_str(), other.as_str())
    }
}

impl PartialEq<String> for &NonEmptyString {
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }

    fn ne(&self, other: &String) -> bool {
        PartialEq::ne(self.as_str(), other.as_str())
    }
}
////////////////////////////////////////////////////////////

/// <NonEmptyStr>
////////////////////////////////////////////////////////////
impl PartialEq<NonEmptyStr> for NonEmptyString {
    fn eq(&self, other: &NonEmptyStr) -> bool {
        PartialEq::eq(self.as_ne_str(), other)
    }

    fn ne(&self, other: &NonEmptyStr) -> bool {
        PartialEq::ne(self.as_ne_str(), other)
    }
}

impl PartialEq<&NonEmptyStr> for NonEmptyString {
    fn eq(&self, other: &&NonEmptyStr) -> bool {
        PartialEq::eq(self.as_ne_str(), *other)
    }

    fn ne(&self, other: &&NonEmptyStr) -> bool {
        PartialEq::ne(self.as_ne_str(), *other)
    }
}

impl PartialEq<NonEmptyStr> for &NonEmptyString {
    fn eq(&self, other: &NonEmptyStr) -> bool {
        PartialEq::eq(self.as_ne_str(), other)
    }

    fn ne(&self, other: &NonEmptyStr) -> bool {
        PartialEq::ne(self.as_ne_str(), other)
    }
}
////////////////////////////////////////////////////////////

impl Display for NonEmptyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmp(nes: &NonEmptyString, s: &str) {
        assert_eq!(nes, s);
        //assert_eq!(s, nes);
        assert_eq!(&nes[..], s);
        assert_eq!(nes.as_str(), s);
        assert_eq!(nes.as_ne_str(), s);
        assert_eq!(nes.deref(), s);
        assert_eq!(nes.inner(), s);
    }

    #[test]
    fn non_empty_string() {
        let foo = "foo";
        let foo_str = foo.to_owned();
        let ne_foo = NonEmptyStr::new(foo).unwrap();

        // `new()` from non-empty string
        let ne_foo_str = NonEmptyString::new(foo_str.clone()).unwrap();
        cmp(&ne_foo_str, foo);

        assert_eq!(ne_foo_str, ne_foo_str);
        assert_eq!(ne_foo_str, &ne_foo_str);
        assert_eq!(&ne_foo_str, ne_foo_str);
        assert_eq!(&ne_foo_str, &ne_foo_str);
        assert_eq!(ne_foo_str, *ne_foo_str);
        assert_eq!(*ne_foo_str, ne_foo_str);
        assert_eq!(*ne_foo_str, *ne_foo_str);

        // `new()` from empty string
        let empty = "";
        assert!(NonEmptyString::new(empty.to_owned()).is_none());

        // `unchecked_new()` from non-empty string
        {
            let ne_foo_str = unsafe { NonEmptyString::new_unchecked(foo_str.clone()) };
            cmp(&ne_foo_str, foo);
        }

        // from `NonEmptyStr`.
        {
            let ne_foo_str: NonEmptyString = ne_foo.into();
            cmp(&ne_foo_str, foo);

            let ne_foo_str = <NonEmptyString as From<&NonEmptyStr>>::from(&ne_foo);
            cmp(&ne_foo_str, foo);
        }

        // try from non-empty slice
        {
            use std::convert::TryInto;

            let ne_foo_str: NonEmptyString = foo.try_into().unwrap();
            cmp(&ne_foo_str, foo);

            let ne_foo_str = <NonEmptyString as TryFrom<&str>>::try_from(foo).unwrap();
            cmp(&ne_foo_str, foo);
        }

        // try from empty slice
        {
            use std::convert::TryInto;

            let ne_foo_str: Result<NonEmptyString, _> = "".try_into();
            assert!(ne_foo_str.is_err());

            let ne_foo_str = <NonEmptyString as TryFrom<&str>>::try_from("");
            assert!(ne_foo_str.is_err());
        }

        // try from non-empty `String`
        {
            use std::convert::TryInto;

            let ne_foo_str: NonEmptyString = foo_str.clone().try_into().unwrap();
            cmp(&ne_foo_str, foo);

            let ne_foo_str =
                <NonEmptyString as TryFrom<String>>::try_from(foo_str.clone()).unwrap();
            cmp(&ne_foo_str, foo);
        }

        // try from empty `String`
        {
            use std::convert::TryInto;

            let empty_str = "".to_owned();

            let ne_foo_str: Result<NonEmptyString, _> = empty_str.clone().try_into();
            assert!(ne_foo_str.is_err());

            let ne_foo_str = <NonEmptyString as TryFrom<String>>::try_from(empty_str);
            assert!(ne_foo_str.is_err());
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "tried to create a non-empty string from an empty source")]
    fn new_unchecked_panic() {
        let _ = unsafe { NonEmptyString::new_unchecked("".to_owned()) };
    }
}
