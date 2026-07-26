const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__RawEventRecord_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RawEventRecord {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RawEventRecord>
}

impl ::protobuf::Message for RawEventRecord {
  type MessageView<'msg> = RawEventRecordView<'msg>;
  type MessageMut<'msg> = RawEventRecordMut<'msg>;
}

impl ::std::default::Default for RawEventRecord {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RawEventRecord {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RawEventRecord` is `Sync` because it does not implement interior mutability.
//    Neither does `RawEventRecordMut`.
unsafe impl ::std::marker::Sync for RawEventRecord {}

// SAFETY:
// - `RawEventRecord` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RawEventRecord {}

impl ::protobuf::Proxied for RawEventRecord {
  type View<'msg> = RawEventRecordView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RawEventRecord {}

impl ::protobuf::MutProxied for RawEventRecord {
  type Mut<'msg> = RawEventRecordMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RawEventRecordView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RawEventRecord>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RawEventRecordView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RawEventRecordView<'msg> {
  type Message = RawEventRecord;
}

impl ::std::fmt::Debug for RawEventRecordView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RawEventRecordView<'_> {
  fn default() -> RawEventRecordView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RawEventRecord>> for RawEventRecordView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RawEventRecord>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RawEventRecordView<'msg> {

  pub fn to_owned(&self) -> RawEventRecord {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // event: optional message lifechronicle.events.v1.EventEnvelope
  pub fn has_event(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn event_opt(self) -> ::std::option::Option<super::EventEnvelopeView<'msg>> {
    self.has_event().then(|| self.event())
  }
  pub fn event(self) -> super::EventEnvelopeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EventEnvelopeView::default())
  }

  // batch_id: optional string
  pub fn batch_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // received_at: optional message google.protobuf.Timestamp
  pub fn has_received_at(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn received_at_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_received_at().then(|| self.received_at())
  }
  pub fn received_at(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // ingested_at: optional message google.protobuf.Timestamp
  pub fn has_ingested_at(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn ingested_at_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_ingested_at().then(|| self.ingested_at())
  }
  pub fn ingested_at(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // submitted_sha256: optional bytes
  pub fn submitted_sha256(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // canonical_sha256: optional bytes
  pub fn canonical_sha256(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // authenticated_principal: optional string
  pub fn authenticated_principal(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // quality_signals: repeated message lifechronicle.events.v1.QualitySignal
  pub fn quality_signals(self) -> ::protobuf::RepeatedView<'msg, super::QualitySignal> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::QualitySignal>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `RawEventRecordView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RawEventRecordView<'_> {}

// SAFETY:
// - `RawEventRecordView` is `Send` because while its alive a `RawEventRecordMut` cannot.
// - `RawEventRecordView` does not use thread-local data.
unsafe impl ::std::marker::Send for RawEventRecordView<'_> {}

impl<'msg> ::protobuf::AsView for RawEventRecordView<'msg> {
  type Proxied = RawEventRecord;
  fn as_view(&self) -> ::protobuf::View<'msg, RawEventRecord> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RawEventRecordView<'msg> {
  fn into_view<'shorter>(self) -> RawEventRecordView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RawEventRecord> for RawEventRecordView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RawEventRecord {
    let mut dst = RawEventRecord::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RawEventRecord> for RawEventRecordMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RawEventRecord {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RawEventRecord {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RawEventRecordView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RawEventRecordMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RawEventRecordMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RawEventRecord>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RawEventRecordMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RawEventRecordMut<'msg> {
  type Message = RawEventRecord;
}

impl ::std::fmt::Debug for RawEventRecordMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RawEventRecord>> for RawEventRecordMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RawEventRecord>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RawEventRecordMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RawEventRecord> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RawEventRecord {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // event: optional message lifechronicle.events.v1.EventEnvelope
  pub fn has_event(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_event(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn event_opt(&self) -> ::std::option::Option<super::EventEnvelopeView<'_>> {
    self.has_event().then(|| self.event())
  }
  pub fn event(&self) -> super::EventEnvelopeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EventEnvelopeView::default())
  }
  pub fn event_mut(&mut self) -> super::EventEnvelopeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_event(&mut self,
    val: impl ::protobuf::IntoProxied<super::EventEnvelope>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // batch_id: optional string
  pub fn batch_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_batch_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // received_at: optional message google.protobuf.Timestamp
  pub fn has_received_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_received_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn received_at_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_received_at().then(|| self.received_at())
  }
  pub fn received_at(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn received_at_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_received_at(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // ingested_at: optional message google.protobuf.Timestamp
  pub fn has_ingested_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_ingested_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn ingested_at_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_ingested_at().then(|| self.ingested_at())
  }
  pub fn ingested_at(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn ingested_at_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_ingested_at(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // submitted_sha256: optional bytes
  pub fn submitted_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_submitted_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // canonical_sha256: optional bytes
  pub fn canonical_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_canonical_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // authenticated_principal: optional string
  pub fn authenticated_principal(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authenticated_principal(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // quality_signals: repeated message lifechronicle.events.v1.QualitySignal
  pub fn quality_signals(&self) -> ::protobuf::RepeatedView<'_, super::QualitySignal> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::QualitySignal>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn quality_signals_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::QualitySignal> {
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
  pub fn set_quality_signals(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::QualitySignal>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

}

// SAFETY:
// - `RawEventRecordMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RawEventRecordMut<'_> {}

// SAFETY:
// - `RawEventRecordMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RawEventRecordMut<'_> {}

impl<'msg> ::protobuf::AsView for RawEventRecordMut<'msg> {
  type Proxied = RawEventRecord;
  fn as_view(&self) -> ::protobuf::View<'_, RawEventRecord> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RawEventRecordMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RawEventRecord>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RawEventRecordMut<'msg> {
  type MutProxied = RawEventRecord;
  fn as_mut(&mut self) -> RawEventRecordMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RawEventRecordMut<'msg> {
  fn into_mut<'shorter>(self) -> RawEventRecordMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RawEventRecord {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RawEventRecord> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RawEventRecordView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RawEventRecordMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // event: optional message lifechronicle.events.v1.EventEnvelope
  pub fn has_event(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_event(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn event_opt(&self) -> ::std::option::Option<super::EventEnvelopeView<'_>> {
    self.has_event().then(|| self.event())
  }
  pub fn event(&self) -> super::EventEnvelopeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EventEnvelopeView::default())
  }
  pub fn event_mut(&mut self) -> super::EventEnvelopeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_event(&mut self,
    val: impl ::protobuf::IntoProxied<super::EventEnvelope>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // batch_id: optional string
  pub fn batch_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_batch_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // received_at: optional message google.protobuf.Timestamp
  pub fn has_received_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_received_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn received_at_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_received_at().then(|| self.received_at())
  }
  pub fn received_at(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn received_at_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
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
  pub fn set_received_at(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // ingested_at: optional message google.protobuf.Timestamp
  pub fn has_ingested_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_ingested_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn ingested_at_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_ingested_at().then(|| self.ingested_at())
  }
  pub fn ingested_at(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn ingested_at_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_ingested_at(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // submitted_sha256: optional bytes
  pub fn submitted_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_submitted_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // canonical_sha256: optional bytes
  pub fn canonical_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_canonical_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

  // authenticated_principal: optional string
  pub fn authenticated_principal(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authenticated_principal(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // quality_signals: repeated message lifechronicle.events.v1.QualitySignal
  pub fn quality_signals(&self) -> ::protobuf::RepeatedView<'_, super::QualitySignal> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::QualitySignal>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn quality_signals_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::QualitySignal> {
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
  pub fn set_quality_signals(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::QualitySignal>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

}  // impl RawEventRecord

impl ::std::ops::Drop for RawEventRecord {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RawEventRecord {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RawEventRecord {
  type Proxied = Self;
  fn as_view(&self) -> RawEventRecordView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RawEventRecord {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RawEventRecordMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RawEventRecord {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__RawEventRecord_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X330P0P1XG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__RawEventRecord_msg_init.0, &[<super::EventEnvelope as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::QualitySignal as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__RawEventRecord_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RawEventRecord {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RawEventRecord {
  type Msg = RawEventRecord;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RawEventRecord> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RawEventRecord {
  type Msg = RawEventRecord;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RawEventRecord> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RawEventRecordMut<'_> {
  type Msg = RawEventRecord;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RawEventRecord> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RawEventRecordMut<'_> {
  type Msg = RawEventRecord;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RawEventRecord> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RawEventRecordView<'_> {
  type Msg = RawEventRecord;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RawEventRecord> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RawEventRecordMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut lifechronicle__events__v1__RawSeriesRecord_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RawSeriesRecord {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RawSeriesRecord>
}

impl ::protobuf::Message for RawSeriesRecord {
  type MessageView<'msg> = RawSeriesRecordView<'msg>;
  type MessageMut<'msg> = RawSeriesRecordMut<'msg>;
}

impl ::std::default::Default for RawSeriesRecord {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RawSeriesRecord {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RawSeriesRecord` is `Sync` because it does not implement interior mutability.
//    Neither does `RawSeriesRecordMut`.
unsafe impl ::std::marker::Sync for RawSeriesRecord {}

// SAFETY:
// - `RawSeriesRecord` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RawSeriesRecord {}

impl ::protobuf::Proxied for RawSeriesRecord {
  type View<'msg> = RawSeriesRecordView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RawSeriesRecord {}

impl ::protobuf::MutProxied for RawSeriesRecord {
  type Mut<'msg> = RawSeriesRecordMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RawSeriesRecordView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RawSeriesRecord>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RawSeriesRecordView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RawSeriesRecordView<'msg> {
  type Message = RawSeriesRecord;
}

impl ::std::fmt::Debug for RawSeriesRecordView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RawSeriesRecordView<'_> {
  fn default() -> RawSeriesRecordView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RawSeriesRecord>> for RawSeriesRecordView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RawSeriesRecord>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RawSeriesRecordView<'msg> {

  pub fn to_owned(&self) -> RawSeriesRecord {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // chunk: optional message lifechronicle.events.v1.SeriesChunkMetadata
  pub fn has_chunk(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn chunk_opt(self) -> ::std::option::Option<super::SeriesChunkMetadataView<'msg>> {
    self.has_chunk().then(|| self.chunk())
  }
  pub fn chunk(self) -> super::SeriesChunkMetadataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SeriesChunkMetadataView::default())
  }

  // user_id: optional string
  pub fn user_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // batch_id: optional string
  pub fn batch_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // received_at: optional message google.protobuf.Timestamp
  pub fn has_received_at(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn received_at_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_received_at().then(|| self.received_at())
  }
  pub fn received_at(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // ingested_at: optional message google.protobuf.Timestamp
  pub fn has_ingested_at(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn ingested_at_opt(self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'msg>> {
    self.has_ingested_at().then(|| self.ingested_at())
  }
  pub fn ingested_at(self) -> ::protobuf_well_known_types::TimestampView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }

  // object: optional message lifechronicle.events.v1.SeriesObjectReference
  pub fn has_object(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn object_opt(self) -> ::std::option::Option<super::SeriesObjectReferenceView<'msg>> {
    self.has_object().then(|| self.object())
  }
  pub fn object(self) -> super::SeriesObjectReferenceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SeriesObjectReferenceView::default())
  }

  // series_submitted_sha256: optional bytes
  pub fn series_submitted_sha256(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // series_canonical_sha256: optional bytes
  pub fn series_canonical_sha256(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

  // authenticated_principal: optional string
  pub fn authenticated_principal(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // quality_signals: repeated message lifechronicle.events.v1.QualitySignal
  pub fn quality_signals(self) -> ::protobuf::RepeatedView<'msg, super::QualitySignal> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::QualitySignal>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `RawSeriesRecordView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RawSeriesRecordView<'_> {}

// SAFETY:
// - `RawSeriesRecordView` is `Send` because while its alive a `RawSeriesRecordMut` cannot.
// - `RawSeriesRecordView` does not use thread-local data.
unsafe impl ::std::marker::Send for RawSeriesRecordView<'_> {}

impl<'msg> ::protobuf::AsView for RawSeriesRecordView<'msg> {
  type Proxied = RawSeriesRecord;
  fn as_view(&self) -> ::protobuf::View<'msg, RawSeriesRecord> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RawSeriesRecordView<'msg> {
  fn into_view<'shorter>(self) -> RawSeriesRecordView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RawSeriesRecord> for RawSeriesRecordView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RawSeriesRecord {
    let mut dst = RawSeriesRecord::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RawSeriesRecord> for RawSeriesRecordMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RawSeriesRecord {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RawSeriesRecord {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RawSeriesRecordView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RawSeriesRecordMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RawSeriesRecordMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RawSeriesRecord>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RawSeriesRecordMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RawSeriesRecordMut<'msg> {
  type Message = RawSeriesRecord;
}

impl ::std::fmt::Debug for RawSeriesRecordMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RawSeriesRecord>> for RawSeriesRecordMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RawSeriesRecord>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RawSeriesRecordMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RawSeriesRecord> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RawSeriesRecord {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // chunk: optional message lifechronicle.events.v1.SeriesChunkMetadata
  pub fn has_chunk(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_chunk(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn chunk_opt(&self) -> ::std::option::Option<super::SeriesChunkMetadataView<'_>> {
    self.has_chunk().then(|| self.chunk())
  }
  pub fn chunk(&self) -> super::SeriesChunkMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SeriesChunkMetadataView::default())
  }
  pub fn chunk_mut(&mut self) -> super::SeriesChunkMetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_chunk(&mut self,
    val: impl ::protobuf::IntoProxied<super::SeriesChunkMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // user_id: optional string
  pub fn user_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_user_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // batch_id: optional string
  pub fn batch_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_batch_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // received_at: optional message google.protobuf.Timestamp
  pub fn has_received_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_received_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn received_at_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_received_at().then(|| self.received_at())
  }
  pub fn received_at(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn received_at_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_received_at(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // ingested_at: optional message google.protobuf.Timestamp
  pub fn has_ingested_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_ingested_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn ingested_at_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_ingested_at().then(|| self.ingested_at())
  }
  pub fn ingested_at(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn ingested_at_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_ingested_at(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // object: optional message lifechronicle.events.v1.SeriesObjectReference
  pub fn has_object(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_object(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn object_opt(&self) -> ::std::option::Option<super::SeriesObjectReferenceView<'_>> {
    self.has_object().then(|| self.object())
  }
  pub fn object(&self) -> super::SeriesObjectReferenceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SeriesObjectReferenceView::default())
  }
  pub fn object_mut(&mut self) -> super::SeriesObjectReferenceMut<'_> {
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
  pub fn set_object(&mut self,
    val: impl ::protobuf::IntoProxied<super::SeriesObjectReference>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // series_submitted_sha256: optional bytes
  pub fn series_submitted_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_series_submitted_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // series_canonical_sha256: optional bytes
  pub fn series_canonical_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_series_canonical_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // authenticated_principal: optional string
  pub fn authenticated_principal(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authenticated_principal(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // quality_signals: repeated message lifechronicle.events.v1.QualitySignal
  pub fn quality_signals(&self) -> ::protobuf::RepeatedView<'_, super::QualitySignal> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::QualitySignal>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn quality_signals_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::QualitySignal> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        9,
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
  pub fn set_quality_signals(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::QualitySignal>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        src);
    }
  }

}

// SAFETY:
// - `RawSeriesRecordMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RawSeriesRecordMut<'_> {}

// SAFETY:
// - `RawSeriesRecordMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RawSeriesRecordMut<'_> {}

impl<'msg> ::protobuf::AsView for RawSeriesRecordMut<'msg> {
  type Proxied = RawSeriesRecord;
  fn as_view(&self) -> ::protobuf::View<'_, RawSeriesRecord> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RawSeriesRecordMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RawSeriesRecord>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RawSeriesRecordMut<'msg> {
  type MutProxied = RawSeriesRecord;
  fn as_mut(&mut self) -> RawSeriesRecordMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RawSeriesRecordMut<'msg> {
  fn into_mut<'shorter>(self) -> RawSeriesRecordMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RawSeriesRecord {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RawSeriesRecord> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RawSeriesRecordView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RawSeriesRecordMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // chunk: optional message lifechronicle.events.v1.SeriesChunkMetadata
  pub fn has_chunk(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_chunk(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn chunk_opt(&self) -> ::std::option::Option<super::SeriesChunkMetadataView<'_>> {
    self.has_chunk().then(|| self.chunk())
  }
  pub fn chunk(&self) -> super::SeriesChunkMetadataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SeriesChunkMetadataView::default())
  }
  pub fn chunk_mut(&mut self) -> super::SeriesChunkMetadataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_chunk(&mut self,
    val: impl ::protobuf::IntoProxied<super::SeriesChunkMetadata>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // user_id: optional string
  pub fn user_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_user_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // batch_id: optional string
  pub fn batch_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_batch_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // received_at: optional message google.protobuf.Timestamp
  pub fn has_received_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_received_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn received_at_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_received_at().then(|| self.received_at())
  }
  pub fn received_at(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn received_at_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_received_at(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // ingested_at: optional message google.protobuf.Timestamp
  pub fn has_ingested_at(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_ingested_at(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn ingested_at_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::TimestampView<'_>> {
    self.has_ingested_at().then(|| self.ingested_at())
  }
  pub fn ingested_at(&self) -> ::protobuf_well_known_types::TimestampView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::TimestampView::default())
  }
  pub fn ingested_at_mut(&mut self) -> ::protobuf_well_known_types::TimestampMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_ingested_at(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Timestamp>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // object: optional message lifechronicle.events.v1.SeriesObjectReference
  pub fn has_object(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_object(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn object_opt(&self) -> ::std::option::Option<super::SeriesObjectReferenceView<'_>> {
    self.has_object().then(|| self.object())
  }
  pub fn object(&self) -> super::SeriesObjectReferenceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::SeriesObjectReferenceView::default())
  }
  pub fn object_mut(&mut self) -> super::SeriesObjectReferenceMut<'_> {
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
  pub fn set_object(&mut self,
    val: impl ::protobuf::IntoProxied<super::SeriesObjectReference>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // series_submitted_sha256: optional bytes
  pub fn series_submitted_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_series_submitted_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // series_canonical_sha256: optional bytes
  pub fn series_canonical_sha256(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_series_canonical_sha256(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // authenticated_principal: optional string
  pub fn authenticated_principal(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        8, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_authenticated_principal(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val);
    }
  }

  // quality_signals: repeated message lifechronicle.events.v1.QualitySignal
  pub fn quality_signals(&self) -> ::protobuf::RepeatedView<'_, super::QualitySignal> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        9
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::QualitySignal>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn quality_signals_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::QualitySignal> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        9,
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
  pub fn set_quality_signals(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::QualitySignal>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        src);
    }
  }

}  // impl RawSeriesRecord

impl ::std::ops::Drop for RawSeriesRecord {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RawSeriesRecord {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RawSeriesRecord {
  type Proxied = Self;
  fn as_view(&self) -> RawSeriesRecordView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RawSeriesRecord {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RawSeriesRecordMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RawSeriesRecord {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::lifechronicle__events__v1__RawSeriesRecord_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X1X3330P0P1XG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::lifechronicle__events__v1__RawSeriesRecord_msg_init.0, &[<super::SeriesChunkMetadata as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Timestamp as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::SeriesObjectReference as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::QualitySignal as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::lifechronicle__events__v1__RawSeriesRecord_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RawSeriesRecord {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RawSeriesRecord {
  type Msg = RawSeriesRecord;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RawSeriesRecord> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RawSeriesRecord {
  type Msg = RawSeriesRecord;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RawSeriesRecord> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RawSeriesRecordMut<'_> {
  type Msg = RawSeriesRecord;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RawSeriesRecord> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RawSeriesRecordMut<'_> {
  type Msg = RawSeriesRecord;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RawSeriesRecord> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RawSeriesRecordView<'_> {
  type Msg = RawSeriesRecord;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RawSeriesRecord> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RawSeriesRecordMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



