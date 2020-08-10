pub mod opentelemetry {
    pub mod proto {
        pub mod collector {
            pub mod metrics {
                pub mod v1 {
                    // tonic::include_proto!("opentelemetry.proto.collector.metrics.v1");
                    include!("opentelemetry.proto.collector.metrics.v1.rs");
                }
            }

            pub mod trace {
                pub mod v1 {
                    // tonic::include_proto!("opentelemetry.proto.collector.trace.v1");
                    include!("opentelemetry.proto.collector.trace.v1.rs");
                }
            }
        }

        pub mod common {
            pub mod v1 {
                // tonic::include_proto!("opentelemetry.proto.common.v1");
                include!("opentelemetry.proto.common.v1.rs");
            }
        }

        pub mod resource {
            pub mod v1 {
                // tonic::include_proto!("opentelemetry.proto.resource.v1");
                include!("opentelemetry.proto.resource.v1.rs");
            }
        }

        pub mod metrics {
            pub mod v1 {
                // tonic::include_proto!("opentelemetry.proto.metrics.v1");
                include!("opentelemetry.proto.metrics.v1.rs");
            }
        }

        pub mod trace {
            pub mod v1 {
                // tonic::include_proto!("opentelemetry.proto.trace.v1");
                include!("opentelemetry.proto.trace.v1.rs");
            }
        }
    }
}
