use crate::proto::opentelemetry::proto::common::v1::KeyValueList;
use crate::proto::opentelemetry::proto::resource::v1::Resource;
use crate::proto::opentelemetry::proto::trace::v1::span::{Event, Link, SpanKind};
use crate::proto::opentelemetry::proto::trace::v1::status::StatusCode;
use crate::proto::opentelemetry::proto::trace::v1::{
    InstrumentationLibrarySpans, ResourceSpans, Span, Status,
};
use crate::transform::common::to_nanos;
use opentelemetry::exporter::trace::SpanData;
use std::sync::Arc;

impl From<opentelemetry::api::SpanKind> for SpanKind {
    fn from(span_kind: opentelemetry::api::SpanKind) -> Self {
        match span_kind {
            opentelemetry::api::SpanKind::Client => SpanKind::Client,
            opentelemetry::api::SpanKind::Consumer => SpanKind::Consumer,
            opentelemetry::api::SpanKind::Internal => SpanKind::Internal,
            opentelemetry::api::SpanKind::Producer => SpanKind::Producer,
            opentelemetry::api::SpanKind::Server => SpanKind::Server,
        }
    }
}

impl From<opentelemetry::api::StatusCode> for StatusCode {
    fn from(status_code: opentelemetry::api::StatusCode) -> Self {
        match status_code {
            opentelemetry::api::StatusCode::OK => StatusCode::Ok,
            opentelemetry::api::StatusCode::Canceled => StatusCode::Cancelled,
            opentelemetry::api::StatusCode::Unknown => StatusCode::UnknownError,
            opentelemetry::api::StatusCode::InvalidArgument => StatusCode::InvalidArgument,
            opentelemetry::api::StatusCode::DeadlineExceeded => StatusCode::DeadlineExceeded,
            opentelemetry::api::StatusCode::NotFound => StatusCode::NotFound,
            opentelemetry::api::StatusCode::AlreadyExists => StatusCode::AlreadyExists,
            opentelemetry::api::StatusCode::PermissionDenied => StatusCode::PermissionDenied,
            opentelemetry::api::StatusCode::ResourceExhausted => StatusCode::ResourceExhausted,
            opentelemetry::api::StatusCode::FailedPrecondition => StatusCode::FailedPrecondition,
            opentelemetry::api::StatusCode::Aborted => StatusCode::Aborted,
            opentelemetry::api::StatusCode::OutOfRange => StatusCode::OutOfRange,
            opentelemetry::api::StatusCode::Unimplemented => StatusCode::Unimplemented,
            opentelemetry::api::StatusCode::Internal => StatusCode::InternalError,
            opentelemetry::api::StatusCode::Unavailable => StatusCode::Unavailable,
            opentelemetry::api::StatusCode::DataLoss => StatusCode::DataLoss,
            opentelemetry::api::StatusCode::Unauthenticated => StatusCode::Unauthenticated,
        }
    }
}

impl From<opentelemetry::api::Link> for Link {
    fn from(link: opentelemetry::api::Link) -> Self {
        Link {
            trace_id: link
                .span_context()
                .trace_id()
                .to_u128()
                .to_be_bytes()
                .to_vec(),
            span_id: link
                .span_context()
                .span_id()
                .to_u64()
                .to_be_bytes()
                .to_vec(),
            // TODO Add TraceState to SpanContext API: https://github.com/open-telemetry/opentelemetry-specification/blob/master/specification/trace/api.md#spancontext
            trace_state: "".to_string(),
            attributes: KeyValueList::from(link.attributes().clone()).values,
            dropped_attributes_count: 0,
            ..Default::default()
        }
    }
}

impl From<Arc<SpanData>> for ResourceSpans {
    fn from(source_span: Arc<SpanData>) -> Self {
        ResourceSpans {
            resource: Some(Resource {
                attributes: Default::default(),
                dropped_attributes_count: 0,
            }),
            instrumentation_library_spans: vec![InstrumentationLibrarySpans {
                instrumentation_library: Default::default(),
                spans: vec![Span {
                    trace_id: source_span
                        .span_context
                        .trace_id()
                        .to_u128()
                        .to_be_bytes()
                        .to_vec(),
                    span_id: source_span
                        .span_context
                        .span_id()
                        .to_u64()
                        .to_be_bytes()
                        .to_vec(),
                    // TODO Add TraceState to SpanContext API: https://github.com/open-telemetry/opentelemetry-specification/blob/master/specification/trace/api.md#spancontext
                    trace_state: "".to_string(),
                    parent_span_id: {
                        if source_span.parent_span_id.to_u64() > 0 {
                            source_span.parent_span_id.to_u64().to_be_bytes().to_vec()
                        } else {
                            vec![]
                        }
                    },
                    name: source_span.name.clone(),
                    kind: source_span.span_kind.clone() as i32,
                    start_time_unix_nano: to_nanos(source_span.start_time),
                    end_time_unix_nano: to_nanos(source_span.end_time),
                    attributes: KeyValueList::from(source_span.attributes.clone()).values,
                    dropped_attributes_count: source_span.attributes.dropped_count(),
                    events: source_span
                        .message_events
                        .clone()
                        .into_iter()
                        .map(|event| Event {
                            time_unix_nano: to_nanos(event.timestamp),
                            name: event.name,
                            attributes: KeyValueList::from(event.attributes).values,
                            dropped_attributes_count: 0,
                        })
                        .collect(),
                    dropped_events_count: 0,
                    links: source_span
                        .links
                        .clone()
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    dropped_links_count: 0,
                    status: Some(Status {
                        code: StatusCode::from(source_span.status_code.clone()) as i32,
                        message: source_span.status_message.clone(),
                    }),
                }],
            }],
        }
    }
}
