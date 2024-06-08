use std::sync::Arc;

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Default, Debug)]
pub struct Wrapper<T> {
    pub inner: T,
}

#[derive(Clone, Default, Debug)]
pub struct WrappedSharedMut<T: std::fmt::Debug> {
    inner: Arc<RwLock<Wrapper<T>>>,
}

impl<T: std::fmt::Debug> WrappedSharedMut<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(Wrapper { inner })),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<Wrapper<T>> {
        self.inner.read()
    }

    pub fn write(&self) -> RwLockWriteGuard<Wrapper<T>> {
        self.inner.write()
    }
}

#[derive(Default, Debug)]
pub struct WrappedShared<T: std::fmt::Debug> {
    inner: Arc<Wrapper<T>>,
}

impl<T: std::fmt::Debug> WrappedShared<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Arc::new(Wrapper { inner }),
        }
    }

    pub fn inner(&self) -> &T {
        &self.inner.inner
    }
}
