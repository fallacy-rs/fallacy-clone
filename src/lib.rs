//! Fallible clone.

pub use fallacy_alloc::AllocError;

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

impl_try_clone!(bool, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize);

impl<T> TryClone for &T {
    #[inline]
    fn try_clone(&self) -> Result<Self, AllocError> {
        Ok(*self)
    }

    #[inline]
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
