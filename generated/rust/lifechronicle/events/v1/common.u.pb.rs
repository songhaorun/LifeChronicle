const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__QualitySignal_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct QualitySignal {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<QualitySignal>
}

impl ::protobuf::Message for QualitySignal {
  type MessageView<'msg> = QualitySignalView<'msg>;
  type MessageMut<'msg> = QualitySignalMut<'msg>;
}

impl ::std::default::Default for QualitySignal {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for QualitySignal {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `QualitySignal` is `Sync` because it does not implement interior mutability.
//    Neither does `QualitySignalMut`.
unsafe impl ::std::marker::Sync for QualitySignal {}

// SAFETY:
// - `QualitySignal` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for QualitySignal {}

impl ::protobuf::Proxied for QualitySignal {
  type View<'msg> = QualitySignalView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for QualitySignal {}

impl ::protobuf::MutProxied for QualitySignal {
  type Mut<'msg> = QualitySignalMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct QualitySignalView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, QualitySignal>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for QualitySignalView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for QualitySignalView<'msg> {
  type Message = QualitySignal;
}

impl ::std::fmt::Debug for QualitySignalView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for QualitySignalView<'_> {
  fn default() -> QualitySignalView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, QualitySignal>> for QualitySignalView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, QualitySignal>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> QualitySignalView<'msg> {

  pub fn to_owned(&self) -> QualitySignal {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // code: optional string
  pub fn code(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // severity: optional enum lifechronicle.events.v1.QualitySeverity
  pub fn severity(self) -> super::QualitySeverity {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::QualitySeverity::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // field_path: optional string
  pub fn field_path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // detail_id: optional string
  pub fn detail_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `QualitySignalView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for QualitySignalView<'_> {}

// SAFETY:
// - `QualitySignalView` is `Send` because while its alive a `QualitySignalMut` cannot.
// - `QualitySignalView` does not use thread-local data.
unsafe impl ::std::marker::Send for QualitySignalView<'_> {}

impl<'msg> ::protobuf::AsView for QualitySignalView<'msg> {
  type Proxied = QualitySignal;
  fn as_view(&self) -> ::protobuf::View<'msg, QualitySignal> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for QualitySignalView<'msg> {
  fn into_view<'shorter>(self) -> QualitySignalView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<QualitySignal> for QualitySignalView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> QualitySignal {
    let mut dst = QualitySignal::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<QualitySignal> for QualitySignalMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> QualitySignal {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for QualitySignal {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for QualitySignalView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for QualitySignalMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct QualitySignalMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, QualitySignal>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for QualitySignalMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for QualitySignalMut<'msg> {
  type Message = QualitySignal;
}

impl ::std::fmt::Debug for QualitySignalMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, QualitySignal>> for QualitySignalMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, QualitySignal>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> QualitySignalMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, QualitySignal> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> QualitySignal {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // code: optional string
  pub fn code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // severity: optional enum lifechronicle.events.v1.QualitySeverity
  pub fn severity(&self) -> super::QualitySeverity {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::QualitySeverity::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_severity(&mut self, val: super::QualitySeverity) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // field_path: optional string
  pub fn field_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_field_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // detail_id: optional string
  pub fn detail_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_detail_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}

// SAFETY:
// - `QualitySignalMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for QualitySignalMut<'_> {}

// SAFETY:
// - `QualitySignalMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for QualitySignalMut<'_> {}

impl<'msg> ::protobuf::AsView for QualitySignalMut<'msg> {
  type Proxied = QualitySignal;
  fn as_view(&self) -> ::protobuf::View<'_, QualitySignal> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for QualitySignalMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, QualitySignal>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for QualitySignalMut<'msg> {
  type MutProxied = QualitySignal;
  fn as_mut(&mut self) -> QualitySignalMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for QualitySignalMut<'msg> {
  fn into_mut<'shorter>(self) -> QualitySignalMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl QualitySignal {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, QualitySignal> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> QualitySignalView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> QualitySignalMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // code: optional string
  pub fn code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // severity: optional enum lifechronicle.events.v1.QualitySeverity
  pub fn severity(&self) -> super::QualitySeverity {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::QualitySeverity::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_severity(&mut self, val: super::QualitySeverity) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // field_path: optional string
  pub fn field_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_field_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // detail_id: optional string
  pub fn detail_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_detail_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}  // impl QualitySignal

impl ::std::ops::Drop for QualitySignal {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for QualitySignal {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for QualitySignal {
  type Proxied = Self;
  fn as_view(&self) -> QualitySignalView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for QualitySignal {
  type MutProxied = Self;
  fn as_mut(&mut self) -> QualitySignalMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for QualitySignal {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__QualitySignal_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X.P1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__QualitySignal_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__QualitySignal_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for QualitySignal {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for QualitySignal {
  type Msg = QualitySignal;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QualitySignal> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QualitySignal {
  type Msg = QualitySignal;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QualitySignal> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for QualitySignalMut<'_> {
  type Msg = QualitySignal;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QualitySignal> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QualitySignalMut<'_> {
  type Msg = QualitySignal;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QualitySignal> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for QualitySignalView<'_> {
  type Msg = QualitySignal;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<QualitySignal> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for QualitySignalMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecordKind(i32);

#[allow(non_upper_case_globals)]
impl RecordKind {
  pub const Unspecified: RecordKind = RecordKind(0);
  pub const State: RecordKind = RecordKind(1);
  pub const Interval: RecordKind = RecordKind(2);
  pub const Sample: RecordKind = RecordKind(3);
  pub const Delta: RecordKind = RecordKind(4);
  pub const Series: RecordKind = RecordKind(5);
  pub const Annotation: RecordKind = RecordKind(6);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "State",
      2 => "Interval",
      3 => "Sample",
      4 => "Delta",
      5 => "Series",
      6 => "Annotation",
      _ => return None
    })
  }
}

impl ::std::convert::From<RecordKind> for i32 {
  fn from(val: RecordKind) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for RecordKind {
  fn from(val: i32) -> RecordKind {
    Self(val)
  }
}

impl ::std::default::Default for RecordKind {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for RecordKind {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "RecordKind::{}", constant_name)
    } else {
      write!(f, "RecordKind::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for RecordKind {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for RecordKind {}

impl ::protobuf::Proxied for RecordKind {
  type View<'a> = RecordKind;
}

impl ::protobuf::AsView for RecordKind {
  type Proxied = RecordKind;

  fn as_view(&self) -> RecordKind {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RecordKind {
  fn into_view<'shorter>(self) -> RecordKind where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for RecordKind {
  const NAME: &'static str = "RecordKind";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6)
  }
}

impl ::protobuf::__internal::EntityType for RecordKind {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PrivacyClass(i32);

#[allow(non_upper_case_globals)]
impl PrivacyClass {
  pub const Unspecified: PrivacyClass = PrivacyClass(0);
  pub const Private: PrivacyClass = PrivacyClass(1);
  pub const Sensitive: PrivacyClass = PrivacyClass(2);
  pub const Restricted: PrivacyClass = PrivacyClass(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Private",
      2 => "Sensitive",
      3 => "Restricted",
      _ => return None
    })
  }
}

impl ::std::convert::From<PrivacyClass> for i32 {
  fn from(val: PrivacyClass) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for PrivacyClass {
  fn from(val: i32) -> PrivacyClass {
    Self(val)
  }
}

impl ::std::default::Default for PrivacyClass {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for PrivacyClass {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "PrivacyClass::{}", constant_name)
    } else {
      write!(f, "PrivacyClass::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for PrivacyClass {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for PrivacyClass {}

impl ::protobuf::Proxied for PrivacyClass {
  type View<'a> = PrivacyClass;
}

impl ::protobuf::AsView for PrivacyClass {
  type Proxied = PrivacyClass;

  fn as_view(&self) -> PrivacyClass {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PrivacyClass {
  fn into_view<'shorter>(self) -> PrivacyClass where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for PrivacyClass {
  const NAME: &'static str = "PrivacyClass";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for PrivacyClass {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RetentionClass(i32);

#[allow(non_upper_case_globals)]
impl RetentionClass {
  pub const Unspecified: RetentionClass = RetentionClass(0);
  pub const Standard: RetentionClass = RetentionClass(1);
  pub const LongTerm: RetentionClass = RetentionClass(2);
  pub const Ephemeral: RetentionClass = RetentionClass(3);
  pub const UserManaged: RetentionClass = RetentionClass(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Standard",
      2 => "LongTerm",
      3 => "Ephemeral",
      4 => "UserManaged",
      _ => return None
    })
  }
}

impl ::std::convert::From<RetentionClass> for i32 {
  fn from(val: RetentionClass) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for RetentionClass {
  fn from(val: i32) -> RetentionClass {
    Self(val)
  }
}

impl ::std::default::Default for RetentionClass {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for RetentionClass {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "RetentionClass::{}", constant_name)
    } else {
      write!(f, "RetentionClass::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for RetentionClass {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for RetentionClass {}

impl ::protobuf::Proxied for RetentionClass {
  type View<'a> = RetentionClass;
}

impl ::protobuf::AsView for RetentionClass {
  type Proxied = RetentionClass;

  fn as_view(&self) -> RetentionClass {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RetentionClass {
  fn into_view<'shorter>(self) -> RetentionClass where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for RetentionClass {
  const NAME: &'static str = "RetentionClass";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for RetentionClass {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Compression(i32);

#[allow(non_upper_case_globals)]
impl Compression {
  pub const Unspecified: Compression = Compression(0);
  pub const Zstd: Compression = Compression(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Zstd",
      _ => return None
    })
  }
}

impl ::std::convert::From<Compression> for i32 {
  fn from(val: Compression) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for Compression {
  fn from(val: i32) -> Compression {
    Self(val)
  }
}

impl ::std::default::Default for Compression {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Compression {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Compression::{}", constant_name)
    } else {
      write!(f, "Compression::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Compression {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Compression {}

impl ::protobuf::Proxied for Compression {
  type View<'a> = Compression;
}

impl ::protobuf::AsView for Compression {
  type Proxied = Compression;

  fn as_view(&self) -> Compression {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Compression {
  fn into_view<'shorter>(self) -> Compression where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Compression {
  const NAME: &'static str = "Compression";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for Compression {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QualitySeverity(i32);

#[allow(non_upper_case_globals)]
impl QualitySeverity {
  pub const Unspecified: QualitySeverity = QualitySeverity(0);
  pub const Info: QualitySeverity = QualitySeverity(1);
  pub const Warning: QualitySeverity = QualitySeverity(2);
  pub const Error: QualitySeverity = QualitySeverity(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Info",
      2 => "Warning",
      3 => "Error",
      _ => return None
    })
  }
}

impl ::std::convert::From<QualitySeverity> for i32 {
  fn from(val: QualitySeverity) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for QualitySeverity {
  fn from(val: i32) -> QualitySeverity {
    Self(val)
  }
}

impl ::std::default::Default for QualitySeverity {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for QualitySeverity {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "QualitySeverity::{}", constant_name)
    } else {
      write!(f, "QualitySeverity::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for QualitySeverity {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for QualitySeverity {}

impl ::protobuf::Proxied for QualitySeverity {
  type View<'a> = QualitySeverity;
}

impl ::protobuf::AsView for QualitySeverity {
  type Proxied = QualitySeverity;

  fn as_view(&self) -> QualitySeverity {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for QualitySeverity {
  fn into_view<'shorter>(self) -> QualitySeverity where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for QualitySeverity {
  const NAME: &'static str = "QualitySeverity";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for QualitySeverity {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


