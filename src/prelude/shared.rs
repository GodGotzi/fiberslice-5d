use std::{cell::RefCell, rc::Rc, sync::Arc};

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Default, Debug)]
pub struct SharedMut<T: std::fmt::Debug> {
    inner: Arc<RwLock<T>>,
}

impl<T: std::fmt::Debug> Clone for SharedMut<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: std::fmt::Debug> SharedMut<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        self.inner.read()
    }

    pub fn read_with_fn<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let inner = self.inner.read();
        f(&*inner)
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.inner.write()
    }

    pub fn write_with_fn<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut inner = self.inner.write();
        f(&mut *inner)
    }
}

#[derive(Debug, Default)]
pub struct Shared<T> {
    inner: Arc<T>,
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Shared<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }
}

#[derive(Debug)]
pub struct UnparallelShared<T> {
    inner: Rc<T>,
}

impl<T> Clone for UnparallelShared<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> UnparallelShared<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }
}

#[derive(Debug)]
pub struct UnparallelSharedMut<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> Clone for UnparallelSharedMut<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> UnparallelSharedMut<T> {
    pub fn from_inner(inner: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    pub fn inner(&self) -> &RefCell<T> {
        &self.inner
    }
}
