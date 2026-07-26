const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__Channel_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Channel {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Channel>
}

impl ::protobuf::Message for Channel {
  type MessageView<'msg> = ChannelView<'msg>;
  type MessageMut<'msg> = ChannelMut<'msg>;
}

impl ::std::default::Default for Channel {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Channel {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Channel` is `Sync` because it does not implement interior mutability.
//    Neither does `ChannelMut`.
unsafe impl ::std::marker::Sync for Channel {}

// SAFETY:
// - `Channel` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Channel {}

impl ::protobuf::Proxied for Channel {
  type View<'msg> = ChannelView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Channel {}

impl ::protobuf::MutProxied for Channel {
  type Mut<'msg> = ChannelMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ChannelView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Channel>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ChannelView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ChannelView<'msg> {
  type Message = Channel;
}

impl ::std::fmt::Debug for ChannelView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ChannelView<'_> {
  fn default() -> ChannelView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Channel>> for ChannelView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Channel>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ChannelView<'msg> {

  pub fn to_owned(&self) -> Channel {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // data_type: optional enum lifechronicle.events.v1.ChannelDataType
  pub fn data_type(self) -> super::ChannelDataType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::ChannelDataType::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // unit: optional string
  pub fn unit(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // scale: optional double
  pub fn scale(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        3, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // offset: optional double
  pub fn offset(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        4, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // missing_value_encoding: optional enum lifechronicle.events.v1.MissingValueEncoding
  pub fn missing_value_encoding(self) -> super::MissingValueEncoding {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::MissingValueEncoding::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // missing_value_sentinel: optional bytes
  pub fn missing_value_sentinel(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `ChannelView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ChannelView<'_> {}

// SAFETY:
// - `ChannelView` is `Send` because while its alive a `ChannelMut` cannot.
// - `ChannelView` does not use thread-local data.
unsafe impl ::std::marker::Send for ChannelView<'_> {}

impl<'msg> ::protobuf::AsView for ChannelView<'msg> {
  type Proxied = Channel;
  fn as_view(&self) -> ::protobuf::View<'msg, Channel> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ChannelView<'msg> {
  fn into_view<'shorter>(self) -> ChannelView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Channel> for ChannelView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Channel {
    let mut dst = Channel::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Channel> for ChannelMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Channel {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Channel {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ChannelView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ChannelMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ChannelMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Channel>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ChannelMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ChannelMut<'msg> {
  type Message = Channel;
}

impl ::std::fmt::Debug for ChannelMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Channel>> for ChannelMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Channel>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ChannelMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Channel> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Channel {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // data_type: optional enum lifechronicle.events.v1.ChannelDataType
  pub fn data_type(&self) -> super::ChannelDataType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::ChannelDataType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_data_type(&mut self, val: super::ChannelDataType) {
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

  // unit: optional string
  pub fn unit(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_unit(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // scale: optional double
  pub fn scale(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        3, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_scale(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        3, val.into()
      )
    }
  }

  // offset: optional double
  pub fn offset(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        4, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_offset(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        4, val.into()
      )
    }
  }

  // missing_value_encoding: optional enum lifechronicle.events.v1.MissingValueEncoding
  pub fn missing_value_encoding(&self) -> super::MissingValueEncoding {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::MissingValueEncoding::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_missing_value_encoding(&mut self, val: super::MissingValueEncoding) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

  // missing_value_sentinel: optional bytes
  pub fn missing_value_sentinel(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_missing_value_sentinel(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

}

// SAFETY:
// - `ChannelMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ChannelMut<'_> {}

// SAFETY:
// - `ChannelMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ChannelMut<'_> {}

impl<'msg> ::protobuf::AsView for ChannelMut<'msg> {
  type Proxied = Channel;
  fn as_view(&self) -> ::protobuf::View<'_, Channel> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ChannelMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Channel>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ChannelMut<'msg> {
  type MutProxied = Channel;
  fn as_mut(&mut self) -> ChannelMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ChannelMut<'msg> {
  fn into_mut<'shorter>(self) -> ChannelMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Channel {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Channel> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ChannelView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ChannelMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // data_type: optional enum lifechronicle.events.v1.ChannelDataType
  pub fn data_type(&self) -> super::ChannelDataType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::ChannelDataType::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_data_type(&mut self, val: super::ChannelDataType) {
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

  // unit: optional string
  pub fn unit(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_unit(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // scale: optional double
  pub fn scale(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        3, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_scale(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        3, val.into()
      )
    }
  }

  // offset: optional double
  pub fn offset(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        4, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_offset(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        4, val.into()
      )
    }
  }

  // missing_value_encoding: optional enum lifechronicle.events.v1.MissingValueEncoding
  pub fn missing_value_encoding(&self) -> super::MissingValueEncoding {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::MissingValueEncoding::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_missing_value_encoding(&mut self, val: super::MissingValueEncoding) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

  // missing_value_sentinel: optional bytes
  pub fn missing_value_sentinel(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_missing_value_sentinel(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

}  // impl Channel

impl ::std::ops::Drop for Channel {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Channel {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Channel {
  type Proxied = Self;
  fn as_view(&self) -> ChannelView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Channel {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ChannelMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Channel {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__Channel_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X.P1X P P.P0P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__Channel_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__Channel_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Channel {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Channel {
  type Msg = Channel;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Channel> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Channel {
  type Msg = Channel;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Channel> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ChannelMut<'_> {
  type Msg = Channel;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Channel> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ChannelMut<'_> {
  type Msg = Channel;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Channel> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ChannelView<'_> {
  type Msg = Channel;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Channel> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ChannelMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__ClockMetadata_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ClockMetadata {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ClockMetadata>
}

impl ::protobuf::Message for ClockMetadata {
  type MessageView<'msg> = ClockMetadataView<'msg>;
  type MessageMut<'msg> = ClockMetadataMut<'msg>;
}

impl ::std::default::Default for ClockMetadata {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ClockMetadata {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ClockMetadata` is `Sync` because it does not implement interior mutability.
//    Neither does `ClockMetadataMut`.
unsafe impl ::std::marker::Sync for ClockMetadata {}

// SAFETY:
// - `ClockMetadata` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ClockMetadata {}

impl ::protobuf::Proxied for ClockMetadata {
  type View<'msg> = ClockMetadataView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ClockMetadata {}

impl ::protobuf::MutProxied for ClockMetadata {
  type Mut<'msg> = ClockMetadataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClockMetadataView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClockMetadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClockMetadataView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClockMetadataView<'msg> {
  type Message = ClockMetadata;
}

impl ::std::fmt::Debug for ClockMetadataView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClockMetadataView<'_> {
  fn default() -> ClockMetadataView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ClockMetadata>> for ClockMetadataView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClockMetadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClockMetadataView<'msg> {

  pub fn to_owned(&self) -> ClockMetadata {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // source: optional enum lifechronicle.events.v1.ClockSource
  pub fn source(self) -> super::ClockSource {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ClockSource::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // clock_id: optional string
  pub fn clock_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // estimated_offset_ns: optional int64
  pub fn estimated_offset_ns(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // uncertainty_ns: optional uint64
  pub fn uncertainty_ns(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // drift_ppm: optional double
  pub fn drift_ppm(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        4, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // adjusted_during_chunk: optional bool
  pub fn adjusted_during_chunk(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ClockMetadataView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ClockMetadataView<'_> {}

// SAFETY:
// - `ClockMetadataView` is `Send` because while its alive a `ClockMetadataMut` cannot.
// - `ClockMetadataView` does not use thread-local data.
unsafe impl ::std::marker::Send for ClockMetadataView<'_> {}

impl<'msg> ::protobuf::AsView for ClockMetadataView<'msg> {
  type Proxied = ClockMetadata;
  fn as_view(&self) -> ::protobuf::View<'msg, ClockMetadata> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClockMetadataView<'msg> {
  fn into_view<'shorter>(self) -> ClockMetadataView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ClockMetadata> for ClockMetadataView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClockMetadata {
    let mut dst = ClockMetadata::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ClockMetadata> for ClockMetadataMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClockMetadata {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ClockMetadata {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClockMetadataView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClockMetadataMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClockMetadataMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClockMetadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClockMetadataMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClockMetadataMut<'msg> {
  type Message = ClockMetadata;
}

impl ::std::fmt::Debug for ClockMetadataMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ClockMetadata>> for ClockMetadataMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClockMetadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClockMetadataMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ClockMetadata> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ClockMetadata {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // source: optional enum lifechronicle.events.v1.ClockSource
  pub fn source(&self) -> super::ClockSource {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ClockSource::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_source(&mut self, val: super::ClockSource) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // clock_id: optional string
  pub fn clock_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_clock_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // estimated_offset_ns: optional int64
  pub fn estimated_offset_ns(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_estimated_offset_ns(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // uncertainty_ns: optional uint64
  pub fn uncertainty_ns(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uncertainty_ns(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // drift_ppm: optional double
  pub fn drift_ppm(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        4, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_drift_ppm(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        4, val.into()
      )
    }
  }

  // adjusted_during_chunk: optional bool
  pub fn adjusted_during_chunk(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_adjusted_during_chunk(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

}

// SAFETY:
// - `ClockMetadataMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ClockMetadataMut<'_> {}

// SAFETY:
// - `ClockMetadataMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ClockMetadataMut<'_> {}

impl<'msg> ::protobuf::AsView for ClockMetadataMut<'msg> {
  type Proxied = ClockMetadata;
  fn as_view(&self) -> ::protobuf::View<'_, ClockMetadata> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClockMetadataMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ClockMetadata>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ClockMetadataMut<'msg> {
  type MutProxied = ClockMetadata;
  fn as_mut(&mut self) -> ClockMetadataMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClockMetadataMut<'msg> {
  fn into_mut<'shorter>(self) -> ClockMetadataMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ClockMetadata {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ClockMetadata> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClockMetadataView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClockMetadataMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // source: optional enum lifechronicle.events.v1.ClockSource
  pub fn source(&self) -> super::ClockSource {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ClockSource::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_source(&mut self, val: super::ClockSource) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // clock_id: optional string
  pub fn clock_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_clock_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // estimated_offset_ns: optional int64
  pub fn estimated_offset_ns(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_estimated_offset_ns(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // uncertainty_ns: optional uint64
  pub fn uncertainty_ns(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uncertainty_ns(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // drift_ppm: optional double
  pub fn drift_ppm(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        4, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_drift_ppm(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        4, val.into()
      )
    }
  }

  // adjusted_during_chunk: optional bool
  pub fn adjusted_during_chunk(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_adjusted_during_chunk(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

}  // impl ClockMetadata

impl ::std::ops::Drop for ClockMetadata {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ClockMetadata {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ClockMetadata {
  type Proxied = Self;
  fn as_view(&self) -> ClockMetadataView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ClockMetadata {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClockMetadataMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ClockMetadata {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__ClockMetadata_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P1X+P,P P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__ClockMetadata_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__ClockMetadata_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClockMetadata {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClockMetadata {
  type Msg = ClockMetadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClockMetadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClockMetadata {
  type Msg = ClockMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClockMetadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClockMetadataMut<'_> {
  type Msg = ClockMetadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClockMetadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClockMetadataMut<'_> {
  type Msg = ClockMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClockMetadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClockMetadataView<'_> {
  type Msg = ClockMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClockMetadata> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClockMetadataMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__SeriesChunk_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SeriesChunk {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SeriesChunk>
}

impl ::protobuf::Message for SeriesChunk {
  type MessageView<'msg> = SeriesChunkView<'msg>;
  type MessageMut<'msg> = SeriesChunkMut<'msg>;
}

impl ::std::default::Default for SeriesChunk {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SeriesChunk {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SeriesChunk` is `Sync` because it does not implement interior mutability.
//    Neither does `SeriesChunkMut`.
unsafe impl ::std::marker::Sync for SeriesChunk {}

// SAFETY:
// - `SeriesChunk` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SeriesChunk {}

impl ::protobuf::Proxied for SeriesChunk {
  type View<'msg> = SeriesChunkView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SeriesChunk {}

impl ::protobuf::MutProxied for SeriesChunk {
  type Mut<'msg> = SeriesChunkMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SeriesChunkView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SeriesChunk>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SeriesChunkView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SeriesChunkView<'msg> {
  type Message = SeriesChunk;
}

impl ::std::fmt::Debug for SeriesChunkView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SeriesChunkView<'_> {
  fn default() -> SeriesChunkView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SeriesChunk>> for SeriesChunkView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SeriesChunk>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SeriesChunkView<'msg> {

  pub fn to_owned(&self) -> SeriesChunk {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // chunk_id: optional string
  pub fn chunk_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // stream: optional string
  pub fn stream(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // schema_version: optional uint32
  pub fn schema_version(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // start_time_ns: optional int64
  pub fn start_time_ns(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // end_time_ns: optional int64
  pub fn end_time_ns(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // nominal_sample_rate: optional double
  pub fn nominal_sample_rate(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        5, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // timestamp_delta_ns: repeated int64
  pub fn timestamp_delta_ns(self) -> ::protobuf::RepeatedView<'msg, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // channels: repeated message lifechronicle.events.v1.Channel
  pub fn channels(self) -> ::protobuf::RepeatedView<'msg, super::Channel> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Channel>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // compressed_payload: optional bytes
  pub fn compressed_payload(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // checksum: optional bytes
  pub fn checksum(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // device_id: optional string
  pub fn device_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // collector_instance_id: optional string
  pub fn collector_instance_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // source: optional string
  pub fn source(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // timezone: optional string
  pub fn timezone(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        13, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // privacy_class: optional enum lifechronicle.events.v1.PrivacyClass
  pub fn privacy_class(self) -> super::PrivacyClass {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        14, (super::PrivacyClass::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // retention_class: optional enum lifechronicle.events.v1.RetentionClass
  pub fn retention_class(self) -> super::RetentionClass {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        15, (super::RetentionClass::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // sequence: optional uint64
  pub fn sequence(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        16, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // clock: optional message lifechronicle.events.v1.ClockMetadata
  pub fn has_clock(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clock_opt(self) -> ::std::option::Option<super::ClockMetadataView<'msg>> {
    self.has_clock().then(|| self.clock())
  }
  pub fn clock(self) -> super::ClockMetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClockMetadataView::default())
  }

}

// SAFETY:
// - `SeriesChunkView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SeriesChunkView<'_> {}

// SAFETY:
// - `SeriesChunkView` is `Send` because while its alive a `SeriesChunkMut` cannot.
// - `SeriesChunkView` does not use thread-local data.
unsafe impl ::std::marker::Send for SeriesChunkView<'_> {}

impl<'msg> ::protobuf::AsView for SeriesChunkView<'msg> {
  type Proxied = SeriesChunk;
  fn as_view(&self) -> ::protobuf::View<'msg, SeriesChunk> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SeriesChunkView<'msg> {
  fn into_view<'shorter>(self) -> SeriesChunkView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SeriesChunk> for SeriesChunkView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SeriesChunk {
    let mut dst = SeriesChunk::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SeriesChunk> for SeriesChunkMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SeriesChunk {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SeriesChunk {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SeriesChunkView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SeriesChunkMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SeriesChunkMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SeriesChunk>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SeriesChunkMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SeriesChunkMut<'msg> {
  type Message = SeriesChunk;
}

impl ::std::fmt::Debug for SeriesChunkMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SeriesChunk>> for SeriesChunkMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SeriesChunk>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SeriesChunkMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SeriesChunk> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SeriesChunk {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // chunk_id: optional string
  pub fn chunk_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_chunk_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // stream: optional string
  pub fn stream(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stream(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // schema_version: optional uint32
  pub fn schema_version(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_schema_version(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // start_time_ns: optional int64
  pub fn start_time_ns(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start_time_ns(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // end_time_ns: optional int64
  pub fn end_time_ns(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end_time_ns(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        4, val.into()
      )
    }
  }

  // nominal_sample_rate: optional double
  pub fn nominal_sample_rate(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        5, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_nominal_sample_rate(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        5, val.into()
      )
    }
  }

  // timestamp_delta_ns: repeated int64
  pub fn timestamp_delta_ns(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn timestamp_delta_ns_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_timestamp_delta_ns(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // channels: repeated message lifechronicle.events.v1.Channel
  pub fn channels(&self) -> ::protobuf::RepeatedView<'_, super::Channel> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Channel>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn channels_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Channel> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_channels(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Channel>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // compressed_payload: optional bytes
  pub fn compressed_payload(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_compressed_payload(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // checksum: optional bytes
  pub fn checksum(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_checksum(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val);
    }
  }

  // collector_instance_id: optional string
  pub fn collector_instance_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_instance_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val);
    }
  }

  // source: optional string
  pub fn source(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_source(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val);
    }
  }

  // timezone: optional string
  pub fn timezone(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        13, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_timezone(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val);
    }
  }

  // privacy_class: optional enum lifechronicle.events.v1.PrivacyClass
  pub fn privacy_class(&self) -> super::PrivacyClass {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        14, (super::PrivacyClass::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_privacy_class(&mut self, val: super::PrivacyClass) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        14, val.into()
      )
    }
  }

  // retention_class: optional enum lifechronicle.events.v1.RetentionClass
  pub fn retention_class(&self) -> super::RetentionClass {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        15, (super::RetentionClass::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_retention_class(&mut self, val: super::RetentionClass) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        15, val.into()
      )
    }
  }

  // sequence: optional uint64
  pub fn sequence(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        16, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_sequence(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        16, val.into()
      )
    }
  }

  // clock: optional message lifechronicle.events.v1.ClockMetadata
  pub fn has_clock(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_clock(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn clock_opt(&self) -> ::std::option::Option<super::ClockMetadataView<'_>> {
    self.has_clock().then(|| self.clock())
  }
  pub fn clock(&self) -> super::ClockMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClockMetadataView::default())
  }
  pub fn clock_mut(&mut self) -> super::ClockMetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         17, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_clock(&mut self,
    val: impl ::protobuf::IntoProxied<super::ClockMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val
      );
    }
  }

}

// SAFETY:
// - `SeriesChunkMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SeriesChunkMut<'_> {}

// SAFETY:
// - `SeriesChunkMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SeriesChunkMut<'_> {}

impl<'msg> ::protobuf::AsView for SeriesChunkMut<'msg> {
  type Proxied = SeriesChunk;
  fn as_view(&self) -> ::protobuf::View<'_, SeriesChunk> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SeriesChunkMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SeriesChunk>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SeriesChunkMut<'msg> {
  type MutProxied = SeriesChunk;
  fn as_mut(&mut self) -> SeriesChunkMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SeriesChunkMut<'msg> {
  fn into_mut<'shorter>(self) -> SeriesChunkMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SeriesChunk {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SeriesChunk> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SeriesChunkView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SeriesChunkMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // chunk_id: optional string
  pub fn chunk_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_chunk_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // stream: optional string
  pub fn stream(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stream(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // schema_version: optional uint32
  pub fn schema_version(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_schema_version(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // start_time_ns: optional int64
  pub fn start_time_ns(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start_time_ns(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // end_time_ns: optional int64
  pub fn end_time_ns(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end_time_ns(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        4, val.into()
      )
    }
  }

  // nominal_sample_rate: optional double
  pub fn nominal_sample_rate(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        5, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_nominal_sample_rate(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        5, val.into()
      )
    }
  }

  // timestamp_delta_ns: repeated int64
  pub fn timestamp_delta_ns(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn timestamp_delta_ns_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_timestamp_delta_ns(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // channels: repeated message lifechronicle.events.v1.Channel
  pub fn channels(&self) -> ::protobuf::RepeatedView<'_, super::Channel> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Channel>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn channels_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Channel> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_channels(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Channel>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // compressed_payload: optional bytes
  pub fn compressed_payload(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_compressed_payload(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // checksum: optional bytes
  pub fn checksum(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_checksum(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val);
    }
  }

  // collector_instance_id: optional string
  pub fn collector_instance_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_instance_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val);
    }
  }

  // source: optional string
  pub fn source(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_source(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val);
    }
  }

  // timezone: optional string
  pub fn timezone(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        13, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_timezone(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val);
    }
  }

  // privacy_class: optional enum lifechronicle.events.v1.PrivacyClass
  pub fn privacy_class(&self) -> super::PrivacyClass {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        14, (super::PrivacyClass::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_privacy_class(&mut self, val: super::PrivacyClass) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        14, val.into()
      )
    }
  }

  // retention_class: optional enum lifechronicle.events.v1.RetentionClass
  pub fn retention_class(&self) -> super::RetentionClass {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        15, (super::RetentionClass::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_retention_class(&mut self, val: super::RetentionClass) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        15, val.into()
      )
    }
  }

  // sequence: optional uint64
  pub fn sequence(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        16, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_sequence(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        16, val.into()
      )
    }
  }

  // clock: optional message lifechronicle.events.v1.ClockMetadata
  pub fn has_clock(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(17)
    }
  }
  pub fn clear_clock(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        17
      );
    }
  }
  pub fn clock_opt(&self) -> ::std::option::Option<super::ClockMetadataView<'_>> {
    self.has_clock().then(|| self.clock())
  }
  pub fn clock(&self) -> super::ClockMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(17)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClockMetadataView::default())
  }
  pub fn clock_mut(&mut self) -> super::ClockMetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         17, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_clock(&mut self,
    val: impl ::protobuf::IntoProxied<super::ClockMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        17,
        val
      );
    }
  }

}  // impl SeriesChunk

impl ::std::ops::Drop for SeriesChunk {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SeriesChunk {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SeriesChunk {
  type Proxied = Self;
  fn as_view(&self) -> SeriesChunkView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SeriesChunk {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SeriesChunkMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SeriesChunk {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__SeriesChunk_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$N1X1X)P+P+P P?G0P0P1X1X1X1X.P.P,P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__SeriesChunk_msg_init.0, &[<super::Channel as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ClockMetadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__SeriesChunk_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SeriesChunk {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SeriesChunk {
  type Msg = SeriesChunk;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesChunk> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeriesChunk {
  type Msg = SeriesChunk;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesChunk> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SeriesChunkMut<'_> {
  type Msg = SeriesChunk;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesChunk> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeriesChunkMut<'_> {
  type Msg = SeriesChunk;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesChunk> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeriesChunkView<'_> {
  type Msg = SeriesChunk;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesChunk> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SeriesChunkMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__SeriesChunkMetadata_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SeriesChunkMetadata {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SeriesChunkMetadata>
}

impl ::protobuf::Message for SeriesChunkMetadata {
  type MessageView<'msg> = SeriesChunkMetadataView<'msg>;
  type MessageMut<'msg> = SeriesChunkMetadataMut<'msg>;
}

impl ::std::default::Default for SeriesChunkMetadata {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SeriesChunkMetadata {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SeriesChunkMetadata` is `Sync` because it does not implement interior mutability.
//    Neither does `SeriesChunkMetadataMut`.
unsafe impl ::std::marker::Sync for SeriesChunkMetadata {}

// SAFETY:
// - `SeriesChunkMetadata` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SeriesChunkMetadata {}

impl ::protobuf::Proxied for SeriesChunkMetadata {
  type View<'msg> = SeriesChunkMetadataView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SeriesChunkMetadata {}

impl ::protobuf::MutProxied for SeriesChunkMetadata {
  type Mut<'msg> = SeriesChunkMetadataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SeriesChunkMetadataView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SeriesChunkMetadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SeriesChunkMetadataView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SeriesChunkMetadataView<'msg> {
  type Message = SeriesChunkMetadata;
}

impl ::std::fmt::Debug for SeriesChunkMetadataView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SeriesChunkMetadataView<'_> {
  fn default() -> SeriesChunkMetadataView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SeriesChunkMetadata>> for SeriesChunkMetadataView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SeriesChunkMetadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SeriesChunkMetadataView<'msg> {

  pub fn to_owned(&self) -> SeriesChunkMetadata {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // chunk_id: optional string
  pub fn chunk_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // stream: optional string
  pub fn stream(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // schema_version: optional uint32
  pub fn schema_version(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // start_time_ns: optional int64
  pub fn start_time_ns(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // end_time_ns: optional int64
  pub fn end_time_ns(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // nominal_sample_rate: optional double
  pub fn nominal_sample_rate(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        5, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // timestamp_delta_ns: repeated int64
  pub fn timestamp_delta_ns(self) -> ::protobuf::RepeatedView<'msg, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // channels: repeated message lifechronicle.events.v1.Channel
  pub fn channels(self) -> ::protobuf::RepeatedView<'msg, super::Channel> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Channel>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // checksum: optional bytes
  pub fn checksum(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // device_id: optional string
  pub fn device_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // collector_instance_id: optional string
  pub fn collector_instance_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // source: optional string
  pub fn source(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // timezone: optional string
  pub fn timezone(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // privacy_class: optional enum lifechronicle.events.v1.PrivacyClass
  pub fn privacy_class(self) -> super::PrivacyClass {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        13, (super::PrivacyClass::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // retention_class: optional enum lifechronicle.events.v1.RetentionClass
  pub fn retention_class(self) -> super::RetentionClass {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        14, (super::RetentionClass::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // sequence: optional uint64
  pub fn sequence(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        15, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // clock: optional message lifechronicle.events.v1.ClockMetadata
  pub fn has_clock(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clock_opt(self) -> ::std::option::Option<super::ClockMetadataView<'msg>> {
    self.has_clock().then(|| self.clock())
  }
  pub fn clock(self) -> super::ClockMetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClockMetadataView::default())
  }

}

// SAFETY:
// - `SeriesChunkMetadataView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SeriesChunkMetadataView<'_> {}

// SAFETY:
// - `SeriesChunkMetadataView` is `Send` because while its alive a `SeriesChunkMetadataMut` cannot.
// - `SeriesChunkMetadataView` does not use thread-local data.
unsafe impl ::std::marker::Send for SeriesChunkMetadataView<'_> {}

impl<'msg> ::protobuf::AsView for SeriesChunkMetadataView<'msg> {
  type Proxied = SeriesChunkMetadata;
  fn as_view(&self) -> ::protobuf::View<'msg, SeriesChunkMetadata> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SeriesChunkMetadataView<'msg> {
  fn into_view<'shorter>(self) -> SeriesChunkMetadataView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SeriesChunkMetadata> for SeriesChunkMetadataView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SeriesChunkMetadata {
    let mut dst = SeriesChunkMetadata::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SeriesChunkMetadata> for SeriesChunkMetadataMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SeriesChunkMetadata {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SeriesChunkMetadata {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SeriesChunkMetadataView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SeriesChunkMetadataMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SeriesChunkMetadataMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SeriesChunkMetadata>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SeriesChunkMetadataMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SeriesChunkMetadataMut<'msg> {
  type Message = SeriesChunkMetadata;
}

impl ::std::fmt::Debug for SeriesChunkMetadataMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SeriesChunkMetadata>> for SeriesChunkMetadataMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SeriesChunkMetadata>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SeriesChunkMetadataMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SeriesChunkMetadata> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SeriesChunkMetadata {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // chunk_id: optional string
  pub fn chunk_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_chunk_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // stream: optional string
  pub fn stream(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stream(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // schema_version: optional uint32
  pub fn schema_version(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_schema_version(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // start_time_ns: optional int64
  pub fn start_time_ns(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start_time_ns(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // end_time_ns: optional int64
  pub fn end_time_ns(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end_time_ns(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        4, val.into()
      )
    }
  }

  // nominal_sample_rate: optional double
  pub fn nominal_sample_rate(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        5, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_nominal_sample_rate(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        5, val.into()
      )
    }
  }

  // timestamp_delta_ns: repeated int64
  pub fn timestamp_delta_ns(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn timestamp_delta_ns_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_timestamp_delta_ns(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // channels: repeated message lifechronicle.events.v1.Channel
  pub fn channels(&self) -> ::protobuf::RepeatedView<'_, super::Channel> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Channel>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn channels_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Channel> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_channels(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Channel>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // checksum: optional bytes
  pub fn checksum(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_checksum(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // collector_instance_id: optional string
  pub fn collector_instance_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_instance_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val);
    }
  }

  // source: optional string
  pub fn source(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_source(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val);
    }
  }

  // timezone: optional string
  pub fn timezone(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_timezone(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val);
    }
  }

  // privacy_class: optional enum lifechronicle.events.v1.PrivacyClass
  pub fn privacy_class(&self) -> super::PrivacyClass {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        13, (super::PrivacyClass::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_privacy_class(&mut self, val: super::PrivacyClass) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        13, val.into()
      )
    }
  }

  // retention_class: optional enum lifechronicle.events.v1.RetentionClass
  pub fn retention_class(&self) -> super::RetentionClass {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        14, (super::RetentionClass::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_retention_class(&mut self, val: super::RetentionClass) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        14, val.into()
      )
    }
  }

  // sequence: optional uint64
  pub fn sequence(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        15, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_sequence(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        15, val.into()
      )
    }
  }

  // clock: optional message lifechronicle.events.v1.ClockMetadata
  pub fn has_clock(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_clock(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn clock_opt(&self) -> ::std::option::Option<super::ClockMetadataView<'_>> {
    self.has_clock().then(|| self.clock())
  }
  pub fn clock(&self) -> super::ClockMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClockMetadataView::default())
  }
  pub fn clock_mut(&mut self) -> super::ClockMetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         16, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_clock(&mut self,
    val: impl ::protobuf::IntoProxied<super::ClockMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

}

// SAFETY:
// - `SeriesChunkMetadataMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SeriesChunkMetadataMut<'_> {}

// SAFETY:
// - `SeriesChunkMetadataMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SeriesChunkMetadataMut<'_> {}

impl<'msg> ::protobuf::AsView for SeriesChunkMetadataMut<'msg> {
  type Proxied = SeriesChunkMetadata;
  fn as_view(&self) -> ::protobuf::View<'_, SeriesChunkMetadata> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SeriesChunkMetadataMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SeriesChunkMetadata>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SeriesChunkMetadataMut<'msg> {
  type MutProxied = SeriesChunkMetadata;
  fn as_mut(&mut self) -> SeriesChunkMetadataMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SeriesChunkMetadataMut<'msg> {
  fn into_mut<'shorter>(self) -> SeriesChunkMetadataMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SeriesChunkMetadata {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SeriesChunkMetadata> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SeriesChunkMetadataView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SeriesChunkMetadataMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // chunk_id: optional string
  pub fn chunk_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_chunk_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // stream: optional string
  pub fn stream(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stream(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // schema_version: optional uint32
  pub fn schema_version(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_schema_version(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // start_time_ns: optional int64
  pub fn start_time_ns(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start_time_ns(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

  // end_time_ns: optional int64
  pub fn end_time_ns(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        4, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end_time_ns(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        4, val.into()
      )
    }
  }

  // nominal_sample_rate: optional double
  pub fn nominal_sample_rate(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        5, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_nominal_sample_rate(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        5, val.into()
      )
    }
  }

  // timestamp_delta_ns: repeated int64
  pub fn timestamp_delta_ns(&self) -> ::protobuf::RepeatedView<'_, i64> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<i64>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn timestamp_delta_ns_mut(&mut self) -> ::protobuf::RepeatedMut<'_, i64> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_timestamp_delta_ns(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<i64>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // channels: repeated message lifechronicle.events.v1.Channel
  pub fn channels(&self) -> ::protobuf::RepeatedView<'_, super::Channel> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Channel>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn channels_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Channel> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_channels(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Channel>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

  // checksum: optional bytes
  pub fn checksum(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_checksum(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // collector_instance_id: optional string
  pub fn collector_instance_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_instance_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val);
    }
  }

  // source: optional string
  pub fn source(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        11, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_source(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val);
    }
  }

  // timezone: optional string
  pub fn timezone(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_timezone(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val);
    }
  }

  // privacy_class: optional enum lifechronicle.events.v1.PrivacyClass
  pub fn privacy_class(&self) -> super::PrivacyClass {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        13, (super::PrivacyClass::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_privacy_class(&mut self, val: super::PrivacyClass) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        13, val.into()
      )
    }
  }

  // retention_class: optional enum lifechronicle.events.v1.RetentionClass
  pub fn retention_class(&self) -> super::RetentionClass {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        14, (super::RetentionClass::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_retention_class(&mut self, val: super::RetentionClass) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        14, val.into()
      )
    }
  }

  // sequence: optional uint64
  pub fn sequence(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        15, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_sequence(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        15, val.into()
      )
    }
  }

  // clock: optional message lifechronicle.events.v1.ClockMetadata
  pub fn has_clock(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_clock(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn clock_opt(&self) -> ::std::option::Option<super::ClockMetadataView<'_>> {
    self.has_clock().then(|| self.clock())
  }
  pub fn clock(&self) -> super::ClockMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClockMetadataView::default())
  }
  pub fn clock_mut(&mut self) -> super::ClockMetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         16, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_clock(&mut self,
    val: impl ::protobuf::IntoProxied<super::ClockMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

}  // impl SeriesChunkMetadata

impl ::std::ops::Drop for SeriesChunkMetadata {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SeriesChunkMetadata {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SeriesChunkMetadata {
  type Proxied = Self;
  fn as_view(&self) -> SeriesChunkMetadataView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SeriesChunkMetadata {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SeriesChunkMetadataMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SeriesChunkMetadata {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__SeriesChunkMetadata_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$N1X1X)P+P+P P?G0P1X1X1X1X.P.P,P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__SeriesChunkMetadata_msg_init.0, &[<super::Channel as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ClockMetadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__SeriesChunkMetadata_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SeriesChunkMetadata {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SeriesChunkMetadata {
  type Msg = SeriesChunkMetadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesChunkMetadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeriesChunkMetadata {
  type Msg = SeriesChunkMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesChunkMetadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SeriesChunkMetadataMut<'_> {
  type Msg = SeriesChunkMetadata;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesChunkMetadata> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeriesChunkMetadataMut<'_> {
  type Msg = SeriesChunkMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesChunkMetadata> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeriesChunkMetadataView<'_> {
  type Msg = SeriesChunkMetadata;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesChunkMetadata> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SeriesChunkMetadataMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__SeriesObjectReference_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SeriesObjectReference {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SeriesObjectReference>
}

impl ::protobuf::Message for SeriesObjectReference {
  type MessageView<'msg> = SeriesObjectReferenceView<'msg>;
  type MessageMut<'msg> = SeriesObjectReferenceMut<'msg>;
}

impl ::std::default::Default for SeriesObjectReference {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SeriesObjectReference {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SeriesObjectReference` is `Sync` because it does not implement interior mutability.
//    Neither does `SeriesObjectReferenceMut`.
unsafe impl ::std::marker::Sync for SeriesObjectReference {}

// SAFETY:
// - `SeriesObjectReference` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SeriesObjectReference {}

impl ::protobuf::Proxied for SeriesObjectReference {
  type View<'msg> = SeriesObjectReferenceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SeriesObjectReference {}

impl ::protobuf::MutProxied for SeriesObjectReference {
  type Mut<'msg> = SeriesObjectReferenceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SeriesObjectReferenceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SeriesObjectReference>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SeriesObjectReferenceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SeriesObjectReferenceView<'msg> {
  type Message = SeriesObjectReference;
}

impl ::std::fmt::Debug for SeriesObjectReferenceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SeriesObjectReferenceView<'_> {
  fn default() -> SeriesObjectReferenceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SeriesObjectReference>> for SeriesObjectReferenceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SeriesObjectReference>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SeriesObjectReferenceView<'msg> {

  pub fn to_owned(&self) -> SeriesObjectReference {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // object_key: optional string
  pub fn object_key(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // object_version: optional string
  pub fn object_version(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // compressed_size: optional uint64
  pub fn compressed_size(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // compressed_sha256: optional bytes
  pub fn compressed_sha256(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // compression: optional enum lifechronicle.events.v1.Compression
  pub fn compression(self) -> super::Compression {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::Compression::Unspecified).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `SeriesObjectReferenceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SeriesObjectReferenceView<'_> {}

// SAFETY:
// - `SeriesObjectReferenceView` is `Send` because while its alive a `SeriesObjectReferenceMut` cannot.
// - `SeriesObjectReferenceView` does not use thread-local data.
unsafe impl ::std::marker::Send for SeriesObjectReferenceView<'_> {}

impl<'msg> ::protobuf::AsView for SeriesObjectReferenceView<'msg> {
  type Proxied = SeriesObjectReference;
  fn as_view(&self) -> ::protobuf::View<'msg, SeriesObjectReference> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SeriesObjectReferenceView<'msg> {
  fn into_view<'shorter>(self) -> SeriesObjectReferenceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SeriesObjectReference> for SeriesObjectReferenceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SeriesObjectReference {
    let mut dst = SeriesObjectReference::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SeriesObjectReference> for SeriesObjectReferenceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SeriesObjectReference {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SeriesObjectReference {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SeriesObjectReferenceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SeriesObjectReferenceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SeriesObjectReferenceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SeriesObjectReference>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SeriesObjectReferenceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SeriesObjectReferenceMut<'msg> {
  type Message = SeriesObjectReference;
}

impl ::std::fmt::Debug for SeriesObjectReferenceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SeriesObjectReference>> for SeriesObjectReferenceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SeriesObjectReference>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SeriesObjectReferenceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SeriesObjectReference> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SeriesObjectReference {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // object_key: optional string
  pub fn object_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_object_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // object_version: optional string
  pub fn object_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_object_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // compressed_size: optional uint64
  pub fn compressed_size(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_compressed_size(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // compressed_sha256: optional bytes
  pub fn compressed_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_compressed_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // compression: optional enum lifechronicle.events.v1.Compression
  pub fn compression(&self) -> super::Compression {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::Compression::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_compression(&mut self, val: super::Compression) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

}

// SAFETY:
// - `SeriesObjectReferenceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SeriesObjectReferenceMut<'_> {}

// SAFETY:
// - `SeriesObjectReferenceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SeriesObjectReferenceMut<'_> {}

impl<'msg> ::protobuf::AsView for SeriesObjectReferenceMut<'msg> {
  type Proxied = SeriesObjectReference;
  fn as_view(&self) -> ::protobuf::View<'_, SeriesObjectReference> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SeriesObjectReferenceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SeriesObjectReference>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SeriesObjectReferenceMut<'msg> {
  type MutProxied = SeriesObjectReference;
  fn as_mut(&mut self) -> SeriesObjectReferenceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SeriesObjectReferenceMut<'msg> {
  fn into_mut<'shorter>(self) -> SeriesObjectReferenceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SeriesObjectReference {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SeriesObjectReference> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SeriesObjectReferenceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SeriesObjectReferenceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // object_key: optional string
  pub fn object_key(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_object_key(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // object_version: optional string
  pub fn object_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_object_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // compressed_size: optional uint64
  pub fn compressed_size(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_compressed_size(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // compressed_sha256: optional bytes
  pub fn compressed_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_compressed_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // compression: optional enum lifechronicle.events.v1.Compression
  pub fn compression(&self) -> super::Compression {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::Compression::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_compression(&mut self, val: super::Compression) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

}  // impl SeriesObjectReference

impl ::std::ops::Drop for SeriesObjectReference {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SeriesObjectReference {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SeriesObjectReference {
  type Proxied = Self;
  fn as_view(&self) -> SeriesObjectReferenceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SeriesObjectReference {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SeriesObjectReferenceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SeriesObjectReference {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__SeriesObjectReference_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X,P0P.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__SeriesObjectReference_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__SeriesObjectReference_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SeriesObjectReference {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SeriesObjectReference {
  type Msg = SeriesObjectReference;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesObjectReference> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeriesObjectReference {
  type Msg = SeriesObjectReference;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesObjectReference> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SeriesObjectReferenceMut<'_> {
  type Msg = SeriesObjectReference;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesObjectReference> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeriesObjectReferenceMut<'_> {
  type Msg = SeriesObjectReference;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesObjectReference> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SeriesObjectReferenceView<'_> {
  type Msg = SeriesObjectReference;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SeriesObjectReference> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SeriesObjectReferenceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChannelDataType(i32);

#[allow(non_upper_case_globals)]
impl ChannelDataType {
  pub const Unspecified: ChannelDataType = ChannelDataType(0);
  pub const Int8: ChannelDataType = ChannelDataType(1);
  pub const Uint8: ChannelDataType = ChannelDataType(2);
  pub const Int16: ChannelDataType = ChannelDataType(3);
  pub const Uint16: ChannelDataType = ChannelDataType(4);
  pub const Int32: ChannelDataType = ChannelDataType(5);
  pub const Uint32: ChannelDataType = ChannelDataType(6);
  pub const Int64: ChannelDataType = ChannelDataType(7);
  pub const Uint64: ChannelDataType = ChannelDataType(8);
  pub const Float32: ChannelDataType = ChannelDataType(9);
  pub const Float64: ChannelDataType = ChannelDataType(10);
  pub const Bool: ChannelDataType = ChannelDataType(11);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Int8",
      2 => "Uint8",
      3 => "Int16",
      4 => "Uint16",
      5 => "Int32",
      6 => "Uint32",
      7 => "Int64",
      8 => "Uint64",
      9 => "Float32",
      10 => "Float64",
      11 => "Bool",
      _ => return None
    })
  }
}

impl ::std::convert::From<ChannelDataType> for i32 {
  fn from(val: ChannelDataType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ChannelDataType {
  fn from(val: i32) -> ChannelDataType {
    Self(val)
  }
}

impl ::std::default::Default for ChannelDataType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ChannelDataType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ChannelDataType::{}", constant_name)
    } else {
      write!(f, "ChannelDataType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ChannelDataType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ChannelDataType {}

impl ::protobuf::Proxied for ChannelDataType {
  type View<'a> = ChannelDataType;
}

impl ::protobuf::AsView for ChannelDataType {
  type Proxied = ChannelDataType;

  fn as_view(&self) -> ChannelDataType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ChannelDataType {
  fn into_view<'shorter>(self) -> ChannelDataType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ChannelDataType {
  const NAME: &'static str = "ChannelDataType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6|7|8|9|10|11)
  }
}

impl ::protobuf::__internal::EntityType for ChannelDataType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MissingValueEncoding(i32);

#[allow(non_upper_case_globals)]
impl MissingValueEncoding {
  pub const Unspecified: MissingValueEncoding = MissingValueEncoding(0);
  pub const None: MissingValueEncoding = MissingValueEncoding(1);
  pub const Sentinel: MissingValueEncoding = MissingValueEncoding(2);
  pub const Bitmap: MissingValueEncoding = MissingValueEncoding(3);
  pub const Nan: MissingValueEncoding = MissingValueEncoding(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "None",
      2 => "Sentinel",
      3 => "Bitmap",
      4 => "Nan",
      _ => return None
    })
  }
}

impl ::std::convert::From<MissingValueEncoding> for i32 {
  fn from(val: MissingValueEncoding) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for MissingValueEncoding {
  fn from(val: i32) -> MissingValueEncoding {
    Self(val)
  }
}

impl ::std::default::Default for MissingValueEncoding {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for MissingValueEncoding {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "MissingValueEncoding::{}", constant_name)
    } else {
      write!(f, "MissingValueEncoding::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for MissingValueEncoding {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for MissingValueEncoding {}

impl ::protobuf::Proxied for MissingValueEncoding {
  type View<'a> = MissingValueEncoding;
}

impl ::protobuf::AsView for MissingValueEncoding {
  type Proxied = MissingValueEncoding;

  fn as_view(&self) -> MissingValueEncoding {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MissingValueEncoding {
  fn into_view<'shorter>(self) -> MissingValueEncoding where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for MissingValueEncoding {
  const NAME: &'static str = "MissingValueEncoding";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for MissingValueEncoding {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClockSource(i32);

#[allow(non_upper_case_globals)]
impl ClockSource {
  pub const Unspecified: ClockSource = ClockSource(0);
  pub const SystemWall: ClockSource = ClockSource(1);
  pub const Monotonic: ClockSource = ClockSource(2);
  pub const NetworkSynced: ClockSource = ClockSource(3);
  pub const Gnss: ClockSource = ClockSource(4);
  pub const Device: ClockSource = ClockSource(5);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "SystemWall",
      2 => "Monotonic",
      3 => "NetworkSynced",
      4 => "Gnss",
      5 => "Device",
      _ => return None
    })
  }
}

impl ::std::convert::From<ClockSource> for i32 {
  fn from(val: ClockSource) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ClockSource {
  fn from(val: i32) -> ClockSource {
    Self(val)
  }
}

impl ::std::default::Default for ClockSource {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ClockSource {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ClockSource::{}", constant_name)
    } else {
      write!(f, "ClockSource::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ClockSource {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ClockSource {}

impl ::protobuf::Proxied for ClockSource {
  type View<'a> = ClockSource;
}

impl ::protobuf::AsView for ClockSource {
  type Proxied = ClockSource;

  fn as_view(&self) -> ClockSource {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClockSource {
  fn into_view<'shorter>(self) -> ClockSource where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ClockSource {
  const NAME: &'static str = "ClockSource";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5)
  }
}

impl ::protobuf::__internal::EntityType for ClockSource {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


