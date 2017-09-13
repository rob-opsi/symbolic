use std::path::Path;
use std::borrow::Cow;
use std::ops::Deref;

use memmap::{Mmap, Protection};
use owning_ref::OwningHandle;

use errors::Result;

/// Gives access to bytes loaded from somewhere.
///
/// This type can be used to uniformly access bytes that were created
/// either from mmapping in a path, a vector or a borrowed slice.
pub enum ByteViewInner<'a> {
    Buf(Cow<'a, [u8]>),
    Mmap(Mmap),
}

pub struct ByteView<'a> {
    inner: ByteViewInner<'a>,
}

impl<'a> ByteView<'a> {
    /// Constructs a byte view from a Cow.
    pub fn from_cow(cow: Cow<'a, [u8]>) -> ByteView<'a> {
        ByteView {
            inner: ByteViewInner::Buf(cow)
        }
    }

    /// Constructs an object file from a byte slice.
    pub fn from_slice(buffer: &'a [u8]) -> ByteView<'a> {
        ByteView::from_cow(Cow::Borrowed(buffer))
    }

    /// Constructs an object file from a vector.
    pub fn from_vec(buffer: Vec<u8>) -> ByteView<'static> {
        ByteView::from_cow(Cow::Owned(buffer))
    }

    /// Constructs an object file from a file path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ByteView<'static>> {
        let mmap = Mmap::open_path(path, Protection::Read)?;
        Ok(ByteView {
            inner: ByteViewInner::Mmap(mmap)
        })
    }

    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        match self.inner {
            ByteViewInner::Buf(ref buf) => buf,
            ByteViewInner::Mmap(ref mmap) => unsafe { mmap.as_slice() },
        }
    }
}

impl<'a> Deref for ByteView<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.buffer()
    }
}

impl<'a> AsRef<[u8]> for ByteView<'a> {
    fn as_ref(&self) -> &[u8] {
        self.buffer()
    }
}

/// Like `ByteView` but owns an object based on it.
///
/// In some situations symbolic needs to deal with types that are
/// based on potentially owned or borrowed bytes and wants to provide
/// another view at them.
pub struct ByteViewHandle<'a, T> {
    inner: OwningHandle<Box<ByteView<'a>>, Box<(&'a [u8], T)>>,
}

impl<'a, T> ByteViewHandle<'a, T> {
    /// Creates a new byte view backing from a `ByteView`.
    pub fn new<F: FnOnce(&'a [u8]) -> Result<T>>(
        byteview: ByteView<'a>, f: F) -> Result<ByteViewHandle<'a, T>>
    {
        Ok(ByteViewHandle {
            inner: OwningHandle::try_new(Box::new(byteview), |bv| -> Result<_> {
                let bytes: &[u8] = unsafe { &*bv };
                Ok(Box::new((bytes, f(bytes)?)))
            })?
        })
    }

    /// Returns the underlying storage (byte slice).
    pub fn get_bytes(this: &ByteViewHandle<'a, T>) -> &'a [u8] {
        this.inner.0
    }
}

impl<'a, T> Deref for ByteViewHandle<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner.1
    }
}

impl<'a, T> AsRef<T> for ByteViewHandle<'a, T> {
    fn as_ref(&self) -> &T {
        &self.inner.1
    }
}
