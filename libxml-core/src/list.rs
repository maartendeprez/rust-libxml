use std::{
  ffi::c_void,
  ops::{Deref, DerefMut},
};

use libxml_sys::c_helpers::bindgenFree;

use crate::macros::XmlObj;

pub struct XmlList<T: XmlObj> {
  ptr: *mut T::Inner,
  len: usize,
}

impl<T: XmlObj> Drop for XmlList<T> {
  fn drop(&mut self) {
    unsafe {
      // self.iter_mut().for_each(|elem| elem.drop());
      bindgenFree(self.ptr as *mut c_void);
    }
  }
}

pub(crate) unsafe fn get_list_len<T>(mut ptr: *mut T) -> usize {
  let mut len = 0;
  while !ptr.is_null() {
    len += 1;
    unsafe {
      ptr = ptr.add(1);
    }
  }
  len
}

impl<T: XmlObj> XmlList<T> {
  pub(crate) unsafe fn from_ptr(ptr: *mut T::Inner) -> Self {
    Self {
      len: unsafe { get_list_len::<T::Inner>(ptr) },
      ptr,
    }
  }

  pub(crate) unsafe fn maybe_from_ptr(ptr: *mut T::Inner) -> Option<Self> {
    (!ptr.is_null()).then(|| unsafe { Self::from_ptr(ptr) })
  }

  #[allow(unused)]
  pub(crate) fn as_ptr(&self) -> *mut T::Inner {
    self.ptr
  }
}

impl<T: XmlObj> Deref for XmlList<T> {
  type Target = [T];

  fn deref(&self) -> &Self::Target {
    unsafe { std::slice::from_raw_parts(self.ptr as *const T, self.len) }
  }
}

impl<T: XmlObj> DerefMut for XmlList<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { std::slice::from_raw_parts_mut(self.ptr as *mut T, self.len) }
  }
}
