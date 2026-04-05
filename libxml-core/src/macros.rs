pub trait XmlObj: sealed::Sealed {
  type Inner;
  // unsafe fn from_ptr(ptr: Self::Inner) -> Self;
  // unsafe fn maybe_from_ptr(ptr: Self::Inner) -> Option<Self>;
}

pub(crate) mod sealed {
  pub trait Sealed {}
}

macro_rules! define_wrapper_types {
  ( $desc:tt, $owned:ident, $borrowed:ident, $inner:ty) => {
    /// An owned libxml2 $desc
    #[derive(Debug)]
    pub struct $owned(*mut $inner);

    /// A borrowed libxml2 $desc
    #[derive(Debug)]
    pub struct $borrowed($inner);

    unsafe impl Sync for $owned {}
    unsafe impl Send for $owned {}

    unsafe impl Sync for $borrowed {}
    unsafe impl Send for $borrowed {}

    impl crate::macros::sealed::Sealed for $owned {}
    impl crate::macros::sealed::Sealed for &$borrowed {}

    impl crate::macros::XmlObj for $owned {
      type Inner = *mut $inner;
    }

    impl crate::macros::XmlObj for &$borrowed {
      type Inner = *mut $inner;
    }

    impl $owned {
      /// # Safety
      ///
      /// The caller must ensure that the pointer is either NULL or a valid
      /// pointer to an object of the correct type that we own.
      #[allow(unused)]
      pub(crate) unsafe fn from_ptr(ptr: *mut $inner) -> Self {
        Self(ptr)
      }

      /// # Safety
      ///
      /// The caller must ensure that the pointer is either NULL or a valid
      /// pointer to an object of the correct type that we own.
      #[allow(unused)]
      pub(crate) unsafe fn maybe_from_ptr(ptr: *mut $inner) -> Option<Self> {
        (!ptr.is_null()).then(|| unsafe { Self::from_ptr(ptr) })
      }

      pub(crate) fn as_ptr(&self) -> *mut $inner {
        self.0
      }
    }

    impl $borrowed {
      /// # Safety
      ///
      /// The caller must ensure that the pointer is live and no mutable
      /// references exist for the duration of lifetime 'a.
      pub(crate) unsafe fn from_ptr<'a>(ptr: *mut $inner) -> &'a Self {
        unsafe { &*(ptr as *mut Self) }
      }

      /// # Safety
      ///
      /// The caller must ensure that either the pointer is NULL or that it
      /// meets the requirements of `Self::from_ptr`.
      #[allow(unused)]
      pub(crate) unsafe fn maybe_from_ptr<'a>(ptr: *mut $inner) -> Option<&'a Self> {
        (!ptr.is_null()).then(|| unsafe { Self::from_ptr(ptr) })
      }

      /// # Safety
      ///
      /// The caller must ensure that the pointer is live and that no other
      /// references exist for the duration of lifetime 'a.
      pub(crate) unsafe fn from_ptr_mut<'a>(ptr: *mut $inner) -> &'a mut Self {
        unsafe { &mut *(ptr as *mut Self) }
      }

      /// # Safety
      ///
      /// The caller must ensure that either the pointer is NULL or that it
      /// meets the requirements of `Self::from_ptr_mut`.
      #[allow(unused)]
      pub(crate) unsafe fn maybe_from_ptr_mut<'a>(ptr: *mut $inner) -> Option<&'a mut Self> {
        (!ptr.is_null()).then(|| unsafe { Self::from_ptr_mut(ptr) })
      }

      #[allow(unused)]
      pub(crate) fn as_ptr(&self) -> *mut $inner {
        &self.0 as *const $inner as *mut $inner
      }
    }

    impl Deref for $owned {
      type Target = $borrowed;

      /// SAFETY: lifetime of the borrowed object is linked to the passed-in
      /// reference to the owned object.
      fn deref(&self) -> &Self::Target {
        unsafe { $borrowed::from_ptr(self.as_ptr()) }
      }
    }

    impl DerefMut for $owned {
      // SAFETY: lifetime of the borrowed object is linked to the passed-in
      // reference to the owned object.
      fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { $borrowed::from_ptr_mut(self.0) }
      }
    }
  };
}

macro_rules! define_wrapper_types_with_lifetime {
  ( $desc:tt, $owned:ident, $borrowed:ident, $ref:ty, $inner:ty) => {
    /// An owned libxml2 $desc
    #[derive(Debug)]
    pub struct $owned<'a>(*mut $inner, PhantomData<&'a $ref>);

    /// A borrowed libxml2 $desc
    #[derive(Debug)]
    pub struct $borrowed<'a>($inner, PhantomData<&'a $ref>);

    unsafe impl Sync for $owned<'_> {}
    unsafe impl Send for $owned<'_> {}

    unsafe impl Sync for $borrowed<'_> {}
    unsafe impl Send for $borrowed<'_> {}

    impl crate::macros::sealed::Sealed for $owned<'_> {}
    impl crate::macros::sealed::Sealed for &$borrowed<'_> {}

    impl crate::macros::XmlObj for $owned<'_> {
      type Inner = *mut $inner;
    }

    impl crate::macros::XmlObj for &$borrowed<'_> {
      type Inner = *mut $inner;
    }

    impl<'a> $owned<'a> {
      /// # Safety
      ///
      /// The caller must ensure that the pointer is either NULL or a valid
      /// pointer to an object of the correct type that we own.
      #[allow(unused)]
      pub(crate) unsafe fn from_ptr(ptr: *mut $inner) -> Self {
        Self(ptr, PhantomData)
      }

      /// # Safety
      ///
      /// The caller must ensure that the pointer is either NULL or a valid
      /// pointer to an object of the correct type that we own.
      #[allow(unused)]
      pub(crate) unsafe fn maybe_from_ptr(ptr: *mut $inner) -> Option<Self> {
        (!ptr.is_null()).then(|| unsafe { Self::from_ptr(ptr) })
      }

      pub(crate) fn as_ptr(&self) -> *mut $inner {
        self.0
      }
    }

    impl<'a> $borrowed<'a> {
      /// # Safety
      ///
      /// The caller must ensure that the pointer is live and no mutable
      /// references exist for the duration of lifetime 'a.
      pub(crate) unsafe fn from_ptr<'b>(ptr: *mut $inner) -> &'b Self
      where
        'a: 'b,
      {
        unsafe { &*(ptr as *mut Self) }
      }

      /// # Safety
      ///
      /// The caller must ensure that either the pointer is NULL or that it
      /// meets the requirements of `Self::from_ptr`.
      #[allow(unused)]
      pub(crate) unsafe fn maybe_from_ptr<'b>(ptr: *mut $inner) -> Option<&'b Self>
      where
        'a: 'b,
      {
        (!ptr.is_null()).then(|| unsafe { Self::from_ptr(ptr) })
      }

      /// # Safety
      ///
      /// The caller must ensure that the pointer is live and that no other
      /// references exist for the duration of lifetime 'a.
      pub(crate) unsafe fn from_ptr_mut<'b>(ptr: *mut $inner) -> &'b mut Self
      where
        'a: 'b,
      {
        unsafe { &mut *(ptr as *mut Self) }
      }

      /// # Safety
      ///
      /// The caller must ensure that either the pointer is NULL or that it
      /// meets the requirements of `Self::from_ptr_mut`.
      #[allow(unused)]
      pub(crate) unsafe fn maybe_from_ptr_mut<'b>(ptr: *mut $inner) -> Option<&'b mut Self>
      where
        'a: 'b,
      {
        (!ptr.is_null()).then(|| unsafe { Self::from_ptr_mut(ptr) })
      }

      #[allow(unused)]
      pub(crate) fn as_ptr(&self) -> *mut $inner {
        &self.0 as *const $inner as *mut $inner
      }
    }

    impl<'a> Deref for $owned<'a> {
      type Target = $borrowed<'a>;

      /// SAFETY: lifetime of the borrowed object is linked to the passed-in
      /// reference to the owned object.
      fn deref(&self) -> &Self::Target {
        unsafe { $borrowed::from_ptr(self.as_ptr()) }
      }
    }

    impl<'a> DerefMut for $owned<'a> {
      // SAFETY: lifetime of the borrowed object is linked to the passed-in
      // reference to the owned object.
      fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { $borrowed::from_ptr_mut(self.0) }
      }
    }
  };
}

pub(crate) use define_wrapper_types;
pub(crate) use define_wrapper_types_with_lifetime;
