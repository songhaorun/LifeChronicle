const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__UploadBatch_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct UploadBatch {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<UploadBatch>
}

impl ::protobuf::Message for UploadBatch {
  type MessageView<'msg> = UploadBatchView<'msg>;
  type MessageMut<'msg> = UploadBatchMut<'msg>;
}

impl ::std::default::Default for UploadBatch {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for UploadBatch {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `UploadBatch` is `Sync` because it does not implement interior mutability.
//    Neither does `UploadBatchMut`.
unsafe impl ::std::marker::Sync for UploadBatch {}

// SAFETY:
// - `UploadBatch` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for UploadBatch {}

impl ::protobuf::Proxied for UploadBatch {
  type View<'msg> = UploadBatchView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for UploadBatch {}

impl ::protobuf::MutProxied for UploadBatch {
  type Mut<'msg> = UploadBatchMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UploadBatchView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UploadBatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UploadBatchView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UploadBatchView<'msg> {
  type Message = UploadBatch;
}

impl ::std::fmt::Debug for UploadBatchView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UploadBatchView<'_> {
  fn default() -> UploadBatchView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, UploadBatch>> for UploadBatchView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, UploadBatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UploadBatchView<'msg> {

  pub fn to_owned(&self) -> UploadBatch {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // batch_id: optional string
  pub fn batch_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // device_id: optional string
  pub fn device_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // collector_instance_id: optional string
  pub fn collector_instance_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // sequence_start: optional uint64
  pub fn sequence_start(self) -> u64 {
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

  // sequence_end: optional uint64
  pub fn sequence_end(self) -> u64 {
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

  // created_at: optional message google.protobuf.Timestamp
  pub fn has_created_at(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn created_at_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_created_at().then(|| self.created_at())
  }
  pub fn created_at(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // nonce: optional bytes
  pub fn nonce(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
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
        7, (super::Compression::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // compressed_items: optional bytes
  pub fn compressed_items(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // payload_sha256: optional bytes
  pub fn payload_sha256(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // signature: optional bytes
  pub fn signature(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
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

}

// SAFETY:
// - `UploadBatchView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UploadBatchView<'_> {}

// SAFETY:
// - `UploadBatchView` is `Send` because while its alive a `UploadBatchMut` cannot.
// - `UploadBatchView` does not use thread-local data.
unsafe impl ::std::marker::Send for UploadBatchView<'_> {}

impl<'msg> ::protobuf::AsView for UploadBatchView<'msg> {
  type Proxied = UploadBatch;
  fn as_view(&self) -> ::protobuf::View<'msg, UploadBatch> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UploadBatchView<'msg> {
  fn into_view<'shorter>(self) -> UploadBatchView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<UploadBatch> for UploadBatchView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UploadBatch {
    let mut dst = UploadBatch::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<UploadBatch> for UploadBatchMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> UploadBatch {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for UploadBatch {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UploadBatchView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UploadBatchMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UploadBatchMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UploadBatch>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UploadBatchMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UploadBatchMut<'msg> {
  type Message = UploadBatch;
}

impl ::std::fmt::Debug for UploadBatchMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, UploadBatch>> for UploadBatchMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, UploadBatch>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UploadBatchMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, UploadBatch> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> UploadBatch {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // batch_id: optional string
  pub fn batch_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_batch_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // collector_instance_id: optional string
  pub fn collector_instance_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_instance_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // sequence_start: optional uint64
  pub fn sequence_start(&self) -> u64 {
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
  pub fn set_sequence_start(&mut self, val: u64) {
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

  // sequence_end: optional uint64
  pub fn sequence_end(&self) -> u64 {
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
  pub fn set_sequence_end(&mut self, val: u64) {
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

  // created_at: optional message google.protobuf.Timestamp
  pub fn has_created_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_created_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn created_at_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_created_at().then(|| self.created_at())
  }
  pub fn created_at(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn created_at_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_created_at(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // nonce: optional bytes
  pub fn nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
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
        7, (super::Compression::Unspecified).into()
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
        7, val.into()
      )
    }
  }

  // compressed_items: optional bytes
  pub fn compressed_items(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_compressed_items(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // payload_sha256: optional bytes
  pub fn payload_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_payload_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // signature: optional bytes
  pub fn signature(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_signature(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
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

}

// SAFETY:
// - `UploadBatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UploadBatchMut<'_> {}

// SAFETY:
// - `UploadBatchMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UploadBatchMut<'_> {}

impl<'msg> ::protobuf::AsView for UploadBatchMut<'msg> {
  type Proxied = UploadBatch;
  fn as_view(&self) -> ::protobuf::View<'_, UploadBatch> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UploadBatchMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, UploadBatch>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UploadBatchMut<'msg> {
  type MutProxied = UploadBatch;
  fn as_mut(&mut self) -> UploadBatchMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UploadBatchMut<'msg> {
  fn into_mut<'shorter>(self) -> UploadBatchMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl UploadBatch {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, UploadBatch> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UploadBatchView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UploadBatchMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // batch_id: optional string
  pub fn batch_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_batch_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // device_id: optional string
  pub fn device_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_device_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // collector_instance_id: optional string
  pub fn collector_instance_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_collector_instance_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // sequence_start: optional uint64
  pub fn sequence_start(&self) -> u64 {
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
  pub fn set_sequence_start(&mut self, val: u64) {
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

  // sequence_end: optional uint64
  pub fn sequence_end(&self) -> u64 {
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
  pub fn set_sequence_end(&mut self, val: u64) {
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

  // created_at: optional message google.protobuf.Timestamp
  pub fn has_created_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_created_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn created_at_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_created_at().then(|| self.created_at())
  }
  pub fn created_at(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn created_at_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_created_at(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // nonce: optional bytes
  pub fn nonce(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_nonce(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
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
        7, (super::Compression::Unspecified).into()
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
        7, val.into()
      )
    }
  }

  // compressed_items: optional bytes
  pub fn compressed_items(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_compressed_items(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // payload_sha256: optional bytes
  pub fn payload_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_payload_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // signature: optional bytes
  pub fn signature(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_signature(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
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

}  // impl UploadBatch

impl ::std::ops::Drop for UploadBatch {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for UploadBatch {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for UploadBatch {
  type Proxied = Self;
  fn as_view(&self) -> UploadBatchView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for UploadBatch {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UploadBatchMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for UploadBatch {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__UploadBatch_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X1X,P,P30P.P0P0P0P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__UploadBatch_msg_init.0, &[<::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__UploadBatch_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UploadBatch {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UploadBatch {
  type Msg = UploadBatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UploadBatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UploadBatch {
  type Msg = UploadBatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UploadBatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UploadBatchMut<'_> {
  type Msg = UploadBatch;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UploadBatch> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UploadBatchMut<'_> {
  type Msg = UploadBatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UploadBatch> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UploadBatchView<'_> {
  type Msg = UploadBatch;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<UploadBatch> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UploadBatchMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__BatchItems_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BatchItems {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BatchItems>
}

impl ::protobuf::Message for BatchItems {
  type MessageView<'msg> = BatchItemsView<'msg>;
  type MessageMut<'msg> = BatchItemsMut<'msg>;
}

impl ::std::default::Default for BatchItems {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BatchItems {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BatchItems` is `Sync` because it does not implement interior mutability.
//    Neither does `BatchItemsMut`.
unsafe impl ::std::marker::Sync for BatchItems {}

// SAFETY:
// - `BatchItems` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BatchItems {}

impl ::protobuf::Proxied for BatchItems {
  type View<'msg> = BatchItemsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BatchItems {}

impl ::protobuf::MutProxied for BatchItems {
  type Mut<'msg> = BatchItemsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BatchItemsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BatchItems>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BatchItemsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BatchItemsView<'msg> {
  type Message = BatchItems;
}

impl ::std::fmt::Debug for BatchItemsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BatchItemsView<'_> {
  fn default() -> BatchItemsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BatchItems>> for BatchItemsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BatchItems>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BatchItemsView<'msg> {

  pub fn to_owned(&self) -> BatchItems {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // events: repeated message lifechronicle.events.v1.EventEnvelope
  pub fn events(self) -> ::protobuf::RepeatedView<'msg, super::EventEnvelope> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EventEnvelope>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // series_chunks: repeated message lifechronicle.events.v1.SeriesChunk
  pub fn series_chunks(self) -> ::protobuf::RepeatedView<'msg, super::SeriesChunk> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::SeriesChunk>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `BatchItemsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BatchItemsView<'_> {}

// SAFETY:
// - `BatchItemsView` is `Send` because while its alive a `BatchItemsMut` cannot.
// - `BatchItemsView` does not use thread-local data.
unsafe impl ::std::marker::Send for BatchItemsView<'_> {}

impl<'msg> ::protobuf::AsView for BatchItemsView<'msg> {
  type Proxied = BatchItems;
  fn as_view(&self) -> ::protobuf::View<'msg, BatchItems> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BatchItemsView<'msg> {
  fn into_view<'shorter>(self) -> BatchItemsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BatchItems> for BatchItemsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BatchItems {
    let mut dst = BatchItems::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BatchItems> for BatchItemsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BatchItems {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BatchItems {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BatchItemsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BatchItemsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BatchItemsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BatchItems>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BatchItemsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BatchItemsMut<'msg> {
  type Message = BatchItems;
}

impl ::std::fmt::Debug for BatchItemsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BatchItems>> for BatchItemsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BatchItems>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BatchItemsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BatchItems> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BatchItems {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // events: repeated message lifechronicle.events.v1.EventEnvelope
  pub fn events(&self) -> ::protobuf::RepeatedView<'_, super::EventEnvelope> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EventEnvelope>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn events_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::EventEnvelope> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_events(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::EventEnvelope>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // series_chunks: repeated message lifechronicle.events.v1.SeriesChunk
  pub fn series_chunks(&self) -> ::protobuf::RepeatedView<'_, super::SeriesChunk> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::SeriesChunk>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn series_chunks_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::SeriesChunk> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
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
  pub fn set_series_chunks(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::SeriesChunk>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `BatchItemsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BatchItemsMut<'_> {}

// SAFETY:
// - `BatchItemsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BatchItemsMut<'_> {}

impl<'msg> ::protobuf::AsView for BatchItemsMut<'msg> {
  type Proxied = BatchItems;
  fn as_view(&self) -> ::protobuf::View<'_, BatchItems> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BatchItemsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BatchItems>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BatchItemsMut<'msg> {
  type MutProxied = BatchItems;
  fn as_mut(&mut self) -> BatchItemsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BatchItemsMut<'msg> {
  fn into_mut<'shorter>(self) -> BatchItemsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BatchItems {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BatchItems> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BatchItemsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BatchItemsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // events: repeated message lifechronicle.events.v1.EventEnvelope
  pub fn events(&self) -> ::protobuf::RepeatedView<'_, super::EventEnvelope> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EventEnvelope>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn events_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::EventEnvelope> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
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
  pub fn set_events(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::EventEnvelope>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // series_chunks: repeated message lifechronicle.events.v1.SeriesChunk
  pub fn series_chunks(&self) -> ::protobuf::RepeatedView<'_, super::SeriesChunk> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::SeriesChunk>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn series_chunks_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::SeriesChunk> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
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
  pub fn set_series_chunks(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::SeriesChunk>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl BatchItems

impl ::std::ops::Drop for BatchItems {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BatchItems {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BatchItems {
  type Proxied = Self;
  fn as_view(&self) -> BatchItemsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BatchItems {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BatchItemsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BatchItems {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__BatchItems_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$GG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__BatchItems_msg_init.0, &[<super::EventEnvelope as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::SeriesChunk as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__BatchItems_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BatchItems {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BatchItems {
  type Msg = BatchItems;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BatchItems> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BatchItems {
  type Msg = BatchItems;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BatchItems> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BatchItemsMut<'_> {
  type Msg = BatchItems;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BatchItems> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BatchItemsMut<'_> {
  type Msg = BatchItems;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BatchItems> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BatchItemsView<'_> {
  type Msg = BatchItems;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BatchItems> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BatchItemsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__ItemAcknowledgement_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ItemAcknowledgement {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ItemAcknowledgement>
}

impl ::protobuf::Message for ItemAcknowledgement {
  type MessageView<'msg> = ItemAcknowledgementView<'msg>;
  type MessageMut<'msg> = ItemAcknowledgementMut<'msg>;
}

impl ::std::default::Default for ItemAcknowledgement {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ItemAcknowledgement {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ItemAcknowledgement` is `Sync` because it does not implement interior mutability.
//    Neither does `ItemAcknowledgementMut`.
unsafe impl ::std::marker::Sync for ItemAcknowledgement {}

// SAFETY:
// - `ItemAcknowledgement` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ItemAcknowledgement {}

impl ::protobuf::Proxied for ItemAcknowledgement {
  type View<'msg> = ItemAcknowledgementView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ItemAcknowledgement {}

impl ::protobuf::MutProxied for ItemAcknowledgement {
  type Mut<'msg> = ItemAcknowledgementMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ItemAcknowledgementView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ItemAcknowledgement>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ItemAcknowledgementView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ItemAcknowledgementView<'msg> {
  type Message = ItemAcknowledgement;
}

impl ::std::fmt::Debug for ItemAcknowledgementView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ItemAcknowledgementView<'_> {
  fn default() -> ItemAcknowledgementView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ItemAcknowledgement>> for ItemAcknowledgementView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ItemAcknowledgement>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ItemAcknowledgementView<'msg> {

  pub fn to_owned(&self) -> ItemAcknowledgement {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // item_id: optional string
  pub fn item_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // status: optional enum lifechronicle.events.v1.ItemStatus
  pub fn status(self) -> super::ItemStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::ItemStatus::Unspecified).into()
      ).try_into().unwrap()
    }
  }

  // error_code: optional string
  pub fn error_code(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // error_detail_id: optional string
  pub fn error_detail_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `ItemAcknowledgementView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ItemAcknowledgementView<'_> {}

// SAFETY:
// - `ItemAcknowledgementView` is `Send` because while its alive a `ItemAcknowledgementMut` cannot.
// - `ItemAcknowledgementView` does not use thread-local data.
unsafe impl ::std::marker::Send for ItemAcknowledgementView<'_> {}

impl<'msg> ::protobuf::AsView for ItemAcknowledgementView<'msg> {
  type Proxied = ItemAcknowledgement;
  fn as_view(&self) -> ::protobuf::View<'msg, ItemAcknowledgement> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ItemAcknowledgementView<'msg> {
  fn into_view<'shorter>(self) -> ItemAcknowledgementView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ItemAcknowledgement> for ItemAcknowledgementView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ItemAcknowledgement {
    let mut dst = ItemAcknowledgement::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ItemAcknowledgement> for ItemAcknowledgementMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ItemAcknowledgement {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ItemAcknowledgement {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ItemAcknowledgementView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ItemAcknowledgementMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ItemAcknowledgementMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ItemAcknowledgement>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ItemAcknowledgementMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ItemAcknowledgementMut<'msg> {
  type Message = ItemAcknowledgement;
}

impl ::std::fmt::Debug for ItemAcknowledgementMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ItemAcknowledgement>> for ItemAcknowledgementMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ItemAcknowledgement>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ItemAcknowledgementMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ItemAcknowledgement> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ItemAcknowledgement {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // item_id: optional string
  pub fn item_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_item_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // status: optional enum lifechronicle.events.v1.ItemStatus
  pub fn status(&self) -> super::ItemStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::ItemStatus::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_status(&mut self, val: super::ItemStatus) {
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

  // error_code: optional string
  pub fn error_code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_error_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // error_detail_id: optional string
  pub fn error_detail_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_error_detail_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}

// SAFETY:
// - `ItemAcknowledgementMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ItemAcknowledgementMut<'_> {}

// SAFETY:
// - `ItemAcknowledgementMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ItemAcknowledgementMut<'_> {}

impl<'msg> ::protobuf::AsView for ItemAcknowledgementMut<'msg> {
  type Proxied = ItemAcknowledgement;
  fn as_view(&self) -> ::protobuf::View<'_, ItemAcknowledgement> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ItemAcknowledgementMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ItemAcknowledgement>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ItemAcknowledgementMut<'msg> {
  type MutProxied = ItemAcknowledgement;
  fn as_mut(&mut self) -> ItemAcknowledgementMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ItemAcknowledgementMut<'msg> {
  fn into_mut<'shorter>(self) -> ItemAcknowledgementMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ItemAcknowledgement {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ItemAcknowledgement> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ItemAcknowledgementView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ItemAcknowledgementMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // item_id: optional string
  pub fn item_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_item_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // status: optional enum lifechronicle.events.v1.ItemStatus
  pub fn status(&self) -> super::ItemStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::ItemStatus::Unspecified).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_status(&mut self, val: super::ItemStatus) {
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

  // error_code: optional string
  pub fn error_code(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_error_code(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // error_detail_id: optional string
  pub fn error_detail_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_error_detail_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}  // impl ItemAcknowledgement

impl ::std::ops::Drop for ItemAcknowledgement {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ItemAcknowledgement {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ItemAcknowledgement {
  type Proxied = Self;
  fn as_view(&self) -> ItemAcknowledgementView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ItemAcknowledgement {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ItemAcknowledgementMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ItemAcknowledgement {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__ItemAcknowledgement_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X.P1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__ItemAcknowledgement_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__ItemAcknowledgement_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ItemAcknowledgement {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ItemAcknowledgement {
  type Msg = ItemAcknowledgement;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ItemAcknowledgement> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ItemAcknowledgement {
  type Msg = ItemAcknowledgement;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ItemAcknowledgement> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ItemAcknowledgementMut<'_> {
  type Msg = ItemAcknowledgement;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ItemAcknowledgement> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ItemAcknowledgementMut<'_> {
  type Msg = ItemAcknowledgement;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ItemAcknowledgement> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ItemAcknowledgementView<'_> {
  type Msg = ItemAcknowledgement;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ItemAcknowledgement> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ItemAcknowledgementMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__BatchAcknowledgement_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct BatchAcknowledgement {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<BatchAcknowledgement>
}

impl ::protobuf::Message for BatchAcknowledgement {
  type MessageView<'msg> = BatchAcknowledgementView<'msg>;
  type MessageMut<'msg> = BatchAcknowledgementMut<'msg>;
}

impl ::std::default::Default for BatchAcknowledgement {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for BatchAcknowledgement {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `BatchAcknowledgement` is `Sync` because it does not implement interior mutability.
//    Neither does `BatchAcknowledgementMut`.
unsafe impl ::std::marker::Sync for BatchAcknowledgement {}

// SAFETY:
// - `BatchAcknowledgement` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for BatchAcknowledgement {}

impl ::protobuf::Proxied for BatchAcknowledgement {
  type View<'msg> = BatchAcknowledgementView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for BatchAcknowledgement {}

impl ::protobuf::MutProxied for BatchAcknowledgement {
  type Mut<'msg> = BatchAcknowledgementMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BatchAcknowledgementView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BatchAcknowledgement>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BatchAcknowledgementView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BatchAcknowledgementView<'msg> {
  type Message = BatchAcknowledgement;
}

impl ::std::fmt::Debug for BatchAcknowledgementView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BatchAcknowledgementView<'_> {
  fn default() -> BatchAcknowledgementView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, BatchAcknowledgement>> for BatchAcknowledgementView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, BatchAcknowledgement>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BatchAcknowledgementView<'msg> {

  pub fn to_owned(&self) -> BatchAcknowledgement {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // batch_id: optional string
  pub fn batch_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // items: repeated message lifechronicle.events.v1.ItemAcknowledgement
  pub fn items(self) -> ::protobuf::RepeatedView<'msg, super::ItemAcknowledgement> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ItemAcknowledgement>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // acknowledged_at: optional message google.protobuf.Timestamp
  pub fn has_acknowledged_at(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn acknowledged_at_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_acknowledged_at().then(|| self.acknowledged_at())
  }
  pub fn acknowledged_at(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

}

// SAFETY:
// - `BatchAcknowledgementView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BatchAcknowledgementView<'_> {}

// SAFETY:
// - `BatchAcknowledgementView` is `Send` because while its alive a `BatchAcknowledgementMut` cannot.
// - `BatchAcknowledgementView` does not use thread-local data.
unsafe impl ::std::marker::Send for BatchAcknowledgementView<'_> {}

impl<'msg> ::protobuf::AsView for BatchAcknowledgementView<'msg> {
  type Proxied = BatchAcknowledgement;
  fn as_view(&self) -> ::protobuf::View<'msg, BatchAcknowledgement> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BatchAcknowledgementView<'msg> {
  fn into_view<'shorter>(self) -> BatchAcknowledgementView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<BatchAcknowledgement> for BatchAcknowledgementView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BatchAcknowledgement {
    let mut dst = BatchAcknowledgement::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<BatchAcknowledgement> for BatchAcknowledgementMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> BatchAcknowledgement {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for BatchAcknowledgement {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BatchAcknowledgementView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BatchAcknowledgementMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BatchAcknowledgementMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BatchAcknowledgement>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BatchAcknowledgementMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BatchAcknowledgementMut<'msg> {
  type Message = BatchAcknowledgement;
}

impl ::std::fmt::Debug for BatchAcknowledgementMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, BatchAcknowledgement>> for BatchAcknowledgementMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, BatchAcknowledgement>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BatchAcknowledgementMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, BatchAcknowledgement> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> BatchAcknowledgement {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // batch_id: optional string
  pub fn batch_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_batch_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // items: repeated message lifechronicle.events.v1.ItemAcknowledgement
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::ItemAcknowledgement> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ItemAcknowledgement>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ItemAcknowledgement> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ItemAcknowledgement>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // acknowledged_at: optional message google.protobuf.Timestamp
  pub fn has_acknowledged_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_acknowledged_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn acknowledged_at_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_acknowledged_at().then(|| self.acknowledged_at())
  }
  pub fn acknowledged_at(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn acknowledged_at_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_acknowledged_at(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}

// SAFETY:
// - `BatchAcknowledgementMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BatchAcknowledgementMut<'_> {}

// SAFETY:
// - `BatchAcknowledgementMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BatchAcknowledgementMut<'_> {}

impl<'msg> ::protobuf::AsView for BatchAcknowledgementMut<'msg> {
  type Proxied = BatchAcknowledgement;
  fn as_view(&self) -> ::protobuf::View<'_, BatchAcknowledgement> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BatchAcknowledgementMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, BatchAcknowledgement>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BatchAcknowledgementMut<'msg> {
  type MutProxied = BatchAcknowledgement;
  fn as_mut(&mut self) -> BatchAcknowledgementMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BatchAcknowledgementMut<'msg> {
  fn into_mut<'shorter>(self) -> BatchAcknowledgementMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl BatchAcknowledgement {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, BatchAcknowledgement> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BatchAcknowledgementView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BatchAcknowledgementMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // batch_id: optional string
  pub fn batch_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_batch_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // items: repeated message lifechronicle.events.v1.ItemAcknowledgement
  pub fn items(&self) -> ::protobuf::RepeatedView<'_, super::ItemAcknowledgement> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::ItemAcknowledgement>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::ItemAcknowledgement> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
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
  pub fn set_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::ItemAcknowledgement>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // acknowledged_at: optional message google.protobuf.Timestamp
  pub fn has_acknowledged_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_acknowledged_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn acknowledged_at_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_acknowledged_at().then(|| self.acknowledged_at())
  }
  pub fn acknowledged_at(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn acknowledged_at_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_acknowledged_at(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl BatchAcknowledgement

impl ::std::ops::Drop for BatchAcknowledgement {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for BatchAcknowledgement {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for BatchAcknowledgement {
  type Proxied = Self;
  fn as_view(&self) -> BatchAcknowledgementView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for BatchAcknowledgement {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BatchAcknowledgementMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for BatchAcknowledgement {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__BatchAcknowledgement_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1XG3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__BatchAcknowledgement_msg_init.0, &[<super::ItemAcknowledgement as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__BatchAcknowledgement_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BatchAcknowledgement {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BatchAcknowledgement {
  type Msg = BatchAcknowledgement;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BatchAcknowledgement> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BatchAcknowledgement {
  type Msg = BatchAcknowledgement;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BatchAcknowledgement> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BatchAcknowledgementMut<'_> {
  type Msg = BatchAcknowledgement;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BatchAcknowledgement> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BatchAcknowledgementMut<'_> {
  type Msg = BatchAcknowledgement;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BatchAcknowledgement> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BatchAcknowledgementView<'_> {
  type Msg = BatchAcknowledgement;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<BatchAcknowledgement> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BatchAcknowledgementMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ItemStatus(i32);

#[allow(non_upper_case_globals)]
impl ItemStatus {
  pub const Unspecified: ItemStatus = ItemStatus(0);
  pub const AcceptedToLog: ItemStatus = ItemStatus(1);
  pub const Duplicate: ItemStatus = ItemStatus(2);
  pub const RejectedPermanent: ItemStatus = ItemStatus(3);
  pub const Retryable: ItemStatus = ItemStatus(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "AcceptedToLog",
      2 => "Duplicate",
      3 => "RejectedPermanent",
      4 => "Retryable",
      _ => return None
    })
  }
}

impl ::std::convert::From<ItemStatus> for i32 {
  fn from(val: ItemStatus) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for ItemStatus {
  fn from(val: i32) -> ItemStatus {
    Self(val)
  }
}

impl ::std::default::Default for ItemStatus {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ItemStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ItemStatus::{}", constant_name)
    } else {
      write!(f, "ItemStatus::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ItemStatus {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ItemStatus {}

impl ::protobuf::Proxied for ItemStatus {
  type View<'a> = ItemStatus;
}

impl ::protobuf::AsView for ItemStatus {
  type Proxied = ItemStatus;

  fn as_view(&self) -> ItemStatus {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ItemStatus {
  fn into_view<'shorter>(self) -> ItemStatus where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ItemStatus {
  const NAME: &'static str = "ItemStatus";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for ItemStatus {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IngestionErrorCode(i32);

#[allow(non_upper_case_globals)]
impl IngestionErrorCode {
  pub const Unspecified: IngestionErrorCode = IngestionErrorCode(0);
  pub const AuthInvalid: IngestionErrorCode = IngestionErrorCode(1);
  pub const DeviceRevoked: IngestionErrorCode = IngestionErrorCode(2);
  pub const SignatureInvalid: IngestionErrorCode = IngestionErrorCode(3);
  pub const NonceReplayed: IngestionErrorCode = IngestionErrorCode(4);
  pub const PayloadHashMismatch: IngestionErrorCode = IngestionErrorCode(5);
  pub const ProtoDecodeFailed: IngestionErrorCode = IngestionErrorCode(6);
  pub const StreamUnknown: IngestionErrorCode = IngestionErrorCode(7);
  pub const SchemaVersionUnsupported: IngestionErrorCode = IngestionErrorCode(8);
  pub const SchemaValidationFailed: IngestionErrorCode = IngestionErrorCode(9);
  pub const IdContentConflict: IngestionErrorCode = IngestionErrorCode(10);
  pub const BatchLimitExceeded: IngestionErrorCode = IngestionErrorCode(11);
  pub const RateLimited: IngestionErrorCode = IngestionErrorCode(12);
  pub const KafkaUnavailable: IngestionErrorCode = IngestionErrorCode(13);
  pub const ObjectStorageUnavailable: IngestionErrorCode = IngestionErrorCode(14);
  pub const InternalRetryable: IngestionErrorCode = IngestionErrorCode(15);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unspecified",
      1 => "AuthInvalid",
      2 => "DeviceRevoked",
      3 => "SignatureInvalid",
      4 => "NonceReplayed",
      5 => "PayloadHashMismatch",
      6 => "ProtoDecodeFailed",
      7 => "StreamUnknown",
      8 => "SchemaVersionUnsupported",
      9 => "SchemaValidationFailed",
      10 => "IdContentConflict",
      11 => "BatchLimitExceeded",
      12 => "RateLimited",
      13 => "KafkaUnavailable",
      14 => "ObjectStorageUnavailable",
      15 => "InternalRetryable",
      _ => return None
    })
  }
}

impl ::std::convert::From<IngestionErrorCode> for i32 {
  fn from(val: IngestionErrorCode) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for IngestionErrorCode {
  fn from(val: i32) -> IngestionErrorCode {
    Self(val)
  }
}

impl ::std::default::Default for IngestionErrorCode {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for IngestionErrorCode {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "IngestionErrorCode::{}", constant_name)
    } else {
      write!(f, "IngestionErrorCode::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for IngestionErrorCode {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for IngestionErrorCode {}

impl ::protobuf::Proxied for IngestionErrorCode {
  type View<'a> = IngestionErrorCode;
}

impl ::protobuf::AsView for IngestionErrorCode {
  type Proxied = IngestionErrorCode;

  fn as_view(&self) -> IngestionErrorCode {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for IngestionErrorCode {
  fn into_view<'shorter>(self) -> IngestionErrorCode where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for IngestionErrorCode {
  const NAME: &'static str = "IngestionErrorCode";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6|7|8|9|10|11|12|13|14|15)
  }
}

impl ::protobuf::__internal::EntityType for IngestionErrorCode {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


