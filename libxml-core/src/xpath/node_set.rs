use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use libxml_sys::bindings::{xmlNodeSet, xmlXPathFreeNodeSet};

use crate::tree::NodeRef;
use crate::{macros::define_wrapper_types_with_lifetime, tree::DocumentRef};

define_wrapper_types_with_lifetime!(xpathnodeset, NodeSet, NodeSetRef, DocumentRef, xmlNodeSet);

impl Drop for NodeSet<'_> {
  fn drop(&mut self) {
    unsafe { xmlXPathFreeNodeSet(self.as_ptr()) }
  }
}

impl<'a> NodeSetRef<'a> {
  pub fn len(&self) -> usize {
    self.0.nodeNr as usize
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }
}

impl<'a> Deref for NodeSetRef<'a> {
  type Target = [&'a NodeRef];

  fn deref(&self) -> &Self::Target {
    unsafe { std::slice::from_raw_parts(self.0.nodeTab as *mut &'a NodeRef, self.len()) }
  }
}

impl<'a> DerefMut for NodeSetRef<'a> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { std::slice::from_raw_parts_mut(self.0.nodeTab as *mut &'a NodeRef, self.len()) }
  }
}
