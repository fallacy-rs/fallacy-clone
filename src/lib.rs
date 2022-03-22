//! Fallible clone.

pub use fallacy_alloc::AllocError;
use std::sync::Arc;

#[cfg(feature = "derive")]
pub use fallacy_clone_derive::TryClone;

/// Tries to clone, return an error instead of panic if allocation failed.
pub trait TryClone: Sized {
    fn try_clone(&self) -> Result<Self, AllocError>;

    /// Performs copy-assignment from `source`.
    ///
    /// `a.try_clone_from(&b)` is equivalent to `a = b.try_clone()` in functionality,
    /// but can be overridden to reuse the resources of `a` to avoid unnecessary
    /// allocations.
    #[inline]
    fn try_clone_from(&mut self, source: &Self) -> Result<(), AllocError> {
        *self = source.try_clone()?;
        Ok(())
    }
}

macro_rules! impl_try_clone {
    ($($val: ty),*) => {
        $(impl TryClone for $val {
            #[inline(always)]
            fn try_clone(&self) -> Result<Self, AllocError> {
                Ok(*self)
            }
            #[inline(always)]
            fn try_clone_from(&mut self, source: &Self) -> Result<(), AllocError> {
                *self = *source;
                Ok(())
            }
        })*
    }
}

impl_try_clone!(bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);

impl<T: ?Sized> TryClone for &T {
    #[inline(always)]
    fn try_clone(&self) -> Result<Self, AllocError> {
        Ok(*self)
    }

    #[inline(always)]
    fn try_clone_from(&mut self, source: &Self) -> Result<(), AllocError> {
        *self = *source;
        Ok(())
    }
}

impl<T: TryClone> TryClone for Option<T> {
    #[inline]
    fn try_clone(&self) -> Result<Self, AllocError> {
        Ok(match self {
            Some(t) => Some(t.try_clone()?),
            None => None,
        })
    }

    #[inline]
    fn try_clone_from(&mut self, source: &Self) -> Result<(), AllocError> {
        match source {
            None => *self = None,
            Some(src) => match self {
                None => *self = Some(src.try_clone()?),
                Some(dest) => dest.try_clone_from(src)?,
            },
        }
        Ok(())
    }
}

impl TryClone for String {
    #[inline]
    fn try_clone(&self) -> Result<Self, AllocError> {
        let mut s = String::new();
        s.try_reserve(s.len())?;
        s.push_str(self);
        Ok(s)
    }

    #[inline]
    fn try_clone_from(&mut self, source: &Self) -> Result<(), AllocError> {
        self.clear();
        self.try_reserve(source.len())?;
        self.push_str(source);
        Ok(())
    }
}

impl<T: ?Sized> TryClone for Arc<T> {
    #[inline]
    fn try_clone(&self) -> Result<Self, AllocError> {
        Ok(self.clone())
    }
}
