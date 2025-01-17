#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullableString {
    #[prost(oneof="nullable_string::String", tags="1")]
    pub string: ::core::option::Option<nullable_string::String>,
}
/// Nested message and enum types in `NullableString`.
pub mod nullable_string {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum String {
        #[prost(string, tag="1")]
        Value(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullableDouble {
    #[prost(oneof="nullable_double::Double", tags="1")]
    pub double: ::core::option::Option<nullable_double::Double>,
}
/// Nested message and enum types in `NullableDouble`.
pub mod nullable_double {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Double {
        #[prost(double, tag="1")]
        Value(f64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullableBool {
    #[prost(oneof="nullable_bool::Bool", tags="1")]
    pub bool: ::core::option::Option<nullable_bool::Bool>,
}
/// Nested message and enum types in `NullableBool`.
pub mod nullable_bool {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Bool {
        #[prost(bool, tag="1")]
        Value(bool),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullableTimestamp {
    #[prost(oneof="nullable_timestamp::Timestamp", tags="1")]
    pub timestamp: ::core::option::Option<nullable_timestamp::Timestamp>,
}
/// Nested message and enum types in `NullableTimestamp`.
pub mod nullable_timestamp {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Timestamp {
        #[prost(message, tag="1")]
        Value(::prost_types::Timestamp),
    }
}
/// Light-weight representation of a .NET System.Security.Claims.ClaimsIdentity object.
/// This is the same serialization as found in EasyAuth, and needs to be kept in sync with
/// its ClaimsIdentitySlim definition, as seen in the WebJobs extension:
/// <https://github.com/Azure/azure-webjobs-sdk-extensions/blob/dev/src/WebJobs.Extensions.Http/ClaimsIdentitySlim.cs>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcClaimsIdentity {
    #[prost(message, optional, tag="1")]
    pub authentication_type: ::core::option::Option<NullableString>,
    #[prost(message, optional, tag="2")]
    pub name_claim_type: ::core::option::Option<NullableString>,
    #[prost(message, optional, tag="3")]
    pub role_claim_type: ::core::option::Option<NullableString>,
    #[prost(message, repeated, tag="4")]
    pub claims: ::prost::alloc::vec::Vec<RpcClaim>,
}
/// Light-weight representation of a .NET System.Security.Claims.Claim object.
/// This is the same serialization as found in EasyAuth, and needs to be kept in sync with
/// its ClaimSlim definition, as seen in the WebJobs extension:
/// <https://github.com/Azure/azure-webjobs-sdk-extensions/blob/dev/src/WebJobs.Extensions.Http/ClaimSlim.cs>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcClaim {
    #[prost(string, tag="1")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingMessage {
    /// Used to identify message between host and worker
    #[prost(string, tag="1")]
    pub request_id: ::prost::alloc::string::String,
    /// Payload of the message
    #[prost(oneof="streaming_message::Content", tags="20, 17, 16, 15, 14, 12, 13, 6, 7, 8, 9, 4, 5, 21, 2, 25, 26")]
    pub content: ::core::option::Option<streaming_message::Content>,
}
/// Nested message and enum types in `StreamingMessage`.
pub mod streaming_message {
    /// Payload of the message
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// Worker initiates stream
        #[prost(message, tag="20")]
        StartStream(super::StartStream),
        /// Host sends capabilities/init data to worker
        #[prost(message, tag="17")]
        WorkerInitRequest(super::WorkerInitRequest),
        /// Worker responds after initializing with its capabilities & status
        #[prost(message, tag="16")]
        WorkerInitResponse(super::WorkerInitResponse),
        /// Worker periodically sends empty heartbeat message to host
        #[prost(message, tag="15")]
        WorkerHeartbeat(super::WorkerHeartbeat),
        /// Host sends terminate message to worker.
        /// Worker terminates if it can, otherwise host terminates after a grace period
        #[prost(message, tag="14")]
        WorkerTerminate(super::WorkerTerminate),
        /// Add any worker relevant status to response
        #[prost(message, tag="12")]
        WorkerStatusRequest(super::WorkerStatusRequest),
        #[prost(message, tag="13")]
        WorkerStatusResponse(super::WorkerStatusResponse),
        /// On file change event, host sends notification to worker
        #[prost(message, tag="6")]
        FileChangeEventRequest(super::FileChangeEventRequest),
        /// Worker requests a desired action (restart worker, reload function)
        #[prost(message, tag="7")]
        WorkerActionResponse(super::WorkerActionResponse),
        /// Host sends required metadata to worker to load function
        #[prost(message, tag="8")]
        FunctionLoadRequest(super::FunctionLoadRequest),
        /// Worker responds after loading with the load result
        #[prost(message, tag="9")]
        FunctionLoadResponse(super::FunctionLoadResponse),
        /// Host requests a given invocation
        #[prost(message, tag="4")]
        InvocationRequest(super::InvocationRequest),
        /// Worker responds to a given invocation
        #[prost(message, tag="5")]
        InvocationResponse(super::InvocationResponse),
        /// Host sends cancel message to attempt to cancel an invocation. 
        /// If an invocation is cancelled, host will receive an invocation response with status cancelled.
        #[prost(message, tag="21")]
        InvocationCancel(super::InvocationCancel),
        /// Worker logs a message back to the host
        #[prost(message, tag="2")]
        RpcLog(super::RpcLog),
        #[prost(message, tag="25")]
        FunctionEnvironmentReloadRequest(super::FunctionEnvironmentReloadRequest),
        #[prost(message, tag="26")]
        FunctionEnvironmentReloadResponse(super::FunctionEnvironmentReloadResponse),
    }
}
// Process.Start required info
//   connection details
//   protocol type
//   protocol version 

/// Worker sends the host information identifying itself
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartStream {
    /// id of the worker
    #[prost(string, tag="2")]
    pub worker_id: ::prost::alloc::string::String,
}
/// Host requests the worker to initialize itself 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerInitRequest {
    /// version of the host sending init request
    #[prost(string, tag="1")]
    pub host_version: ::prost::alloc::string::String,
    /// A map of host supported features/capabilities
    #[prost(map="string, string", tag="2")]
    pub capabilities: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// inform worker of supported categories and their levels
    /// i.e. Worker = Verbose, Function.MyFunc = None
    #[prost(map="string, enumeration(rpc_log::Level)", tag="3")]
    pub log_categories: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
}
/// Worker responds with the result of initializing itself
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerInitResponse {
    /// Version of worker
    #[prost(string, tag="1")]
    pub worker_version: ::prost::alloc::string::String,
    /// A map of worker supported features/capabilities
    #[prost(map="string, string", tag="2")]
    pub capabilities: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Status of the response
    #[prost(message, optional, tag="3")]
    pub result: ::core::option::Option<StatusResult>,
}
/// Used by the host to determine success/failure/cancellation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResult {
    /// Status for the given result
    #[prost(enumeration="status_result::Status", tag="4")]
    pub status: i32,
    /// Specific message about the result
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
    /// Exception message (if exists) for the status
    #[prost(message, optional, tag="2")]
    pub exception: ::core::option::Option<RpcException>,
    /// Captured logs or relevant details can use the logs property
    #[prost(message, repeated, tag="3")]
    pub logs: ::prost::alloc::vec::Vec<RpcLog>,
}
/// Nested message and enum types in `StatusResult`.
pub mod status_result {
    /// Indicates Failure/Success/Cancelled
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Failure = 0,
        Success = 1,
        Cancelled = 2,
    }
}
// TODO: investigate grpc heartbeat - don't limit to grpc implemention

/// Message is empty by design - Will add more fields in future if needed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerHeartbeat {
}
/// Warning before killing the process after grace_period
/// Worker self terminates ..no response on this
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerTerminate {
    #[prost(message, optional, tag="1")]
    pub grace_period: ::core::option::Option<::prost_types::Duration>,
}
/// Host notifies worker of file content change
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileChangeEventRequest {
    /// type for this event
    #[prost(enumeration="file_change_event_request::Type", tag="1")]
    pub r#type: i32,
    /// full file path for the file change notification
    #[prost(string, tag="2")]
    pub full_path: ::prost::alloc::string::String,
    /// Name of the function affected
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `FileChangeEventRequest`.
pub mod file_change_event_request {
    /// Types of File change operations (See link for more info: <https://msdn.microsoft.com/en-us/library/t6xf43e0(v=vs.110>).aspx)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Unknown = 0,
        Created = 1,
        Deleted = 2,
        Changed = 4,
        Renamed = 8,
        All = 15,
    }
}
/// Indicates whether worker reloaded successfully or needs a restart
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerActionResponse {
    /// action for this response
    #[prost(enumeration="worker_action_response::Action", tag="1")]
    pub action: i32,
    /// text reason for the response
    #[prost(string, tag="2")]
    pub reason: ::prost::alloc::string::String,
}
/// Nested message and enum types in `WorkerActionResponse`.
pub mod worker_action_response {
    /// indicates whether a restart is needed, or reload succesfully
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        Restart = 0,
        Reload = 1,
    }
}
/// NOT USED
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerStatusRequest {
}
/// NOT USED
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerStatusResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionEnvironmentReloadRequest {
    /// Environment variables from the current process
    #[prost(map="string, string", tag="1")]
    pub environment_variables: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionEnvironmentReloadResponse {
    /// Status of the response
    #[prost(message, optional, tag="3")]
    pub result: ::core::option::Option<StatusResult>,
}
/// Host tells the worker to load a Function
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionLoadRequest {
    /// unique function identifier (avoid name collisions, facilitate reload case)
    #[prost(string, tag="1")]
    pub function_id: ::prost::alloc::string::String,
    /// Metadata for the request
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<RpcFunctionMetadata>,
    /// A flag indicating if managed dependency is enabled or not
    #[prost(bool, tag="3")]
    pub managed_dependency_enabled: bool,
}
/// Worker tells host result of reload
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionLoadResponse {
    /// unique function identifier
    #[prost(string, tag="1")]
    pub function_id: ::prost::alloc::string::String,
    /// Result of load operation
    ///
    /// TODO: return type expected?
    #[prost(message, optional, tag="2")]
    pub result: ::core::option::Option<StatusResult>,
    /// Result of load operation
    #[prost(bool, tag="3")]
    pub is_dependency_downloaded: bool,
}
/// Information on how a Function should be loaded and its bindings
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcFunctionMetadata {
    /// TODO: do we want the host's name - the language worker might do a better job of assignment than the host
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// base directory for the Function
    #[prost(string, tag="1")]
    pub directory: ::prost::alloc::string::String,
    /// Script file specified
    #[prost(string, tag="2")]
    pub script_file: ::prost::alloc::string::String,
    /// Entry point specified
    #[prost(string, tag="3")]
    pub entry_point: ::prost::alloc::string::String,
    /// Bindings info
    #[prost(map="string, message", tag="6")]
    pub bindings: ::std::collections::HashMap<::prost::alloc::string::String, BindingInfo>,
    /// Is set to true for proxy
    #[prost(bool, tag="7")]
    pub is_proxy: bool,
}
/// Host requests worker to invoke a Function
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvocationRequest {
    /// Unique id for each invocation
    #[prost(string, tag="1")]
    pub invocation_id: ::prost::alloc::string::String,
    /// Unique id for each Function
    #[prost(string, tag="2")]
    pub function_id: ::prost::alloc::string::String,
    /// Input bindings (include trigger)
    #[prost(message, repeated, tag="3")]
    pub input_data: ::prost::alloc::vec::Vec<ParameterBinding>,
    /// binding metadata from trigger
    #[prost(map="string, message", tag="4")]
    pub trigger_metadata: ::std::collections::HashMap<::prost::alloc::string::String, TypedData>,
}
/// Host requests worker to cancel invocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvocationCancel {
    /// Unique id for invocation
    #[prost(string, tag="2")]
    pub invocation_id: ::prost::alloc::string::String,
    /// Time period before force shutdown
    ///
    /// could also use absolute time
    #[prost(message, optional, tag="1")]
    pub grace_period: ::core::option::Option<::prost_types::Duration>,
}
/// Worker responds with status of Invocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvocationResponse {
    /// Unique id for invocation
    #[prost(string, tag="1")]
    pub invocation_id: ::prost::alloc::string::String,
    /// Output binding data
    #[prost(message, repeated, tag="2")]
    pub output_data: ::prost::alloc::vec::Vec<ParameterBinding>,
    /// data returned from Function (for $return and triggers with return support)
    #[prost(message, optional, tag="4")]
    pub return_value: ::core::option::Option<TypedData>,
    /// Status of the invocation (success/failure/canceled)
    #[prost(message, optional, tag="3")]
    pub result: ::core::option::Option<StatusResult>,
}
/// Used to encapsulate data which could be a variety of types
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedData {
    #[prost(oneof="typed_data::Data", tags="1, 2, 3, 4, 5, 6, 7")]
    pub data: ::core::option::Option<typed_data::Data>,
}
/// Nested message and enum types in `TypedData`.
pub mod typed_data {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(string, tag="1")]
        String(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        Json(::prost::alloc::string::String),
        #[prost(bytes, tag="3")]
        Bytes(::prost::alloc::vec::Vec<u8>),
        #[prost(bytes, tag="4")]
        Stream(::prost::alloc::vec::Vec<u8>),
        #[prost(message, tag="5")]
        Http(::prost::alloc::boxed::Box<super::RpcHttp>),
        #[prost(sint64, tag="6")]
        Int(i64),
        #[prost(double, tag="7")]
        Double(f64),
    }
}
/// Used to describe a given binding on invocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterBinding {
    /// Name for the binding
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Data for the binding
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<TypedData>,
}
/// Used to describe a given binding on load
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindingInfo {
    /// Type of binding (e.g. HttpTrigger)
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// Direction of the given binding
    #[prost(enumeration="binding_info::Direction", tag="3")]
    pub direction: i32,
    #[prost(enumeration="binding_info::DataType", tag="4")]
    pub data_type: i32,
}
/// Nested message and enum types in `BindingInfo`.
pub mod binding_info {
    /// Indicates whether it is an input or output binding (or a fancy inout binding)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Direction {
        In = 0,
        Out = 1,
        Inout = 2,
    }
    /// Indicates the type of the data for the binding
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataType {
        Undefined = 0,
        String = 1,
        Binary = 2,
        Stream = 3,
    }
}
/// Used to send logs back to the Host 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcLog {
    /// Unique id for invocation (if exists)
    #[prost(string, tag="1")]
    pub invocation_id: ::prost::alloc::string::String,
    /// TOD: This should be an enum
    /// Category for the log (startup, load, invocation, etc.)
    #[prost(string, tag="2")]
    pub category: ::prost::alloc::string::String,
    /// Level for the given log message
    #[prost(enumeration="rpc_log::Level", tag="3")]
    pub level: i32,
    /// Message for the given log
    #[prost(string, tag="4")]
    pub message: ::prost::alloc::string::String,
    /// Id for the even associated with this log (if exists)
    #[prost(string, tag="5")]
    pub event_id: ::prost::alloc::string::String,
    /// Exception (if exists)
    #[prost(message, optional, tag="6")]
    pub exception: ::core::option::Option<RpcException>,
    /// json serialized property bag, or could use a type scheme like map<string, TypedData>
    #[prost(string, tag="7")]
    pub properties: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RpcLog`.
pub mod rpc_log {
    /// Matching ILogger semantics
    /// <https://github.com/aspnet/Logging/blob/9506ccc3f3491488fe88010ef8b9eb64594abf95/src/Microsoft.Extensions.Logging/Logger.cs>
    /// Level for the Log
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Level {
        Trace = 0,
        Debug = 1,
        Information = 2,
        Warning = 3,
        Error = 4,
        Critical = 5,
        None = 6,
    }
}
/// Encapsulates an Exception 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcException {
    /// Source of the exception
    #[prost(string, tag="3")]
    pub source: ::prost::alloc::string::String,
    /// Stack trace for the exception
    #[prost(string, tag="1")]
    pub stack_trace: ::prost::alloc::string::String,
    /// Textual message describing hte exception
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
/// Http cookie type. Note that only name and value are used for Http requests
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcHttpCookie {
    /// Cookie name
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Cookie value
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
    /// Specifies allowed hosts to receive the cookie
    #[prost(message, optional, tag="3")]
    pub domain: ::core::option::Option<NullableString>,
    /// Specifies URL path that must exist in the requested URL
    #[prost(message, optional, tag="4")]
    pub path: ::core::option::Option<NullableString>,
    /// Sets the cookie to expire at a specific date instead of when the client closes.
    /// It is generally recommended that you use "Max-Age" over "Expires".
    #[prost(message, optional, tag="5")]
    pub expires: ::core::option::Option<NullableTimestamp>,
    /// Sets the cookie to only be sent with an encrypted request
    #[prost(message, optional, tag="6")]
    pub secure: ::core::option::Option<NullableBool>,
    /// Sets the cookie to be inaccessible to JavaScript's Document.cookie API
    #[prost(message, optional, tag="7")]
    pub http_only: ::core::option::Option<NullableBool>,
    /// Allows servers to assert that a cookie ought not to be sent along with cross-site requests
    #[prost(enumeration="rpc_http_cookie::SameSite", tag="8")]
    pub same_site: i32,
    /// Number of seconds until the cookie expires. A zero or negative number will expire the cookie immediately.
    #[prost(message, optional, tag="9")]
    pub max_age: ::core::option::Option<NullableDouble>,
}
/// Nested message and enum types in `RpcHttpCookie`.
pub mod rpc_http_cookie {
    /// Enum that lets servers require that a cookie shouoldn't be sent with cross-site requests
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SameSite {
        None = 0,
        Lax = 1,
        Strict = 2,
    }
}
/// TODO - solidify this or remove it
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcHttp {
    #[prost(string, tag="1")]
    pub method: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="3")]
    pub headers: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(message, optional, boxed, tag="4")]
    pub body: ::core::option::Option<::prost::alloc::boxed::Box<TypedData>>,
    #[prost(map="string, string", tag="10")]
    pub params: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="12")]
    pub status_code: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="15")]
    pub query: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(bool, tag="16")]
    pub enable_content_negotiation: bool,
    #[prost(message, optional, boxed, tag="17")]
    pub raw_body: ::core::option::Option<::prost::alloc::boxed::Box<TypedData>>,
    #[prost(message, repeated, tag="18")]
    pub identities: ::prost::alloc::vec::Vec<RpcClaimsIdentity>,
    #[prost(message, repeated, tag="19")]
    pub cookies: ::prost::alloc::vec::Vec<RpcHttpCookie>,
}
/// Generated client implementations.
pub mod function_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Interface exported by the server.
    #[derive(Debug, Clone)]
    pub struct FunctionRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FunctionRpcClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> FunctionRpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> FunctionRpcClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            FunctionRpcClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn event_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::StreamingMessage>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamingMessage>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/AzureFunctionsRpcMessages.FunctionRpc/EventStream",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
