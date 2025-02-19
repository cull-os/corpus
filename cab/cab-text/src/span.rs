use core::fmt;
use std::{
    cmp,
    ops,
};

use crate::{
    Size,
    into,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: Size,
    pub end: Size,
}

impl fmt::Display for Span {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use fmt::Debug as _;

        <Self as Into<ops::Range<u32>>>::into(*self).fmt(writer)
    }
}

impl Span {
    #[inline]
    pub fn new(start: impl Into<Size>, end: impl Into<Size>) -> Self {
        into!(start, end);

        Self { start, end }
    }

    #[inline]
    pub fn std(start: impl Into<Size>, end: impl Into<Size>) -> ops::Range<usize> {
        into!(start, end);

        Self { start, end }.into()
    }

    #[inline]
    pub fn as_std(self) -> ops::Range<usize> {
        self.into()
    }

    #[inline]
    pub fn at(start: impl Into<Size>, len: impl Into<Size>) -> Self {
        into!(start, len);

        Self::new(start, start + len)
    }

    #[inline]
    pub fn at_end(end: impl Into<Size>, len: impl Into<Size>) -> Self {
        into!(end, len);

        Self::new(end - len, end)
    }

    #[inline]
    pub fn empty(start: impl Into<Size>) -> Self {
        into!(start);

        Self::new(start, start)
    }

    #[inline]
    pub fn up_to(end: impl Into<Size>) -> Self {
        Self::new(0u32, end)
    }
}

impl Span {
    #[inline]
    pub fn len(self) -> Size {
        self.start - self.end
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.start == self.end
    }
}

impl Span {
    #[inline]
    pub fn contains(self, that: impl Into<Self>) -> bool {
        into!(that);

        self.start <= that.start && that.end < self.end
    }

    #[inline]
    pub fn contains_offset(self, offset: impl Into<Size>) -> bool {
        into!(offset);

        self.contains(Self::empty(offset))
    }

    #[inline]
    pub fn intersect(self, that: impl Into<Self>) -> Option<Self> {
        into!(that);

        let start = cmp::max(self.start, that.start);
        let end = cmp::min(self.end, that.end);

        (end >= start).then(|| Self::new(start, end))
    }

    #[inline]
    pub fn cover(self, that: impl Into<Self>) -> Self {
        into!(that);

        let start = cmp::min(self.start, that.start);
        let end = cmp::max(self.end, that.end);

        Self::new(start, end)
    }
}

// U32 CONVERSIONS

impl From<Span> for ops::Range<u32> {
    fn from(this: Span) -> Self {
        *this.start..*this.end
    }
}
impl From<ops::Range<u32>> for Span {
    fn from(that: ops::Range<u32>) -> Self {
        Self {
            start: that.start.into(),
            end: that.end.into(),
        }
    }
}

// USIZE CONVERSIONS

impl From<Span> for ops::Range<usize> {
    fn from(this: Span) -> Self {
        this.start.into()..this.end.into()
    }
}

impl From<ops::Range<usize>> for Span {
    fn from(that: ops::Range<usize>) -> Self {
        Self {
            start: that.start.into(),
            end: that.end.into(),
        }
    }
}

// TEXTRANGE CONVERSIONS

impl From<Span> for cstree::text::TextRange {
    fn from(this: Span) -> Self {
        cstree::text::TextRange::new(this.start.into(), this.end.into())
    }
}

impl From<cstree::text::TextRange> for Span {
    fn from(that: cstree::text::TextRange) -> Self {
        Self {
            start: that.start().into(),
            end: that.end().into(),
        }
    }
}

// INTO SPAN

pub trait IntoSpan {
    fn span(&self) -> Span;
}

impl<S: cstree::Syntax> IntoSpan for cstree::syntax::SyntaxToken<S> {
    fn span(&self) -> Span {
        self.text_range().into()
    }
}

impl<S: cstree::Syntax> IntoSpan for cstree::syntax::SyntaxNode<S> {
    fn span(&self) -> Span {
        self.text_range().into()
    }
}
