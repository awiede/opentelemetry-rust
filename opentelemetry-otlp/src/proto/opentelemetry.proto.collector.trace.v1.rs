#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTraceServiceRequest {
    /// An array of ResourceSpans.
    /// For data coming from a single resource this array will typically contain one
    /// element. Intermediary nodes (such as OpenTelemetry Collector) that receive
    /// data from multiple origins typically batch the data before forwarding further and
    /// in that case this array will contain multiple elements.
    #[prost(message, repeated, tag = "1")]
    pub resource_spans: ::std::vec::Vec<super::super::super::trace::v1::ResourceSpans>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportTraceServiceResponse {}
#[doc = r" Generated client implementations."]
pub mod trace_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service that can be used to push spans between one Application instrumented with"]
    #[doc = " OpenTelemetry and an collector, or between an collector and a central collector (in this"]
    #[doc = " case spans are sent/received to/from multiple Applications)."]
    pub struct TraceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TraceServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TraceServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " For performance reasons, it is recommended to keep this RPC"]
        #[doc = " alive for the entire life of the application."]
        pub async fn export(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportTraceServiceRequest>,
        ) -> Result<tonic::Response<super::ExportTraceServiceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/opentelemetry.proto.collector.trace.v1.TraceService/Export",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for TraceServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for TraceServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TraceServiceClient {{ ... }}")
        }
    }
}
