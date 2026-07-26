const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__AppForeground_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AppForeground {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AppForeground>
}

impl ::protobuf::Message for AppForeground {
  type MessageView<'msg> = AppForegroundView<'msg>;
  type MessageMut<'msg> = AppForegroundMut<'msg>;
}

impl ::std::default::Default for AppForeground {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AppForeground {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AppForeground` is `Sync` because it does not implement interior mutability.
//    Neither does `AppForegroundMut`.
unsafe impl ::std::marker::Sync for AppForeground {}

// SAFETY:
// - `AppForeground` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AppForeground {}

impl ::protobuf::Proxied for AppForeground {
  type View<'msg> = AppForegroundView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AppForeground {}

impl ::protobuf::MutProxied for AppForeground {
  type Mut<'msg> = AppForegroundMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AppForegroundView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AppForeground>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AppForegroundView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AppForegroundView<'msg> {
  type Message = AppForeground;
}

impl ::std::fmt::Debug for AppForegroundView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AppForegroundView<'_> {
  fn default() -> AppForegroundView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AppForeground>> for AppForegroundView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AppForeground>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AppForegroundView<'msg> {

  pub fn to_owned(&self) -> AppForeground {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // application_id: optional string
  pub fn application_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // application_name: optional string
  pub fn application_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // executable_name: optional string
  pub fn executable_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // window_title: optional string
  pub fn window_title(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // process_id: optional uint64
  pub fn process_id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // window_id: optional string
  pub fn window_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // fullscreen: optional bool
  pub fn fullscreen(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `AppForegroundView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AppForegroundView<'_> {}

// SAFETY:
// - `AppForegroundView` is `Send` because while its alive a `AppForegroundMut` cannot.
// - `AppForegroundView` does not use thread-local data.
unsafe impl ::std::marker::Send for AppForegroundView<'_> {}

impl<'msg> ::protobuf::AsView for AppForegroundView<'msg> {
  type Proxied = AppForeground;
  fn as_view(&self) -> ::protobuf::View<'msg, AppForeground> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AppForegroundView<'msg> {
  fn into_view<'shorter>(self) -> AppForegroundView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AppForeground> for AppForegroundView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AppForeground {
    let mut dst = AppForeground::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AppForeground> for AppForegroundMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AppForeground {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AppForeground {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AppForegroundView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AppForegroundMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AppForegroundMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AppForeground>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AppForegroundMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AppForegroundMut<'msg> {
  type Message = AppForeground;
}

impl ::std::fmt::Debug for AppForegroundMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AppForeground>> for AppForegroundMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AppForeground>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AppForegroundMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AppForeground> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AppForeground {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // application_id: optional string
  pub fn application_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_application_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // application_name: optional string
  pub fn application_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_application_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // executable_name: optional string
  pub fn executable_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_executable_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // window_title: optional string
  pub fn window_title(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_window_title(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // process_id: optional uint64
  pub fn process_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_process_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        4, val.into()
      )
    }
  }

  // window_id: optional string
  pub fn window_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_window_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // fullscreen: optional bool
  pub fn fullscreen(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fullscreen(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

}

// SAFETY:
// - `AppForegroundMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AppForegroundMut<'_> {}

// SAFETY:
// - `AppForegroundMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AppForegroundMut<'_> {}

impl<'msg> ::protobuf::AsView for AppForegroundMut<'msg> {
  type Proxied = AppForeground;
  fn as_view(&self) -> ::protobuf::View<'_, AppForeground> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AppForegroundMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AppForeground>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AppForegroundMut<'msg> {
  type MutProxied = AppForeground;
  fn as_mut(&mut self) -> AppForegroundMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AppForegroundMut<'msg> {
  fn into_mut<'shorter>(self) -> AppForegroundMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AppForeground {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AppForeground> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AppForegroundView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AppForegroundMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // application_id: optional string
  pub fn application_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_application_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // application_name: optional string
  pub fn application_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_application_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // executable_name: optional string
  pub fn executable_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_executable_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // window_title: optional string
  pub fn window_title(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_window_title(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // process_id: optional uint64
  pub fn process_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_process_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        4, val.into()
      )
    }
  }

  // window_id: optional string
  pub fn window_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_window_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // fullscreen: optional bool
  pub fn fullscreen(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fullscreen(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

}  // impl AppForeground

impl ::std::ops::Drop for AppForeground {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AppForeground {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AppForeground {
  type Proxied = Self;
  fn as_view(&self) -> AppForegroundView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AppForeground {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AppForegroundMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AppForeground {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__AppForeground_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X1X1X,P1X/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__AppForeground_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__AppForeground_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AppForeground {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AppForeground {
  type Msg = AppForeground;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AppForeground> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AppForeground {
  type Msg = AppForeground;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AppForeground> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AppForegroundMut<'_> {
  type Msg = AppForeground;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AppForeground> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AppForegroundMut<'_> {
  type Msg = AppForeground;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AppForeground> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AppForegroundView<'_> {
  type Msg = AppForeground;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AppForeground> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AppForegroundMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__IdleState_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct IdleState {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<IdleState>
}

impl ::protobuf::Message for IdleState {
  type MessageView<'msg> = IdleStateView<'msg>;
  type MessageMut<'msg> = IdleStateMut<'msg>;
}

impl ::std::default::Default for IdleState {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for IdleState {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `IdleState` is `Sync` because it does not implement interior mutability.
//    Neither does `IdleStateMut`.
unsafe impl ::std::marker::Sync for IdleState {}

// SAFETY:
// - `IdleState` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for IdleState {}

impl ::protobuf::Proxied for IdleState {
  type View<'msg> = IdleStateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for IdleState {}

impl ::protobuf::MutProxied for IdleState {
  type Mut<'msg> = IdleStateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct IdleStateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, IdleState>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for IdleStateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for IdleStateView<'msg> {
  type Message = IdleState;
}

impl ::std::fmt::Debug for IdleStateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for IdleStateView<'_> {
  fn default() -> IdleStateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, IdleState>> for IdleStateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, IdleState>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> IdleStateView<'msg> {

  pub fn to_owned(&self) -> IdleState {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // status: optional enum lifechronicle.events.v1.IdleStatus
  pub fn status(self) -> super::IdleStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::IdleStatus::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // idle_duration_ms: optional uint64
  pub fn idle_duration_ms(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `IdleStateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for IdleStateView<'_> {}

// SAFETY:
// - `IdleStateView` is `Send` because while its alive a `IdleStateMut` cannot.
// - `IdleStateView` does not use thread-local data.
unsafe impl ::std::marker::Send for IdleStateView<'_> {}

impl<'msg> ::protobuf::AsView for IdleStateView<'msg> {
  type Proxied = IdleState;
  fn as_view(&self) -> ::protobuf::View<'msg, IdleState> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IdleStateView<'msg> {
  fn into_view<'shorter>(self) -> IdleStateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<IdleState> for IdleStateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> IdleState {
    let mut dst = IdleState::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<IdleState> for IdleStateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> IdleState {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for IdleState {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for IdleStateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for IdleStateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct IdleStateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, IdleState>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for IdleStateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for IdleStateMut<'msg> {
  type Message = IdleState;
}

impl ::std::fmt::Debug for IdleStateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, IdleState>> for IdleStateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, IdleState>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> IdleStateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, IdleState> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> IdleState {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // status: optional enum lifechronicle.events.v1.IdleStatus
  pub fn status(&self) -> super::IdleStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::IdleStatus::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_status(&mut self, val: super::IdleStatus) {
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

  // idle_duration_ms: optional uint64
  pub fn idle_duration_ms(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_idle_duration_ms(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `IdleStateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for IdleStateMut<'_> {}

// SAFETY:
// - `IdleStateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for IdleStateMut<'_> {}

impl<'msg> ::protobuf::AsView for IdleStateMut<'msg> {
  type Proxied = IdleState;
  fn as_view(&self) -> ::protobuf::View<'_, IdleState> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IdleStateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, IdleState>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for IdleStateMut<'msg> {
  type MutProxied = IdleState;
  fn as_mut(&mut self) -> IdleStateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for IdleStateMut<'msg> {
  fn into_mut<'shorter>(self) -> IdleStateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl IdleState {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, IdleState> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> IdleStateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> IdleStateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // status: optional enum lifechronicle.events.v1.IdleStatus
  pub fn status(&self) -> super::IdleStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::IdleStatus::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_status(&mut self, val: super::IdleStatus) {
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

  // idle_duration_ms: optional uint64
  pub fn idle_duration_ms(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_idle_duration_ms(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

}  // impl IdleState

impl ::std::ops::Drop for IdleState {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for IdleState {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for IdleState {
  type Proxied = Self;
  fn as_view(&self) -> IdleStateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for IdleState {
  type MutProxied = Self;
  fn as_mut(&mut self) -> IdleStateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for IdleState {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__IdleState_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P,P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__IdleState_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__IdleState_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for IdleState {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for IdleState {
  type Msg = IdleState;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdleState> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IdleState {
  type Msg = IdleState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdleState> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for IdleStateMut<'_> {
  type Msg = IdleState;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdleState> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IdleStateMut<'_> {
  type Msg = IdleState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdleState> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for IdleStateView<'_> {
  type Msg = IdleState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<IdleState> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for IdleStateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__ScreenState_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ScreenState {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ScreenState>
}

impl ::protobuf::Message for ScreenState {
  type MessageView<'msg> = ScreenStateView<'msg>;
  type MessageMut<'msg> = ScreenStateMut<'msg>;
}

impl ::std::default::Default for ScreenState {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ScreenState {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ScreenState` is `Sync` because it does not implement interior mutability.
//    Neither does `ScreenStateMut`.
unsafe impl ::std::marker::Sync for ScreenState {}

// SAFETY:
// - `ScreenState` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ScreenState {}

impl ::protobuf::Proxied for ScreenState {
  type View<'msg> = ScreenStateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ScreenState {}

impl ::protobuf::MutProxied for ScreenState {
  type Mut<'msg> = ScreenStateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ScreenStateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScreenState>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScreenStateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ScreenStateView<'msg> {
  type Message = ScreenState;
}

impl ::std::fmt::Debug for ScreenStateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ScreenStateView<'_> {
  fn default() -> ScreenStateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ScreenState>> for ScreenStateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ScreenState>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScreenStateView<'msg> {

  pub fn to_owned(&self) -> ScreenState {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // power_state: optional enum lifechronicle.events.v1.ScreenPowerState
  pub fn power_state(self) -> super::ScreenPowerState {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ScreenPowerState::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // locked: optional bool
  pub fn locked(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }

  // display_id: optional string
  pub fn display_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `ScreenStateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ScreenStateView<'_> {}

// SAFETY:
// - `ScreenStateView` is `Send` because while its alive a `ScreenStateMut` cannot.
// - `ScreenStateView` does not use thread-local data.
unsafe impl ::std::marker::Send for ScreenStateView<'_> {}

impl<'msg> ::protobuf::AsView for ScreenStateView<'msg> {
  type Proxied = ScreenState;
  fn as_view(&self) -> ::protobuf::View<'msg, ScreenState> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScreenStateView<'msg> {
  fn into_view<'shorter>(self) -> ScreenStateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ScreenState> for ScreenStateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScreenState {
    let mut dst = ScreenState::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ScreenState> for ScreenStateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ScreenState {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ScreenState {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScreenStateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ScreenStateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ScreenStateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScreenState>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ScreenStateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ScreenStateMut<'msg> {
  type Message = ScreenState;
}

impl ::std::fmt::Debug for ScreenStateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ScreenState>> for ScreenStateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ScreenState>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ScreenStateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ScreenState> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ScreenState {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // power_state: optional enum lifechronicle.events.v1.ScreenPowerState
  pub fn power_state(&self) -> super::ScreenPowerState {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ScreenPowerState::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_power_state(&mut self, val: super::ScreenPowerState) {
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

  // locked: optional bool
  pub fn locked(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_locked(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // display_id: optional string
  pub fn display_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_display_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `ScreenStateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ScreenStateMut<'_> {}

// SAFETY:
// - `ScreenStateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ScreenStateMut<'_> {}

impl<'msg> ::protobuf::AsView for ScreenStateMut<'msg> {
  type Proxied = ScreenState;
  fn as_view(&self) -> ::protobuf::View<'_, ScreenState> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScreenStateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ScreenState>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ScreenStateMut<'msg> {
  type MutProxied = ScreenState;
  fn as_mut(&mut self) -> ScreenStateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ScreenStateMut<'msg> {
  fn into_mut<'shorter>(self) -> ScreenStateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ScreenState {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ScreenState> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ScreenStateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ScreenStateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // power_state: optional enum lifechronicle.events.v1.ScreenPowerState
  pub fn power_state(&self) -> super::ScreenPowerState {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::ScreenPowerState::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_power_state(&mut self, val: super::ScreenPowerState) {
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

  // locked: optional bool
  pub fn locked(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_locked(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // display_id: optional string
  pub fn display_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_display_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl ScreenState

impl ::std::ops::Drop for ScreenState {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ScreenState {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ScreenState {
  type Proxied = Self;
  fn as_view(&self) -> ScreenStateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ScreenState {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ScreenStateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ScreenState {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__ScreenState_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P/P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__ScreenState_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__ScreenState_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScreenState {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScreenState {
  type Msg = ScreenState;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScreenState> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScreenState {
  type Msg = ScreenState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScreenState> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ScreenStateMut<'_> {
  type Msg = ScreenState;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScreenState> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScreenStateMut<'_> {
  type Msg = ScreenState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScreenState> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ScreenStateView<'_> {
  type Msg = ScreenState;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ScreenState> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ScreenStateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdleStatus(i32);

#[allow(non_upper_case_globals)]
impl IdleStatus {
  pub const Unspecified: IdleStatus = IdleStatus(0);
  pub const Active: IdleStatus = IdleStatus(1);
  pub const Idle: IdleStatus = IdleStatus(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "Active",
      2 => "Idle",
      _ => return None
    })
  }
}

impl ::std::convert::From<IdleStatus> for i32 {
  fn from(val: IdleStatus) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for IdleStatus {
  fn from(val: i32) -> IdleStatus {
    Self(val)
  }
}

impl ::std::default::Default for IdleStatus {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for IdleStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "IdleStatus::{}", constant_name)
    } else {
      write!(f, "IdleStatus::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for IdleStatus {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for IdleStatus {}

impl ::protobuf::Proxied for IdleStatus {
  type View<'a> = IdleStatus;
}

impl ::protobuf::AsView for IdleStatus {
  type Proxied = IdleStatus;

  fn as_view(&self) -> IdleStatus {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IdleStatus {
  fn into_view<'shorter>(self) -> IdleStatus where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for IdleStatus {
  const NAME: &'static str = "IdleStatus";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for IdleStatus {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScreenPowerState(i32);

#[allow(non_upper_case_globals)]
impl ScreenPowerState {
  pub const Unspecified: ScreenPowerState = ScreenPowerState(0);
  pub const On: ScreenPowerState = ScreenPowerState(1);
  pub const Dimmed: ScreenPowerState = ScreenPowerState(2);
  pub const Off: ScreenPowerState = ScreenPowerState(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "On",
      2 => "Dimmed",
      3 => "Off",
      _ => return None
    })
  }
}

impl ::std::convert::From<ScreenPowerState> for i32 {
  fn from(val: ScreenPowerState) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ScreenPowerState {
  fn from(val: i32) -> ScreenPowerState {
    Self(val)
  }
}

impl ::std::default::Default for ScreenPowerState {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ScreenPowerState {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ScreenPowerState::{}", constant_name)
    } else {
      write!(f, "ScreenPowerState::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ScreenPowerState {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ScreenPowerState {}

impl ::protobuf::Proxied for ScreenPowerState {
  type View<'a> = ScreenPowerState;
}

impl ::protobuf::AsView for ScreenPowerState {
  type Proxied = ScreenPowerState;

  fn as_view(&self) -> ScreenPowerState {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScreenPowerState {
  fn into_view<'shorter>(self) -> ScreenPowerState where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ScreenPowerState {
  const NAME: &'static str = "ScreenPowerState";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for ScreenPowerState {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


