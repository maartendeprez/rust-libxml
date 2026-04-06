use std::{
  ffi::{CStr, c_void},
  fmt::{Debug, Display},
  ops::Deref,
};

use libc::c_char;
use libxml_sys::{bindings::xmlChar, c_helpers::bindgenFree};

/// Convert a borrowed libxml2 C string to a valid rust &str.
///
/// # Safety
///
/// The caller is responsible for passing in a non-null pointer to a valid c
/// string, and for choosing a suitable lifetime 'a, ensuring the memory is not
/// dallocated for that duration.
pub(crate) unsafe fn str_from_xml_str<'a>(ptr: *const xmlChar) -> &'a str {
  let s = unsafe { CStr::from_ptr(ptr as *const c_char) };
  // Docs say `xmlChar` is "a basic byte in an UTF-8 encoded string".
  s.to_str()
    .expect("xml string should be a valid UTF-8 string")
}

/// Convert a borrowed libxml2 C string or NULL to a valid rust &str.
///
/// # Safety
///
/// The caller is responsible for passing in a non-null pointer to a valid c
/// string, and for choosing a suitable lifetime 'a, ensuring the memory is not
/// dallocated for that duration.
pub(crate) unsafe fn maybe_str_from_xml_str<'a>(ptr: *const xmlChar) -> Option<&'a str> {
  (!ptr.is_null()).then(|| unsafe { str_from_xml_str(ptr) })
}

pub struct XmlString(*const xmlChar);

impl XmlString {
  pub(crate) unsafe fn new(ptr: *const xmlChar) -> Self {
    Self(ptr)
  }

  pub(crate) unsafe fn maybe_new(ptr: *const xmlChar) -> Option<Self> {
    (!ptr.is_null()).then_some(Self(ptr))
  }
}

impl PartialEq<Self> for XmlString {
  fn eq(&self, other: &Self) -> bool {
    self.deref() == other.deref()
  }
}

impl PartialEq<str> for XmlString {
  fn eq(&self, other: &str) -> bool {
    self.deref() == other
  }
}

impl Display for XmlString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.deref())
  }
}

impl Debug for XmlString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "XmlString({:?})", self.deref())
  }
}

impl Deref for XmlString {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    // SAFETY: lifetime of return value is tied to the owned `XmlString`.
    unsafe { str_from_xml_str(self.0) }
  }
}

impl Drop for XmlString {
  fn drop(&mut self) {
    unsafe {
      bindgenFree(self.0 as *mut c_void);
    }
  }
}
