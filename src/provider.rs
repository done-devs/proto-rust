#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderResponse {
    #[prost(bool, tag = "1")]
    pub successful: bool,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub count: i64,
    #[prost(message, optional, tag = "4")]
    pub value: ::core::option::Option<::prost_wkt_types::Value>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub favorite: bool,
    #[prost(bool, tag = "5")]
    pub today: bool,
    #[prost(bool, tag = "6")]
    pub is_reminder_on: bool,
    #[prost(enumeration = "Status", tag = "7")]
    pub status: i32,
    #[prost(enumeration = "Priority", tag = "8")]
    pub priority: i32,
    #[prost(message, repeated, tag = "9")]
    pub sub_tasks: ::prost::alloc::vec::Vec<SubTask>,
    #[prost(string, repeated, tag = "10")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub notes: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "12")]
    pub completion_date: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "13")]
    pub deletion_date: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "14")]
    pub due_date: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "15")]
    pub reminder_date: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "16")]
    pub reminder_time: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(string, optional, tag = "17")]
    pub recurrence: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, tag = "18")]
    pub created_date_time: i64,
    #[prost(int64, tag = "19")]
    pub last_modified_date_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubTask {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub completed: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct List {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub icon: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
}
impl Priority {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Priority::Low => "LOW",
            Priority::Normal => "NORMAL",
            Priority::High => "HIGH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOW" => Some(Self::Low),
            "NORMAL" => Some(Self::Normal),
            "HIGH" => Some(Self::High),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    NotStarted = 0,
    Completed = 1,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::NotStarted => "NOT_STARTED",
            Status::Completed => "COMPLETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOT_STARTED" => Some(Self::NotStarted),
            "COMPLETED" => Some(Self::Completed),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod provider_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ProviderClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProviderClient<tonic::transport::Channel> {
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
    impl<T> ProviderClient<T>
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ProviderClient<InterceptedService<T, F>>
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
            ProviderClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn get_task(
            &mut self,
            request: impl tonic::IntoRequest<::prost::alloc::string::String>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status> {
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
                "/provider.Provider/GetTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_tasks(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ProviderResponse>>,
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
                "/provider.Provider/GetTasks",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn create_task(
            &mut self,
            request: impl tonic::IntoRequest<super::Task>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status> {
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
                "/provider.Provider/CreateTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_task(
            &mut self,
            request: impl tonic::IntoRequest<super::Task>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status> {
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
                "/provider.Provider/UpdateTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_task(
            &mut self,
            request: impl tonic::IntoRequest<::prost::alloc::string::String>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status> {
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
                "/provider.Provider/DeleteTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_list(
            &mut self,
            request: impl tonic::IntoRequest<::prost::alloc::string::String>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status> {
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
                "/provider.Provider/GetList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_lists(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ProviderResponse>>,
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
                "/provider.Provider/GetLists",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn get_list_ids(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status> {
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
                "/provider.Provider/GetListIds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_list(
            &mut self,
            request: impl tonic::IntoRequest<super::List>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status> {
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
                "/provider.Provider/CreateList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_list(
            &mut self,
            request: impl tonic::IntoRequest<super::List>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status> {
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
                "/provider.Provider/UpdateList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_list(
            &mut self,
            request: impl tonic::IntoRequest<::prost::alloc::string::String>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status> {
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
                "/provider.Provider/DeleteList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_tasks_from_list(
            &mut self,
            request: impl tonic::IntoRequest<::prost::alloc::string::String>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ProviderResponse>>,
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
                "/provider.Provider/GetTasksFromList",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn get_task_ids_from_list(
            &mut self,
            request: impl tonic::IntoRequest<::prost::alloc::string::String>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status> {
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
                "/provider.Provider/GetTaskIdsFromList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_task_count_from_list(
            &mut self,
            request: impl tonic::IntoRequest<::prost::alloc::string::String>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status> {
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
                "/provider.Provider/GetTaskCountFromList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod provider_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ProviderServer.
    #[async_trait]
    pub trait Provider: Send + Sync + 'static {
        async fn get_task(
            &self,
            request: tonic::Request<::prost::alloc::string::String>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status>;
        /// Server streaming response type for the GetTasks method.
        type GetTasksStream: futures_core::Stream<
                Item = Result<super::ProviderResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn get_tasks(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<Self::GetTasksStream>, tonic::Status>;
        async fn create_task(
            &self,
            request: tonic::Request<super::Task>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status>;
        async fn update_task(
            &self,
            request: tonic::Request<super::Task>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status>;
        async fn delete_task(
            &self,
            request: tonic::Request<::prost::alloc::string::String>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status>;
        async fn get_list(
            &self,
            request: tonic::Request<::prost::alloc::string::String>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status>;
        /// Server streaming response type for the GetLists method.
        type GetListsStream: futures_core::Stream<
                Item = Result<super::ProviderResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn get_lists(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<Self::GetListsStream>, tonic::Status>;
        async fn get_list_ids(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status>;
        async fn create_list(
            &self,
            request: tonic::Request<super::List>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status>;
        async fn update_list(
            &self,
            request: tonic::Request<super::List>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status>;
        async fn delete_list(
            &self,
            request: tonic::Request<::prost::alloc::string::String>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status>;
        /// Server streaming response type for the GetTasksFromList method.
        type GetTasksFromListStream: futures_core::Stream<
                Item = Result<super::ProviderResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn get_tasks_from_list(
            &self,
            request: tonic::Request<::prost::alloc::string::String>,
        ) -> Result<tonic::Response<Self::GetTasksFromListStream>, tonic::Status>;
        async fn get_task_ids_from_list(
            &self,
            request: tonic::Request<::prost::alloc::string::String>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status>;
        async fn get_task_count_from_list(
            &self,
            request: tonic::Request<::prost::alloc::string::String>,
        ) -> Result<tonic::Response<super::ProviderResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ProviderServer<T: Provider> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Provider> ProviderServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ProviderServer<T>
    where
        T: Provider,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/provider.Provider/GetTask" => {
                    #[allow(non_camel_case_types)]
                    struct GetTaskSvc<T: Provider>(pub Arc<T>);
                    impl<
                        T: Provider,
                    > tonic::server::UnaryService<::prost::alloc::string::String>
                    for GetTaskSvc<T> {
                        type Response = super::ProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<::prost::alloc::string::String>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provider.Provider/GetTasks" => {
                    #[allow(non_camel_case_types)]
                    struct GetTasksSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider> tonic::server::ServerStreamingService<()>
                    for GetTasksSvc<T> {
                        type Response = super::ProviderResponse;
                        type ResponseStream = T::GetTasksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_tasks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTasksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provider.Provider/CreateTask" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTaskSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider> tonic::server::UnaryService<super::Task>
                    for CreateTaskSvc<T> {
                        type Response = super::ProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Task>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provider.Provider/UpdateTask" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTaskSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider> tonic::server::UnaryService<super::Task>
                    for UpdateTaskSvc<T> {
                        type Response = super::ProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Task>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provider.Provider/DeleteTask" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTaskSvc<T: Provider>(pub Arc<T>);
                    impl<
                        T: Provider,
                    > tonic::server::UnaryService<::prost::alloc::string::String>
                    for DeleteTaskSvc<T> {
                        type Response = super::ProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<::prost::alloc::string::String>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provider.Provider/GetList" => {
                    #[allow(non_camel_case_types)]
                    struct GetListSvc<T: Provider>(pub Arc<T>);
                    impl<
                        T: Provider,
                    > tonic::server::UnaryService<::prost::alloc::string::String>
                    for GetListSvc<T> {
                        type Response = super::ProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<::prost::alloc::string::String>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provider.Provider/GetLists" => {
                    #[allow(non_camel_case_types)]
                    struct GetListsSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider> tonic::server::ServerStreamingService<()>
                    for GetListsSvc<T> {
                        type Response = super::ProviderResponse;
                        type ResponseStream = T::GetListsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_lists(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetListsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provider.Provider/GetListIds" => {
                    #[allow(non_camel_case_types)]
                    struct GetListIdsSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider> tonic::server::UnaryService<()>
                    for GetListIdsSvc<T> {
                        type Response = super::ProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_list_ids(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetListIdsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provider.Provider/CreateList" => {
                    #[allow(non_camel_case_types)]
                    struct CreateListSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider> tonic::server::UnaryService<super::List>
                    for CreateListSvc<T> {
                        type Response = super::ProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::List>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provider.Provider/UpdateList" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateListSvc<T: Provider>(pub Arc<T>);
                    impl<T: Provider> tonic::server::UnaryService<super::List>
                    for UpdateListSvc<T> {
                        type Response = super::ProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::List>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provider.Provider/DeleteList" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteListSvc<T: Provider>(pub Arc<T>);
                    impl<
                        T: Provider,
                    > tonic::server::UnaryService<::prost::alloc::string::String>
                    for DeleteListSvc<T> {
                        type Response = super::ProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<::prost::alloc::string::String>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provider.Provider/GetTasksFromList" => {
                    #[allow(non_camel_case_types)]
                    struct GetTasksFromListSvc<T: Provider>(pub Arc<T>);
                    impl<
                        T: Provider,
                    > tonic::server::ServerStreamingService<
                        ::prost::alloc::string::String,
                    > for GetTasksFromListSvc<T> {
                        type Response = super::ProviderResponse;
                        type ResponseStream = T::GetTasksFromListStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<::prost::alloc::string::String>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_tasks_from_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTasksFromListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provider.Provider/GetTaskIdsFromList" => {
                    #[allow(non_camel_case_types)]
                    struct GetTaskIdsFromListSvc<T: Provider>(pub Arc<T>);
                    impl<
                        T: Provider,
                    > tonic::server::UnaryService<::prost::alloc::string::String>
                    for GetTaskIdsFromListSvc<T> {
                        type Response = super::ProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<::prost::alloc::string::String>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_task_ids_from_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTaskIdsFromListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provider.Provider/GetTaskCountFromList" => {
                    #[allow(non_camel_case_types)]
                    struct GetTaskCountFromListSvc<T: Provider>(pub Arc<T>);
                    impl<
                        T: Provider,
                    > tonic::server::UnaryService<::prost::alloc::string::String>
                    for GetTaskCountFromListSvc<T> {
                        type Response = super::ProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<::prost::alloc::string::String>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_task_count_from_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTaskCountFromListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Provider> Clone for ProviderServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Provider> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Provider> tonic::server::NamedService for ProviderServer<T> {
        const NAME: &'static str = "provider.Provider";
    }
}
