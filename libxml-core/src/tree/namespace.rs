use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use libxml_sys::bindings::{xmlFreeNs, xmlNs};

use crate::macros::define_wrapper_types;
use crate::string::maybe_str_from_xml_str;

define_wrapper_types!(ns, Namespace, NamespaceRef, xmlNs);

impl Drop for Namespace {
  fn drop(&mut self) {
    unsafe {
      xmlFreeNs(self.as_ptr());
    }
  }
}

impl PartialEq for Namespace {
  fn eq(&self, other: &Self) -> bool {
    self.deref() == other.deref()
  }
}

impl PartialEq for NamespaceRef {
  fn eq(&self, other: &Self) -> bool {
    self.get_prefix() == other.get_prefix() && self.get_href() == other.get_href()
  }
}

impl Eq for Namespace {}
impl Eq for NamespaceRef {}

impl Hash for Namespace {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.deref().hash(state);
  }
}

impl Hash for NamespaceRef {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.get_prefix().hash(state);
    self.get_href().hash(state);
  }
}

impl NamespaceRef {
  pub fn get_prefix(&self) -> Option<&str> {
    unsafe { maybe_str_from_xml_str(self.0.prefix) }
  }

  pub fn get_href(&self) -> Option<&str> {
    unsafe { maybe_str_from_xml_str(self.0.href) }
  }

  pub fn get_next(&self) -> Option<&Self> {
    unsafe { Self::maybe_from_ptr(self.0.next) }
  }
}
