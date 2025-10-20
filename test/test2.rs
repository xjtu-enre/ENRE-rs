// 实体总数：57
// 关系总数：94

// [实体] Module: std::borrow (通过 use 导入)
// [关系] Use: 使用 std::borrow 模块中的 Cow
use std::borrow::Cow;

// [实体] Module: crate::form::name (通过 use 导入)
// [关系] Use: 使用 crate::form::name 模块中的所有项
use crate::form::name::*;
use crate::http::ext::IntoOwned;
use Eq;
use From;
use PartialEq;
use indexmap;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

/// A potentially owned [`Name`].
///
/// Constructible from a [`NameView`], [`Name`], `&str`, or `String`, a
/// `NameBuf` acts much like a [`Name`] but can be converted into an owned
/// version via [`IntoOwned`](crate::http::ext::IntoOwned).
///
/// ```rust
/// use rocket::form::name::NameBuf;
/// use rocket::http::ext::IntoOwned;
///
/// let alloc = String::from("a.b.c");
/// let name = NameBuf::from(alloc.as_str());
/// let owned: NameBuf<'static> = name.into_owned();
/// ```
// [实体] Struct: NameBuf
// [关系] Define: 定义结构体 NameBuf
// [关系] Inherit: 通过 #[derive(Clone)] 继承 Clone trait 的行为
// [关系] Contain: NameBuf 包含 left 和 right 字段
#[derive(Clone)]
pub struct NameBuf<'v> {
    // [实体] Variable: left (字段)
    left: &'v Name,
    // [实体] Variable: right (字段)
    right: Cow<'v, str>,
}

// [关系] Impl: 为 NameBuf 实现方法
impl<'v> NameBuf<'v> {
    // [实体] Function/Method: split
    // [关系] Define: 定义方法 split
    #[inline]
    fn split(&self) -> (&Name, &Name) {
        // [关系] Call: 调用 Name::new
        // [关系] UseVar: 使用变量 self.right, self.left
        (self.left, Name::new(&self.right))
    }

    /// Returns an iterator over the keys of `self`, including empty keys.
    ///
    /// See [`Name`] for a description of "keys".
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::form::name::NameBuf;
    ///
    /// let name = NameBuf::from("apple.b[foo:bar]zoo.[barb].bat");
    /// let keys: Vec<_> = name.keys().map(|k| k.as_str()).collect();
    /// assert_eq!(keys, &["apple", "b", "foo:bar", "zoo", "", "barb", "bat"]);
    /// ```
    // [实体] Function/Method: keys
    // [关系] Define: 定义方法 keys
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        // [实体] Variable: left (局部变量)
        // [实体] Variable: right (局部变量)
        // [关系] Call: 调用 self.split()
        // [关系] UseVar: 使用变量 self
        let (left, right) = self.split();
        // [关系] Call: 调用 left.keys(), right.keys(), chain
        // [关系] UseVar: 使用变量 left, right
        left.keys().chain(right.keys())
    }

    /// Returns `true` if `self` is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::form::name::NameBuf;
    ///
    /// let name = NameBuf::from("apple.b[foo:bar]zoo.[barb].bat");
    /// assert!(!name.is_empty());
    ///
    /// let name = NameBuf::from("");
    /// assert!(name.is_empty());
    /// ```
    // [实体] Function/Method: is_empty
    // [关系] Define: 定义方法 is_empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        // [实体] Variable: left (局部变量)
        // [实体] Variable: right (局部变量)
        // [关系] Call: 调用 self.split()
        // [关系] UseVar: 使用变量 self
        let (left, right) = self.split();
        // [关系] Call: 调用 left.is_empty(), right.is_empty()
        // [关系] UseVar: 使用变量 left, right
        left.is_empty() && right.is_empty()
    }
}

// [实体] Trait: IntoOwned (通过 impl 知道其存在)
// [关系] Impl: 实现 IntoOwned trait for NameBuf
impl crate::http::ext::IntoOwned for NameBuf<'_> {
    // [实体] Associated items: Owned
    type Owned = NameBuf<'static>;

    // [实体] Function/Method: into_owned
    // [关系] Define: 定义方法 into_owned
    fn into_owned(self) -> Self::Owned {
        // [实体] Variable: right (局部变量)
        // [实体] Variable: l (match arm 变量)
        // [实体] Variable: r (match arm 变量)
        // [关系] UseVar: 使用变量 self.left, self.right
        let right = match (self.left, self.right) {
            // [关系] UseVar: 使用变量 l, r
            // [关系] Call: 调用 l.is_empty()
            (l, Cow::Owned(r)) if l.is_empty() => Cow::Owned(r),
            // [关系] UseVar: 使用变量 l, r
            // [关系] Call: 调用 l.is_empty(), r.to_string(), into()
            (l, r) if l.is_empty() => r.to_string().into(),
            // [关系] UseVar: 使用变量 l, r
            // [关系] Call: 调用 r.is_empty()
            (l, r) if r.is_empty() => l.to_string().into(),
            // [关系] UseVar: 使用变量 l, r
            // [关系] Call: 调用 format!, into()
            (l, r) => format!("{}.{}", l, r).into(),
        };

        NameBuf {
            // [关系] Call: 调用 "".into()
            left: "".into(),
            right,
        }
    }
}

// [实体] Trait: Serialize (通过 impl 知道其存在)
// [关系] Impl: 实现 serde::Serialize trait for NameBuf
impl serde::Serialize for NameBuf<'_> {
    // [实体] Function/Method: serialize
    // [关系] Define: 定义方法 serialize
    // [实体] Variable: serializer (参数)
    // [实体] Variable: S (泛型参数)
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // [关系] Call: 调用 serializer.serialize_str, self.to_string
        // [关系] UseVar: 使用变量 serializer, self
        serializer.serialize_str(&self.to_string())
    }
}

// [实体] Trait: From (通过 impl 知道其存在)
// [关系] Impl: 实现 From<NameView<'v>> for NameBuf<'v>
impl<'v> From<NameView<'v>> for NameBuf<'v> {
    // [实体] Function/Method: from
    // [关系] Define: 定义方法 from
    // [实体] Variable: nv (参数)
    fn from(nv: NameView<'v>) -> Self {
        NameBuf {
            // [关系] Call: 调用 nv.as_name()
            // [关系] UseVar: 使用变量 nv
            left: nv.as_name(),
            // [关系] Call: 调用 Cow::Borrowed
            right: Cow::Borrowed(""),
        }
    }
}

// [实体] Trait: From (通过 impl 知道其存在)
// [关系] Impl: 实现 From<&'v Name> for NameBuf<'v>
impl<'v> From<&'v Name> for NameBuf<'v> {
    // [实体] Function/Method: from
    // [关系] Define: 定义方法 from
    // [实体] Variable: name (参数)
    fn from(name: &'v Name) -> Self {
        NameBuf {
            // [关系] UseVar: 使用变量 name
            left: name,
            // [关系] Call: 调用 Cow::Borrowed
            right: Cow::Borrowed(""),
        }
    }
}

// [实体] Trait: From (通过 impl 知道其存在)
// [关系] Impl: 实现 From<&'v str> for NameBuf<'v>
impl<'v> From<&'v str> for NameBuf<'v> {
    // [实体] Function/Method: from
    // [关系] Define: 定义方法 from
    // [实体] Variable: name (参数)
    fn from(name: &'v str) -> Self {
        // [关系] Call: 调用 NameBuf::from
        // [关系] Call: 调用 (None, Cow::Borrowed(name))
        // [关系] UseVar: 使用变量 name
        NameBuf::from((None, Cow::Borrowed(name)))
    }
}

// [实体] Trait: From (通过 impl 知道其存在)
// [关系] Impl: 实现 From<String> for NameBuf<'v>
impl<'v> From<String> for NameBuf<'v> {
    // [实体] Function/Method: from
    // [关系] Define: 定义方法 from
    // [实体] Variable: name (参数)
    fn from(name: String) -> Self {
        // [关系] Call: 调用 NameBuf::from
        // [关系] Call: 调用 (None, Cow::Owned(name))
        // [关系] UseVar: 使用变量 name
        NameBuf::from((None, Cow::Owned(name)))
    }
}

#[doc(hidden)]
// [实体] Trait: From (通过 impl 知道其存在)
// [关系] Impl: 实现 From<(Option<&'v Name>, Cow<'v, str>)> for NameBuf<'v>
impl<'v> From<(Option<&'v Name>, Cow<'v, str>)> for NameBuf<'v> {
    // [实体] Function/Method: from
    // [关系] Define: 定义方法 from
    // [实体] Variable: prefix (元组参数)
    // [实体] Variable: right (元组参数)
    fn from((prefix, right): (Option<&'v Name>, Cow<'v, str>)) -> Self {
        // [关系] UseVar: 使用变量 prefix
        match prefix {
            // [实体] Variable: left (match arm 变量)
            // [关系] UseVar: 使用变量 left, right
            Some(left) => NameBuf { left, right },
            None => NameBuf {
                // [关系] Call: 调用 "".into()
                left: "".into(),
                right,
            },
        }
    }
}

#[doc(hidden)]
// [实体] Trait: From (通过 impl 知道其存在)
// [关系] Impl: 实现 From<(Option<&'v Name>, String)> for NameBuf<'v>
impl<'v> From<(Option<&'v Name>, String)> for NameBuf<'v> {
    // [实体] Function/Method: from
    // [关系] Define: 定义方法 from
    // [实体] Variable: prefix (元组参数)
    // [实体] Variable: right (元组参数)
    fn from((prefix, right): (Option<&'v Name>, String)) -> Self {
        // [关系] UseVar: 使用变量 prefix
        match prefix {
            // [实体] Variable: left (match arm 变量)
            // [关系] UseVar: 使用变量 left, right
            // [关系] Call: 调用 right.into()
            Some(left) => NameBuf {
                left,
                right: right.into(),
            },
            None => NameBuf {
                // [关系] Call: 调用 "".into()
                left: "".into(),
                // [关系] Call: 调用 right.into()
                right: right.into(),
            },
        }
    }
}

#[doc(hidden)]
// [实体] Trait: From (通过 impl 知道其存在)
// [关系] Impl: 实现 From<(Option<&'v Name>, &'v str)> for NameBuf<'v>
impl<'v> From<(Option<&'v Name>, &'v str)> for NameBuf<'v> {
    // [实体] Function/Method: from
    // [关系] Define: 定义方法 from
    // [实体] Variable: prefix (元组参数)
    // [实体] Variable: suffix (元组参数)
    fn from((prefix, suffix): (Option<&'v Name>, &'v str)) -> Self {
        // [关系] Call: 调用 NameBuf::from
        // [关系] Call: 调用 (prefix, Cow::Borrowed(suffix))
        // [关系] UseVar: 使用变量 prefix, suffix
        NameBuf::from((prefix, Cow::Borrowed(suffix)))
    }
}

#[doc(hidden)]
// [实体] Trait: From (通过 impl 知道其存在)
// [关系] Impl: 实现 From<(&'v Name, &'v str)> for NameBuf<'v>
impl<'v> From<(&'v Name, &'v str)> for NameBuf<'v> {
    // [实体] Function/Method: from
    // [关系] Define: 定义方法 from
    // [实体] Variable: prefix (元组参数)
    // [实体] Variable: suffix (元组参数)
    fn from((prefix, suffix): (&'v Name, &'v str)) -> Self {
        // [关系] Call: 调用 NameBuf::from
        // [关系] Call: 调用 (Some(prefix), Cow::Borrowed(suffix))
        // [关系] UseVar: 使用变量 prefix, suffix
        NameBuf::from((Some(prefix), Cow::Borrowed(suffix)))
    }
}

// [实体] Trait: Debug (通过 impl 知道其存在)
// [关系] Impl: 实现 std::fmt::Debug trait for NameBuf
impl std::fmt::Debug for NameBuf<'_> {
    // [实体] Function/Method: fmt
    // [关系] Define: 定义方法 fmt
    // [实体] Variable: f (参数)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // [关系] Call: 调用 write!
        // [关系] UseVar: 使用变量 f
        write!(f, "\"")?;

        // [实体] Variable: left (局部变量)
        // [实体] Variable: right (局部变量)
        // [关系] Call: 调用 self.split()
        // [关系] UseVar: 使用变量 self
        let (left, right) = self.split();
        // [关系] Call: 调用 left.is_empty()
        // [关系] UseVar: 使用变量 left
        if !left.is_empty() {
            // [关系] Call: 调用 write!, left.escape_debug()
            // [关系] UseVar: 使用变量 f, left
            write!(f, "{}", left.escape_debug())?
        }
        // [关系] Call: 调用 right.is_empty()
        // [关系] UseVar: 使用变量 right
        if !right.is_empty() {
            // [关系] Call: 调用 left.is_empty()
            // [关系] UseVar: 使用变量 left
            if !left.is_empty() {
                // [关系] Call: 调用 f.write_str
                // [关系] UseVar: 使用变量 f
                f.write_str(".")?;
            }
            // [关系] Call: 调用 write!, right.escape_debug()
            // [关系] UseVar: 使用变量 f, right
            write!(f, "{}", right.escape_debug())?;
        }

        // [关系] Call: 调用 write!
        // [关系] UseVar: 使用变量 f
        write!(f, "\"")
    }
}

// [实体] Trait: Display (通过 impl 知道其存在)
// [关系] Impl: 实现 std::fmt::Display trait for NameBuf
impl std::fmt::Display for NameBuf<'_> {
    // [实体] Function/Method: fmt
    // [关系] Define: 定义方法 fmt
    // [实体] Variable: f (参数)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // [实体] Variable: left (局部变量)
        // [实体] Variable: right (局部变量)
        // [关系] Call: 调用 self.split()
        // [关系] UseVar: 使用变量 self
        let (left, right) = self.split();
        // [关系] Call: 调用 left.is_empty()
        // [关系] UseVar: 使用变量 left
        if !left.is_empty() {
            // [关系] Call: 调用 left.fmt
            // [关系] UseVar: 使用变量 left, f
            left.fmt(f)?;
        }
        // [关系] Call: 调用 right.is_empty()
        // [关系] UseVar: 使用变量 right
        if !right.is_empty() {
            // [关系] Call: 调用 left.is_empty()
            // [关系] UseVar: 使用变量 left
            if !left.is_empty() {
                // [关系] Call: 调用 f.write_str
                // [关系] UseVar: 使用变量 f
                f.write_str(".")?;
            }
            // [关系] Call: 调用 right.fmt
            // [关系] UseVar: 使用变量 right, f
            right.fmt(f)?;
        }

        Ok(())
    }
}

// [实体] Trait: PartialEq (通过 impl 知道其存在)
// [关系] Impl: 实现 PartialEq trait for NameBuf
impl PartialEq for NameBuf<'_> {
    // [实体] Function/Method: eq
    // [关系] Define: 定义方法 eq
    // [实体] Variable: other (参数)
    fn eq(&self, other: &Self) -> bool {
        // [关系] Call: 调用 self.keys().eq, other.keys()
        // [关系] UseVar: 使用变量 self, other
        self.keys().eq(other.keys())
    }
}

// [实体] Trait: PartialEq (通过 impl 知道其存在)
// [关系] Impl: 实现 PartialEq<N> for NameBuf
impl<N: AsRef<Name> + ?Sized> PartialEq<N> for NameBuf<'_> {
    // [实体] Function/Method: eq
    // [关系] Define: 定义方法 eq
    // [实体] Variable: other (参数)
    // [实体] Variable: N (泛型参数)
    fn eq(&self, other: &N) -> bool {
        // [关系] Call: 调用 self.keys().eq, other.as_ref().keys()
        // [关系] UseVar: 使用变量 self, other
        self.keys().eq(other.as_ref().keys())
    }
}

// [实体] Trait: PartialEq (通过 impl 知道其存在)
// [关系] Impl: 实现 PartialEq<NameBuf<'_>> for Name
impl PartialEq<NameBuf<'_>> for Name {
    // [实体] Function/Method: eq
    // [关系] Define: 定义方法 eq
    // [实体] Variable: other (参数)
    fn eq(&self, other: &NameBuf<'_>) -> bool {
        // [关系] Call: 调用 self.keys().eq, other.keys()
        // [关系] UseVar: 使用变量 self, other
        self.keys().eq(other.keys())
    }
}

// [实体] Trait: PartialEq (通过 impl 知道其存在)
// [关系] Impl: 实现 PartialEq<NameBuf<'_>> for str
impl PartialEq<NameBuf<'_>> for str {
    // [实体] Function/Method: eq
    // [关系] Define: 定义方法 eq
    // [实体] Variable: other (参数)
    fn eq(&self, other: &NameBuf<'_>) -> bool {
        // [关系] Call: 调用 Name::new, PartialEq::eq
        // [关系] UseVar: 使用变量 self, other
        Name::new(self) == other
    }
}

// [实体] Trait: PartialEq (通过 impl 知道其存在)
// [关系] Impl: 实现 PartialEq<NameBuf<'_>> for &str
impl PartialEq<NameBuf<'_>> for &str {
    // [实体] Function/Method: eq
    // [关系] Define: 定义方法 eq
    // [实体] Variable: other (参数)
    fn eq(&self, other: &NameBuf<'_>) -> bool {
        // [关系] Call: 调用 Name::new, PartialEq::eq
        // [关系] UseVar: 使用变量 self, other
        Name::new(self) == other
    }
}

// [实体] Trait: Eq (通过 impl 知道其存在)
// [关系] Impl: 实现 Eq trait for NameBuf
impl Eq for NameBuf<'_> {}

// [实体] Trait: Hash (通过 impl 知道其存在)
// [关系] Impl: 实现 std::hash::Hash trait for NameBuf
impl std::hash::Hash for NameBuf<'_> {
    // [实体] Function/Method: hash
    // [关系] Define: 定义方法 hash
    // [实体] Variable: state (参数)
    // [实体] Variable: H (泛型参数)
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // [关系] Call: 调用 self.keys().for_each, k.hash
        // [关系] UseVar: 使用变量 self, state
        self.keys().for_each(|k| k.hash(state))
    }
}

// [实体] Trait: Equivalent (通过 impl 知道其存在)
// [关系] Impl: 实现 indexmap::Equivalent<Name> trait for NameBuf
impl indexmap::Equivalent<Name> for NameBuf<'_> {
    // [实体] Function/Method: equivalent
    // [关系] Define: 定义方法 equivalent
    // [实体] Variable: key (参数)
    fn equivalent(&self, key: &Name) -> bool {
        // [关系] Call: 调用 self.keys().eq, key.keys()
        // [关系] UseVar: 使用变量 self, key
        self.keys().eq(key.keys())
    }
}

// [实体] Trait: Equivalent (通过 impl 知道其存在)
// [关系] Impl: 实现 indexmap::Equivalent<NameBuf<'_>> trait for Name
impl indexmap::Equivalent<NameBuf<'_>> for Name {
    // [实体] Function/Method: equivalent
    // [关系] Define: 定义方法 equivalent
    // [实体] Variable: key (参数)
    fn equivalent(&self, key: &NameBuf<'_>) -> bool {
        // [关系] Call: 调用 self.keys().eq, key.keys()
        // [关系] UseVar: 使用变量 self, key
        self.keys().eq(key.keys())
    }
}
