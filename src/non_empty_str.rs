use {
    crate::*,
    std::{
        borrow::{ToOwned, Cow},
        cmp::PartialEq,
        fmt::{Display, Formatter},
        ops::Deref,
    },
};

/// A non-empty UTF-8 string slice.
///
/// This is the borrowed version, [`NonEmptyString`] is the owned version.
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NonEmptyStr(str);

impl NonEmptyStr {
    /// Tries to create a [`NonEmptyStr`] from the string slice `s`.
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
    /// The caller guarantees the string `s` is not empty.
    /// Passing an empty string slice is undefined behaviour.
    ///
    /// # Panics
    /// In debug configuration only, panics if `s` is empty.
    pub unsafe fn new_unchecked(s: &str) -> &Self {
        debug_assert!(
            !s.is_empty(),
            "tried to create a non-empty string slice from an empty source"
        );
        &*(s as *const str as *const _)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl Deref for NonEmptyStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl AsRef<str> for &NonEmptyStr {
    fn as_ref(&self) -> &str {
        self.inner()
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

// Conversions into string slices and owned strings.
// Conversion into a non-empty owned string is implemented by a `From` on it.
////////////////////////////////////////////////////////////
impl<'s> Into<&'s str> for &'s NonEmptyStr {
    fn into(self) -> &'s str {
        self.as_str()
    }
}

impl<'s> Into<String> for &'s NonEmptyStr {
    fn into(self) -> String {
        self.0.to_owned()
    }
}

impl<'s> Into<Cow<'s, str>> for &'s NonEmptyStr {
    fn into(self) -> Cow<'s, str> {
        Cow::Borrowed(self.as_str())
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

    fn ne(&self, other: &&NonEmptyStr) -> bool {
        PartialEq::ne(self, *other)
    }
}

impl PartialEq<NonEmptyStr> for &NonEmptyStr {
    fn eq(&self, other: &NonEmptyStr) -> bool {
        PartialEq::eq(*self, other)
    }

    fn ne(&self, other: &NonEmptyStr) -> bool {
        PartialEq::ne(*self, other)
    }
}
////////////////////////////////////////////////////////////

/// <str>
////////////////////////////////////////////////////////////
impl PartialEq<str> for NonEmptyStr {
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_str(), other)
    }

    fn ne(&self, other: &str) -> bool {
        PartialEq::ne(self.as_str(), other)
    }
}

impl PartialEq<&str> for NonEmptyStr {
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_str(), *other)
    }

    fn ne(&self, other: &&str) -> bool {
        PartialEq::ne(self.as_str(), *other)
    }
}

impl PartialEq<str> for &NonEmptyStr {
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
impl PartialEq<String> for NonEmptyStr {
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }

    fn ne(&self, other: &String) -> bool {
        PartialEq::ne(self.as_str(), other.as_str())
    }
}

impl PartialEq<&String> for NonEmptyStr {
    fn eq(&self, other: &&String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }

    fn ne(&self, other: &&String) -> bool {
        PartialEq::ne(self.as_str(), other.as_str())
    }
}

impl PartialEq<String> for &NonEmptyStr {
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }

    fn ne(&self, other: &String) -> bool {
        PartialEq::ne(self.as_str(), other.as_str())
    }
}
////////////////////////////////////////////////////////////

/// <NonEmptyString>
////////////////////////////////////////////////////////////
impl PartialEq<NonEmptyString> for NonEmptyStr {
    fn eq(&self, other: &NonEmptyString) -> bool {
        PartialEq::eq(self, other.as_ne_str())
    }

    fn ne(&self, other: &NonEmptyString) -> bool {
        PartialEq::ne(self, other.as_ne_str())
    }
}

impl PartialEq<&NonEmptyString> for NonEmptyStr {
    fn eq(&self, other: &&NonEmptyString) -> bool {
        PartialEq::eq(self, other.as_ne_str())
    }

    fn ne(&self, other: &&NonEmptyString) -> bool {
        PartialEq::ne(self, other.as_ne_str())
    }
}

impl PartialEq<NonEmptyString> for &NonEmptyStr {
    fn eq(&self, other: &NonEmptyString) -> bool {
        PartialEq::eq(self, other.as_ne_str())
    }

    fn ne(&self, other: &NonEmptyString) -> bool {
        PartialEq::ne(self, other.as_ne_str())
    }
}
////////////////////////////////////////////////////////////

impl<'s> Display for &'s NonEmptyStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmp(nes: &NonEmptyStr, s: &str) {
        assert_eq!(nes, s);
        //assert_eq!(s, nes);
        assert_eq!(&nes[..], s);
        assert_eq!(nes.as_ref(), s);
        assert_eq!(nes.as_str(), s);
        assert_eq!(nes.deref(), s);
        assert_eq!(nes.inner(), s);
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
    #[should_panic(expected = "tried to create a non-empty string slice from an empty source")]
    fn new_unchecked_panic() {
        let _ = unsafe { NonEmptyStr::new_unchecked("") };
    }
}
