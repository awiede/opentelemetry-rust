/// A collection of InstrumentationLibraryMetrics from a Resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceMetrics {
    /// The resource for the metrics in this message.
    /// If this field is not set then no resource info is known.
    #[prost(message, optional, tag = "1")]
    pub resource: ::std::option::Option<super::super::resource::v1::Resource>,
    /// A list of metrics that originate from a resource.
    #[prost(message, repeated, tag = "2")]
    pub instrumentation_library_metrics: ::std::vec::Vec<InstrumentationLibraryMetrics>,
}
/// A collection of Metrics produced by an InstrumentationLibrary.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentationLibraryMetrics {
    /// The instrumentation library information for the metrics in this message.
    /// If this field is not set then no library info is known.
    #[prost(message, optional, tag = "1")]
    pub instrumentation_library:
        ::std::option::Option<super::super::common::v1::InstrumentationLibrary>,
    /// A list of metrics that originate from an instrumentation library.
    #[prost(message, repeated, tag = "2")]
    pub metrics: ::std::vec::Vec<Metric>,
}
/// Defines a Metric which has one or more timeseries.
///
/// The data model and relation between entities is shown in the
/// diagram below. Here, "DataPoint" is the term used to refer to any
/// one of the specific data point value types, and "points" is the term used
/// to refer to any one of the lists of points contained in the Metric.
///
/// - Metric is composed of a MetricDescriptor and a list of data points.
/// - MetricDescriptor contains a name, description, unit, type, and temporarility.
/// - Points is a list of DataPoints (shown vertically).
/// - DataPoint contains timestamps, labels, and one of the possible value type fields.
///
///     Metric
///  +----------+         +------------------------+
///  |descriptor|-------->| MetricDescriptor       |
///  |          |         | name                   |
///  |          |         | description            |
///  |          |         | unit                   |
///  |    points|--+      | type                   |
///  +----------+  |      | temporarility          |
///                |      +------------------------+
///                |
///                |      +---------------------------+
///                |      |DataPoint 1                |
///                v      |+------+------+   +------+ |
///             +-----+   ||label |label |...|label | |
///             |  1  |-->||value1|value2|...|valueN| |
///             +-----+   |+------+------+   +------+ |
///             |  .  |   |+-----+                    |
///             |  .  |   ||value|                    |
///             |  .  |   |+-----+                    |
///             |  .  |   +---------------------------+
///             |  .  |                   .
///             |  .  |                   .
///             |  .  |                   .
///             |  .  |   +---------------------------+
///             |  .  |   |DataPoint M                |
///             +-----+   |+------+------+   +------+ |
///             |  M  |-->||label |label |...|label | |
///             +-----+   ||value1|value2|...|valueN| |
///                       |+------+------+   +------+ |
///                       |+-----+                    |
///                       ||value|                    |
///                       |+-----+                    |
///                       +---------------------------+
///
/// All DataPoint types have three common fields:
/// - Labels zero or more key-value pairs associated with the data point.
/// - StartTimeUnixNano MUST be set to the start of the interval when the
///   descriptor Temporality includes CUMULATIVE or DELTA. This field is not set
///   for INSTANTANEOUS timeseries, where instead the TimeUnixNano field is
///   set for individual points.
/// - TimeUnixNano MUST be set to:
///   - the end of the interval (CUMULATIVE or DELTA)
///   - the instantaneous time of the event (INSTANTANEOUS).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metric {
    /// metric_descriptor describes the Metric.
    #[prost(message, optional, tag = "1")]
    pub metric_descriptor: ::std::option::Option<MetricDescriptor>,
    /// Data is a list of one or more DataPoints for a single metric. Only one of the
    /// following fields is used for the data, depending on the type of the metric defined
    /// by MetricDescriptor.type field.
    #[prost(message, repeated, tag = "2")]
    pub int64_data_points: ::std::vec::Vec<Int64DataPoint>,
    #[prost(message, repeated, tag = "3")]
    pub double_data_points: ::std::vec::Vec<DoubleDataPoint>,
    #[prost(message, repeated, tag = "4")]
    pub histogram_data_points: ::std::vec::Vec<HistogramDataPoint>,
    #[prost(message, repeated, tag = "5")]
    pub summary_data_points: ::std::vec::Vec<SummaryDataPoint>,
}
/// Defines a metric type and its schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricDescriptor {
    /// name of the metric, including its DNS name prefix. It must be unique.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// description of the metric, which can be used in documentation.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// unit in which the metric value is reported. Follows the format
    /// described by http://unitsofmeasure.org/ucum.html.
    #[prost(string, tag = "3")]
    pub unit: std::string::String,
    /// type is the type of values this metric has.
    #[prost(enumeration = "metric_descriptor::Type", tag = "4")]
    pub r#type: i32,
    /// temporality is the Temporality of values this metric has.
    #[prost(enumeration = "metric_descriptor::Temporality", tag = "5")]
    pub temporality: i32,
}
pub mod metric_descriptor {
    /// Type is the type of values a metric has.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// INVALID_TYPE is the default Type, it MUST not be used.
        InvalidType = 0,
        /// INT64 values are signed 64-bit integers.
        ///
        /// A Metric of this Type MUST store its values as Int64DataPoint.
        Int64 = 1,
        /// MONOTONIC_INT64 values are monotonically increasing signed 64-bit
        /// integers.
        ///
        /// A Metric of this Type MUST store its values as Int64DataPoint.
        MonotonicInt64 = 2,
        /// DOUBLE values are double-precision floating-point numbers.
        ///
        /// A Metric of this Type MUST store its values as DoubleDataPoint.
        Double = 3,
        /// MONOTONIC_DOUBLE values are monotonically increasing double-precision
        /// floating-point numbers.
        ///
        /// A Metric of this Type MUST store its values as DoubleDataPoint.
        MonotonicDouble = 4,
        /// Histogram measurement.
        /// Corresponding values are stored in HistogramDataPoint.
        Histogram = 5,
        /// Summary value. Some frameworks implemented Histograms as a summary of observations
        /// (usually things like request durations and response sizes). While it
        /// also provides a total count of observations and a sum of all observed
        /// values, it calculates configurable quantiles over a sliding time
        /// window.
        /// Corresponding values are stored in SummaryDataPoint.
        Summary = 6,
    }
    /// Temporality is the temporal quality values of a metric have. It
    /// describes how those values relate to the time interval over which they
    /// are reported.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Temporality {
        /// INVALID_TEMPORALITY is the default Temporality, it MUST not be
        /// used.
        InvalidTemporality = 0,
        /// INSTANTANEOUS is a metric whose values are measured at a particular
        /// instant. The values are not aggregated over any time interval and are
        /// unique per timestamp. As such, these metrics are not expected to have
        /// an associated start time.
        Instantaneous = 1,
        /// DELTA is a metric whose values are the aggregation of measurements
        /// made over a time interval. Successive metrics contain aggregation of
        /// values from continuous and non-overlapping intervals.
        ///
        /// The values for a DELTA metric are based only on the time interval
        /// associated with one measurement cycle. There is no dependency on
        /// previous measurements like is the case for CUMULATIVE metrics.
        ///
        /// For example, consider a system measuring the number of requests that
        /// it receives and reports the sum of these requests every second as a
        /// DELTA metric:
        ///
        ///   1. The system starts receiving at time=t_0.
        ///   2. A request is received, the system measures 1 request.
        ///   3. A request is received, the system measures 1 request.
        ///   4. A request is received, the system measures 1 request.
        ///   5. The 1 second collection cycle ends. A metric is exported for the
        ///      number of requests received over the interval of time t_0 to
        ///      t_0+1 with a value of 3.
        ///   6. A request is received, the system measures 1 request.
        ///   7. A request is received, the system measures 1 request.
        ///   8. The 1 second collection cycle ends. A metric is exported for the
        ///      number of requests received over the interval of time t_0+1 to
        ///      t_0+2 with a value of 2.
        Delta = 2,
        /// CUMULATIVE is a metric whose values are the aggregation of
        /// successively made measurements from a fixed start time until the last
        /// reported measurement. This means that current values of a CUMULATIVE
        /// metric depend on all previous measurements since the start time.
        /// Because of this, the sender is required to retain this state in some
        /// form. If this state is lost or invalidated, the CUMULATIVE metric
        /// values MUST be reset and a new fixed start time following the last
        /// reported measurement time sent MUST be used.
        ///
        /// For example, consider a system measuring the number of requests that
        /// it receives and reports the sum of these requests every second as a
        /// CUMULATIVE metric:
        ///
        ///   1. The system starts receiving at time=t_0.
        ///   2. A request is received, the system measures 1 request.
        ///   3. A request is received, the system measures 1 request.
        ///   4. A request is received, the system measures 1 request.
        ///   5. The 1 second collection cycle ends. A metric is exported for the
        ///      number of requests received over the interval of time t_0 to
        ///      t_0+1 with a value of 3.
        ///   6. A request is received, the system measures 1 request.
        ///   7. A request is received, the system measures 1 request.
        ///   8. The 1 second collection cycle ends. A metric is exported for the
        ///      number of requests received over the interval of time t_0 to
        ///      t_0+2 with a value of 5.
        ///   9. The system experiences a fault and loses state.
        ///   10. The system recovers and resumes receiving at time=t_1.
        ///   11. A request is received, the system measures 1 request.
        ///   12. The 1 second collection cycle ends. A metric is exported for the
        ///      number of requests received over the interval of time t_1 to
        ///      t_0+1 with a value of 1.
        Cumulative = 3,
    }
}
/// Int64DataPoint is a single data point in a timeseries that describes the time-varying
/// values of a int64 metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64DataPoint {
    /// The set of labels that uniquely identify this timeseries.
    #[prost(message, repeated, tag = "1")]
    pub labels: ::std::vec::Vec<super::super::common::v1::StringKeyValue>,
    /// start_time_unix_nano is the time when the cumulative value was reset to zero.
    /// This is used for Counter type only. For Gauge the value is not specified and
    /// defaults to 0.
    ///
    /// The cumulative value is over the time interval (start_time_unix_nano, time_unix_nano].
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    ///
    /// Value of 0 indicates that the timestamp is unspecified. In that case the timestamp
    /// may be decided by the backend.
    #[prost(fixed64, tag = "2")]
    pub start_time_unix_nano: u64,
    /// time_unix_nano is the moment when this value was recorded.
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    #[prost(fixed64, tag = "3")]
    pub time_unix_nano: u64,
    /// value itself.
    #[prost(int64, tag = "4")]
    pub value: i64,
}
/// DoubleDataPoint is a single data point in a timeseries that describes the time-varying
/// value of a double metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleDataPoint {
    /// The set of labels that uniquely identify this timeseries.
    #[prost(message, repeated, tag = "1")]
    pub labels: ::std::vec::Vec<super::super::common::v1::StringKeyValue>,
    /// start_time_unix_nano is the time when the cumulative value was reset to zero.
    /// This is used for Counter type only. For Gauge the value is not specified and
    /// defaults to 0.
    ///
    /// The cumulative value is over the time interval (start_time_unix_nano, time_unix_nano].
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    ///
    /// Value of 0 indicates that the timestamp is unspecified. In that case the timestamp
    /// may be decided by the backend.
    #[prost(fixed64, tag = "2")]
    pub start_time_unix_nano: u64,
    /// time_unix_nano is the moment when this value was recorded.
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    #[prost(fixed64, tag = "3")]
    pub time_unix_nano: u64,
    /// value itself.
    #[prost(double, tag = "4")]
    pub value: f64,
}
/// HistogramDataPoint is a single data point in a timeseries that describes the time-varying
/// values of a Histogram. A Histogram contains summary statistics for a population of values,
/// it may optionally contain the distribution of those values across a set of buckets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramDataPoint {
    /// The set of labels that uniquely identify this timeseries.
    #[prost(message, repeated, tag = "1")]
    pub labels: ::std::vec::Vec<super::super::common::v1::StringKeyValue>,
    /// start_time_unix_nano is the time when the cumulative value was reset to zero.
    ///
    /// The cumulative value is over the time interval (start_time_unix_nano, time_unix_nano].
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    ///
    /// Value of 0 indicates that the timestamp is unspecified. In that case the timestamp
    /// may be decided by the backend.
    /// Note: this field is always unspecified and ignored if MetricDescriptor.type==GAUGE_HISTOGRAM.
    #[prost(fixed64, tag = "2")]
    pub start_time_unix_nano: u64,
    /// time_unix_nano is the moment when this value was recorded.
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    #[prost(fixed64, tag = "3")]
    pub time_unix_nano: u64,
    /// count is the number of values in the population. Must be non-negative. This value
    /// must be equal to the sum of the "count" fields in buckets if a histogram is provided.
    #[prost(uint64, tag = "4")]
    pub count: u64,
    /// sum of the values in the population. If count is zero then this field
    /// must be zero. This value must be equal to the sum of the "sum" fields in buckets if
    /// a histogram is provided.
    #[prost(double, tag = "5")]
    pub sum: f64,
    /// buckets is an optional field contains the values of histogram for each bucket.
    ///
    /// The sum of the values in the buckets "count" field must equal the value in the count field.
    ///
    /// The number of elements in buckets array must be by one greater than the
    /// number of elements in bucket_bounds array.
    ///
    /// Note: if HistogramDataPoint.bucket_options defines bucket bounds then this field
    /// must also be present and number of elements in this field must be equal to the
    /// number of buckets defined by bucket_options.
    #[prost(message, repeated, tag = "6")]
    pub buckets: ::std::vec::Vec<histogram_data_point::Bucket>,
    // A histogram may optionally contain the distribution of the values in the population.
    // In that case one of the option fields below and "buckets" field both must be defined.
    // Otherwise all option fields and "buckets" field must be omitted in which case the
    // distribution of values in the histogram is unknown and only the total count and sum are known.

    // explicit_bounds is the only supported bucket option currently.
    // TODO: Add more bucket options.
    /// explicit_bounds specifies buckets with explicitly defined bounds for values.
    /// The bucket boundaries are described by "bounds" field.
    ///
    /// This defines size(bounds) + 1 (= N) buckets. The boundaries for bucket
    /// at index i are:
    ///
    /// (-infinity, bounds[i]) for i == 0
    /// [bounds[i-1], bounds[i]) for 0 < i < N-1
    /// [bounds[i], +infinity) for i == N-1
    /// The values in bounds array must be strictly increasing.
    ///
    /// Note: only [a, b) intervals are currently supported for each bucket except the first one.
    /// If we decide to also support (a, b] intervals we should add support for these by defining
    /// a boolean value which decides what type of intervals to use.
    #[prost(double, repeated, tag = "7")]
    pub explicit_bounds: ::std::vec::Vec<f64>,
}
pub mod histogram_data_point {
    /// Bucket contains values for a bucket.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Bucket {
        /// The number of values in each bucket of the histogram, as described by
        /// bucket_options.
        #[prost(uint64, tag = "1")]
        pub count: u64,
        /// exemplar is an optional representative value of the bucket.
        #[prost(message, optional, tag = "2")]
        pub exemplar: ::std::option::Option<bucket::Exemplar>,
    }
    pub mod bucket {
        /// Exemplars are example points that may be used to annotate aggregated
        /// Histogram values. They are metadata that gives information about a
        /// particular value added to a Histogram bucket.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Exemplar {
            /// Value of the exemplar point. It determines which bucket the exemplar belongs to.
            /// If bucket_options define bounds for this bucket then this value must be within
            /// the defined bounds.
            #[prost(double, tag = "1")]
            pub value: f64,
            /// time_unix_nano is the moment when this exemplar was recorded.
            /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
            #[prost(fixed64, tag = "2")]
            pub time_unix_nano: u64,
            /// exemplar_attachments are contextual information about the example value.
            /// Keys in this list must be unique.
            #[prost(message, repeated, tag = "3")]
            pub attachments:
                ::std::vec::Vec<super::super::super::super::common::v1::StringKeyValue>,
        }
    }
}
/// SummaryDataPoint is a single data point in a timeseries that describes the time-varying
/// values of a Summary metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummaryDataPoint {
    /// The set of labels that uniquely identify this timeseries.
    #[prost(message, repeated, tag = "1")]
    pub labels: ::std::vec::Vec<super::super::common::v1::StringKeyValue>,
    /// start_time_unix_nano is the time when the cumulative value was reset to zero.
    ///
    /// The cumulative value is over the time interval (start_time_unix_nano, time_unix_nano].
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    ///
    /// Value of 0 indicates that the timestamp is unspecified. In that case the timestamp
    /// may be decided by the backend.
    #[prost(fixed64, tag = "2")]
    pub start_time_unix_nano: u64,
    /// time_unix_nano is the moment when this value was recorded.
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    #[prost(fixed64, tag = "3")]
    pub time_unix_nano: u64,
    /// The total number of recorded values since start_time. Optional since
    /// some systems don't expose this.
    #[prost(uint64, tag = "4")]
    pub count: u64,
    /// The total sum of recorded values since start_time. Optional since some
    /// systems don't expose this. If count is zero then this field must be zero.
    #[prost(double, tag = "5")]
    pub sum: f64,
    /// A list of values at different quantiles of the distribution calculated
    /// from the current snapshot. The quantiles must be strictly increasing.
    #[prost(message, repeated, tag = "6")]
    pub quantile_values: ::std::vec::Vec<summary_data_point::ValueAtQuantile>,
}
pub mod summary_data_point {
    /// Represents the value at a given quantile of a distribution.
    ///
    /// To record Min and Max values following conventions are used:
    /// - The 1.0 quantile is equivalent to the maximum value observed.
    /// - The 0.0 quantile is equivalent to the minimum value observed.
    ///
    /// See the following issue for more context:
    /// https://github.com/open-telemetry/opentelemetry-proto/issues/125
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValueAtQuantile {
        /// The quantile of a distribution. Must be in the interval
        /// [0.0, 1.0].
        #[prost(double, tag = "1")]
        pub quantile: f64,
        /// The value at the given quantile of a distribution.
        #[prost(double, tag = "2")]
        pub value: f64,
    }
}
