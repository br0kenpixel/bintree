use crate::BinTree;
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

impl<T: Debug> Debug for BinTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T: Display> Display for BinTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T: Clone> Clone for BinTree<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.clone_inner(),
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

impl<T> Deref for BinTree<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for BinTree<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
