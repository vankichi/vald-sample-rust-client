#[doc = r" Generated client implementations."]
pub mod insert_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Insert service provides ways to add new vectors."]
    #[derive(Debug, Clone)]
    pub struct InsertClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InsertClient<tonic::transport::Channel> {
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
    impl<T> InsertClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InsertClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            InsertClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " A method to add a new single vector."]
        pub async fn insert(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::insert::Request>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Insert/Insert");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to add new multiple vectors by bidirectional streaming."]
        pub async fn stream_insert(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::payload::v1::insert::Request,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::object::StreamLocation>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Insert/StreamInsert");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " A method to add new multiple vectors in a single request."]
        pub async fn multi_insert(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::insert::MultiRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Insert/MultiInsert");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod insert_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with InsertServer."]
    #[async_trait]
    pub trait Insert: Send + Sync + 'static {
        #[doc = " A method to add a new single vector."]
        async fn insert(
            &self,
            request: tonic::Request<super::super::super::payload::v1::insert::Request>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        >;
        #[doc = "Server streaming response type for the StreamInsert method."]
        type StreamInsertStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::object::StreamLocation,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " A method to add new multiple vectors by bidirectional streaming."]
        async fn stream_insert(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::payload::v1::insert::Request>,
            >,
        ) -> Result<tonic::Response<Self::StreamInsertStream>, tonic::Status>;
        #[doc = " A method to add new multiple vectors in a single request."]
        async fn multi_insert(
            &self,
            request: tonic::Request<super::super::super::payload::v1::insert::MultiRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        >;
    }
    #[doc = " Insert service provides ways to add new vectors."]
    #[derive(Debug)]
    pub struct InsertServer<T: Insert> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Insert> InsertServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for InsertServer<T>
    where
        T: Insert,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/vald.v1.Insert/Insert" => {
                    #[allow(non_camel_case_types)]
                    struct InsertSvc<T: Insert>(pub Arc<T>);
                    impl<T: Insert>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::insert::Request,
                        > for InsertSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Location;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::insert::Request,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).insert(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InsertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Insert/StreamInsert" => {
                    #[allow(non_camel_case_types)]
                    struct StreamInsertSvc<T: Insert>(pub Arc<T>);
                    impl<T: Insert>
                        tonic::server::StreamingService<
                            super::super::super::payload::v1::insert::Request,
                        > for StreamInsertSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::StreamLocation;
                        type ResponseStream = T::StreamInsertStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::payload::v1::insert::Request>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_insert(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamInsertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Insert/MultiInsert" => {
                    #[allow(non_camel_case_types)]
                    struct MultiInsertSvc<T: Insert>(pub Arc<T>);
                    impl<T: Insert>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::insert::MultiRequest,
                        > for MultiInsertSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Locations;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::insert::MultiRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).multi_insert(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiInsertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Insert> Clone for InsertServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Insert> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Insert> tonic::transport::NamedService for InsertServer<T> {
        const NAME: &'static str = "vald.v1.Insert";
    }
}
#[doc = r" Generated client implementations."]
pub mod update_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Update service provides ways to update indexed vectors."]
    #[derive(Debug, Clone)]
    pub struct UpdateClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UpdateClient<tonic::transport::Channel> {
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
    impl<T> UpdateClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UpdateClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            UpdateClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " A method to update an indexed vector."]
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::update::Request>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Update/Update");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to update multiple indexed vectors by bidirectional streaming."]
        pub async fn stream_update(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::payload::v1::update::Request,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::object::StreamLocation>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Update/StreamUpdate");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " A method to update multiple indexed vectors in a single request."]
        pub async fn multi_update(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::update::MultiRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Update/MultiUpdate");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod update_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with UpdateServer."]
    #[async_trait]
    pub trait Update: Send + Sync + 'static {
        #[doc = " A method to update an indexed vector."]
        async fn update(
            &self,
            request: tonic::Request<super::super::super::payload::v1::update::Request>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        >;
        #[doc = "Server streaming response type for the StreamUpdate method."]
        type StreamUpdateStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::object::StreamLocation,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " A method to update multiple indexed vectors by bidirectional streaming."]
        async fn stream_update(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::payload::v1::update::Request>,
            >,
        ) -> Result<tonic::Response<Self::StreamUpdateStream>, tonic::Status>;
        #[doc = " A method to update multiple indexed vectors in a single request."]
        async fn multi_update(
            &self,
            request: tonic::Request<super::super::super::payload::v1::update::MultiRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        >;
    }
    #[doc = " Update service provides ways to update indexed vectors."]
    #[derive(Debug)]
    pub struct UpdateServer<T: Update> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Update> UpdateServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UpdateServer<T>
    where
        T: Update,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/vald.v1.Update/Update" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSvc<T: Update>(pub Arc<T>);
                    impl<T: Update>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::update::Request,
                        > for UpdateSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Location;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::update::Request,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Update/StreamUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct StreamUpdateSvc<T: Update>(pub Arc<T>);
                    impl<T: Update>
                        tonic::server::StreamingService<
                            super::super::super::payload::v1::update::Request,
                        > for StreamUpdateSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::StreamLocation;
                        type ResponseStream = T::StreamUpdateStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::payload::v1::update::Request>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Update/MultiUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct MultiUpdateSvc<T: Update>(pub Arc<T>);
                    impl<T: Update>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::update::MultiRequest,
                        > for MultiUpdateSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Locations;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::update::MultiRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).multi_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Update> Clone for UpdateServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Update> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Update> tonic::transport::NamedService for UpdateServer<T> {
        const NAME: &'static str = "vald.v1.Update";
    }
}
#[doc = r" Generated client implementations."]
pub mod search_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Search service provides ways to search indexed vectors."]
    #[derive(Debug, Clone)]
    pub struct SearchClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SearchClient<tonic::transport::Channel> {
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
    impl<T> SearchClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SearchClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SearchClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " A method to search indexed vectors by a raw vector."]
        pub async fn search(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::search::Request>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Response>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Search/Search");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to search indexed vectors by ID."]
        pub async fn search_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::search::IdRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Response>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Search/SearchByID");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to search indexed vectors by multiple vectors."]
        pub async fn stream_search(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::payload::v1::search::Request,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::search::StreamResponse>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Search/StreamSearch");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " A method to search indexed vectors by multiple IDs."]
        pub async fn stream_search_by_id(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::payload::v1::search::IdRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::search::StreamResponse>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Search/StreamSearchByID");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " A method to search indexed vectors by multiple vectors in a single request."]
        pub async fn multi_search(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::search::MultiRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Responses>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Search/MultiSearch");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to search indexed vectors by multiple IDs in a single request."]
        pub async fn multi_search_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::search::MultiIdRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Responses>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Search/MultiSearchByID");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to linear search indexed vectors by a raw vector."]
        pub async fn linear_search(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::search::Request>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Response>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Search/LinearSearch");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to linear search indexed vectors by ID."]
        pub async fn linear_search_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::search::IdRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Response>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Search/LinearSearchByID");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to linear search indexed vectors by multiple vectors."]
        pub async fn stream_linear_search(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::payload::v1::search::Request,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::search::StreamResponse>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Search/StreamLinearSearch");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " A method to linear search indexed vectors by multiple IDs."]
        pub async fn stream_linear_search_by_id(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::payload::v1::search::IdRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::search::StreamResponse>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/vald.v1.Search/StreamLinearSearchByID");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " A method to linear search indexed vectors by multiple vectors in a single"]
        #[doc = " request."]
        pub async fn multi_linear_search(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::search::MultiRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Responses>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Search/MultiLinearSearch");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to linear search indexed vectors by multiple IDs in a single"]
        #[doc = " request."]
        pub async fn multi_linear_search_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::search::MultiIdRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Responses>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/vald.v1.Search/MultiLinearSearchByID");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod search_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SearchServer."]
    #[async_trait]
    pub trait Search: Send + Sync + 'static {
        #[doc = " A method to search indexed vectors by a raw vector."]
        async fn search(
            &self,
            request: tonic::Request<super::super::super::payload::v1::search::Request>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Response>,
            tonic::Status,
        >;
        #[doc = " A method to search indexed vectors by ID."]
        async fn search_by_id(
            &self,
            request: tonic::Request<super::super::super::payload::v1::search::IdRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Response>,
            tonic::Status,
        >;
        #[doc = "Server streaming response type for the StreamSearch method."]
        type StreamSearchStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::search::StreamResponse,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " A method to search indexed vectors by multiple vectors."]
        async fn stream_search(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::payload::v1::search::Request>,
            >,
        ) -> Result<tonic::Response<Self::StreamSearchStream>, tonic::Status>;
        #[doc = "Server streaming response type for the StreamSearchByID method."]
        type StreamSearchByIDStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::search::StreamResponse,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " A method to search indexed vectors by multiple IDs."]
        async fn stream_search_by_id(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::payload::v1::search::IdRequest>,
            >,
        ) -> Result<tonic::Response<Self::StreamSearchByIDStream>, tonic::Status>;
        #[doc = " A method to search indexed vectors by multiple vectors in a single request."]
        async fn multi_search(
            &self,
            request: tonic::Request<super::super::super::payload::v1::search::MultiRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Responses>,
            tonic::Status,
        >;
        #[doc = " A method to search indexed vectors by multiple IDs in a single request."]
        async fn multi_search_by_id(
            &self,
            request: tonic::Request<super::super::super::payload::v1::search::MultiIdRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Responses>,
            tonic::Status,
        >;
        #[doc = " A method to linear search indexed vectors by a raw vector."]
        async fn linear_search(
            &self,
            request: tonic::Request<super::super::super::payload::v1::search::Request>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Response>,
            tonic::Status,
        >;
        #[doc = " A method to linear search indexed vectors by ID."]
        async fn linear_search_by_id(
            &self,
            request: tonic::Request<super::super::super::payload::v1::search::IdRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Response>,
            tonic::Status,
        >;
        #[doc = "Server streaming response type for the StreamLinearSearch method."]
        type StreamLinearSearchStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::search::StreamResponse,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " A method to linear search indexed vectors by multiple vectors."]
        async fn stream_linear_search(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::payload::v1::search::Request>,
            >,
        ) -> Result<tonic::Response<Self::StreamLinearSearchStream>, tonic::Status>;
        #[doc = "Server streaming response type for the StreamLinearSearchByID method."]
        type StreamLinearSearchByIDStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::search::StreamResponse,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " A method to linear search indexed vectors by multiple IDs."]
        async fn stream_linear_search_by_id(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::payload::v1::search::IdRequest>,
            >,
        ) -> Result<tonic::Response<Self::StreamLinearSearchByIDStream>, tonic::Status>;
        #[doc = " A method to linear search indexed vectors by multiple vectors in a single"]
        #[doc = " request."]
        async fn multi_linear_search(
            &self,
            request: tonic::Request<super::super::super::payload::v1::search::MultiRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Responses>,
            tonic::Status,
        >;
        #[doc = " A method to linear search indexed vectors by multiple IDs in a single"]
        #[doc = " request."]
        async fn multi_linear_search_by_id(
            &self,
            request: tonic::Request<super::super::super::payload::v1::search::MultiIdRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Responses>,
            tonic::Status,
        >;
    }
    #[doc = " Search service provides ways to search indexed vectors."]
    #[derive(Debug)]
    pub struct SearchServer<T: Search> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Search> SearchServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SearchServer<T>
    where
        T: Search,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/vald.v1.Search/Search" => {
                    #[allow(non_camel_case_types)]
                    struct SearchSvc<T: Search>(pub Arc<T>);
                    impl<T: Search>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::search::Request,
                        > for SearchSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::search::Request,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).search(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Search/SearchByID" => {
                    #[allow(non_camel_case_types)]
                    struct SearchByIDSvc<T: Search>(pub Arc<T>);
                    impl<T: Search>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::search::IdRequest,
                        > for SearchByIDSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::search::IdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).search_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Search/StreamSearch" => {
                    #[allow(non_camel_case_types)]
                    struct StreamSearchSvc<T: Search>(pub Arc<T>);
                    impl<T: Search>
                        tonic::server::StreamingService<
                            super::super::super::payload::v1::search::Request,
                        > for StreamSearchSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::StreamResponse;
                        type ResponseStream = T::StreamSearchStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::payload::v1::search::Request>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_search(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamSearchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Search/StreamSearchByID" => {
                    #[allow(non_camel_case_types)]
                    struct StreamSearchByIDSvc<T: Search>(pub Arc<T>);
                    impl<T: Search>
                        tonic::server::StreamingService<
                            super::super::super::payload::v1::search::IdRequest,
                        > for StreamSearchByIDSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::StreamResponse;
                        type ResponseStream = T::StreamSearchByIDStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::payload::v1::search::IdRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_search_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamSearchByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Search/MultiSearch" => {
                    #[allow(non_camel_case_types)]
                    struct MultiSearchSvc<T: Search>(pub Arc<T>);
                    impl<T: Search>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::search::MultiRequest,
                        > for MultiSearchSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::Responses;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::search::MultiRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).multi_search(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiSearchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Search/MultiSearchByID" => {
                    #[allow(non_camel_case_types)]
                    struct MultiSearchByIDSvc<T: Search>(pub Arc<T>);
                    impl<T: Search>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::search::MultiIdRequest,
                        > for MultiSearchByIDSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::Responses;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::search::MultiIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).multi_search_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiSearchByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Search/LinearSearch" => {
                    #[allow(non_camel_case_types)]
                    struct LinearSearchSvc<T: Search>(pub Arc<T>);
                    impl<T: Search>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::search::Request,
                        > for LinearSearchSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::search::Request,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).linear_search(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LinearSearchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Search/LinearSearchByID" => {
                    #[allow(non_camel_case_types)]
                    struct LinearSearchByIDSvc<T: Search>(pub Arc<T>);
                    impl<T: Search>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::search::IdRequest,
                        > for LinearSearchByIDSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::search::IdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).linear_search_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LinearSearchByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Search/StreamLinearSearch" => {
                    #[allow(non_camel_case_types)]
                    struct StreamLinearSearchSvc<T: Search>(pub Arc<T>);
                    impl<T: Search>
                        tonic::server::StreamingService<
                            super::super::super::payload::v1::search::Request,
                        > for StreamLinearSearchSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::StreamResponse;
                        type ResponseStream = T::StreamLinearSearchStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::payload::v1::search::Request>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_linear_search(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamLinearSearchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Search/StreamLinearSearchByID" => {
                    #[allow(non_camel_case_types)]
                    struct StreamLinearSearchByIDSvc<T: Search>(pub Arc<T>);
                    impl<T: Search>
                        tonic::server::StreamingService<
                            super::super::super::payload::v1::search::IdRequest,
                        > for StreamLinearSearchByIDSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::StreamResponse;
                        type ResponseStream = T::StreamLinearSearchByIDStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::payload::v1::search::IdRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).stream_linear_search_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamLinearSearchByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Search/MultiLinearSearch" => {
                    #[allow(non_camel_case_types)]
                    struct MultiLinearSearchSvc<T: Search>(pub Arc<T>);
                    impl<T: Search>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::search::MultiRequest,
                        > for MultiLinearSearchSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::Responses;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::search::MultiRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).multi_linear_search(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiLinearSearchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Search/MultiLinearSearchByID" => {
                    #[allow(non_camel_case_types)]
                    struct MultiLinearSearchByIDSvc<T: Search>(pub Arc<T>);
                    impl<T: Search>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::search::MultiIdRequest,
                        > for MultiLinearSearchByIDSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::Responses;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::search::MultiIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).multi_linear_search_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiLinearSearchByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Search> Clone for SearchServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Search> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Search> tonic::transport::NamedService for SearchServer<T> {
        const NAME: &'static str = "vald.v1.Search";
    }
}
#[doc = r" Generated client implementations."]
pub mod filter_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Filter service provides ways to connect to Vald through filter."]
    #[derive(Debug, Clone)]
    pub struct FilterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FilterClient<tonic::transport::Channel> {
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
    impl<T> FilterClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> FilterClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            FilterClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " A method to search object."]
        pub async fn search_object(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::search::ObjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Response>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Filter/SearchObject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to search multiple objects."]
        pub async fn multi_search_object(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::payload::v1::search::MultiObjectRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Responses>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Filter/MultiSearchObject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to search object by bidirectional streaming."]
        pub async fn stream_search_object(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::payload::v1::search::ObjectRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::search::StreamResponse>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Filter/StreamSearchObject");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " A method insert object."]
        pub async fn insert_object(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::insert::ObjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Filter/InsertObject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Represent the streaming RPC to insert object by bidirectional streaming."]
        pub async fn stream_insert_object(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::payload::v1::insert::ObjectRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::object::StreamLocation>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Filter/StreamInsertObject");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " A method to insert multiple objects."]
        pub async fn multi_insert_object(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::payload::v1::insert::MultiObjectRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Filter/MultiInsertObject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to update object."]
        pub async fn update_object(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::update::ObjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Filter/UpdateObject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to update object by bidirectional streaming."]
        pub async fn stream_update_object(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::payload::v1::update::ObjectRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::object::StreamLocation>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Filter/StreamUpdateObject");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " A method to update multiple objects."]
        pub async fn multi_update_object(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::payload::v1::update::MultiObjectRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Filter/MultiUpdateObject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to upsert object."]
        pub async fn upsert_object(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::upsert::ObjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Filter/UpsertObject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to upsert object by bidirectional streaming."]
        pub async fn stream_upsert_object(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::payload::v1::upsert::ObjectRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::object::StreamLocation>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Filter/StreamUpsertObject");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " A method to upsert multiple objects."]
        pub async fn multi_upsert_object(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::payload::v1::upsert::MultiObjectRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Filter/MultiUpsertObject");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod filter_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with FilterServer."]
    #[async_trait]
    pub trait Filter: Send + Sync + 'static {
        #[doc = " A method to search object."]
        async fn search_object(
            &self,
            request: tonic::Request<super::super::super::payload::v1::search::ObjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Response>,
            tonic::Status,
        >;
        #[doc = " A method to search multiple objects."]
        async fn multi_search_object(
            &self,
            request: tonic::Request<super::super::super::payload::v1::search::MultiObjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::search::Responses>,
            tonic::Status,
        >;
        #[doc = "Server streaming response type for the StreamSearchObject method."]
        type StreamSearchObjectStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::search::StreamResponse,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " A method to search object by bidirectional streaming."]
        async fn stream_search_object(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::payload::v1::search::ObjectRequest>,
            >,
        ) -> Result<tonic::Response<Self::StreamSearchObjectStream>, tonic::Status>;
        #[doc = " A method insert object."]
        async fn insert_object(
            &self,
            request: tonic::Request<super::super::super::payload::v1::insert::ObjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        >;
        #[doc = "Server streaming response type for the StreamInsertObject method."]
        type StreamInsertObjectStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::object::StreamLocation,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " Represent the streaming RPC to insert object by bidirectional streaming."]
        async fn stream_insert_object(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::payload::v1::insert::ObjectRequest>,
            >,
        ) -> Result<tonic::Response<Self::StreamInsertObjectStream>, tonic::Status>;
        #[doc = " A method to insert multiple objects."]
        async fn multi_insert_object(
            &self,
            request: tonic::Request<super::super::super::payload::v1::insert::MultiObjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        >;
        #[doc = " A method to update object."]
        async fn update_object(
            &self,
            request: tonic::Request<super::super::super::payload::v1::update::ObjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        >;
        #[doc = "Server streaming response type for the StreamUpdateObject method."]
        type StreamUpdateObjectStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::object::StreamLocation,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " A method to update object by bidirectional streaming."]
        async fn stream_update_object(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::payload::v1::update::ObjectRequest>,
            >,
        ) -> Result<tonic::Response<Self::StreamUpdateObjectStream>, tonic::Status>;
        #[doc = " A method to update multiple objects."]
        async fn multi_update_object(
            &self,
            request: tonic::Request<super::super::super::payload::v1::update::MultiObjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        >;
        #[doc = " A method to upsert object."]
        async fn upsert_object(
            &self,
            request: tonic::Request<super::super::super::payload::v1::upsert::ObjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        >;
        #[doc = "Server streaming response type for the StreamUpsertObject method."]
        type StreamUpsertObjectStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::object::StreamLocation,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " A method to upsert object by bidirectional streaming."]
        async fn stream_upsert_object(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::payload::v1::upsert::ObjectRequest>,
            >,
        ) -> Result<tonic::Response<Self::StreamUpsertObjectStream>, tonic::Status>;
        #[doc = " A method to upsert multiple objects."]
        async fn multi_upsert_object(
            &self,
            request: tonic::Request<super::super::super::payload::v1::upsert::MultiObjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        >;
    }
    #[doc = " Filter service provides ways to connect to Vald through filter."]
    #[derive(Debug)]
    pub struct FilterServer<T: Filter> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Filter> FilterServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FilterServer<T>
    where
        T: Filter,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/vald.v1.Filter/SearchObject" => {
                    #[allow(non_camel_case_types)]
                    struct SearchObjectSvc<T: Filter>(pub Arc<T>);
                    impl<T: Filter>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::search::ObjectRequest,
                        > for SearchObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::Response;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::search::ObjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).search_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Filter/MultiSearchObject" => {
                    #[allow(non_camel_case_types)]
                    struct MultiSearchObjectSvc<T: Filter>(pub Arc<T>);
                    impl<T: Filter>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::search::MultiObjectRequest,
                        > for MultiSearchObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::Responses;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::search::MultiObjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).multi_search_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiSearchObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Filter/StreamSearchObject" => {
                    #[allow(non_camel_case_types)]
                    struct StreamSearchObjectSvc<T: Filter>(pub Arc<T>);
                    impl<T: Filter>
                        tonic::server::StreamingService<
                            super::super::super::payload::v1::search::ObjectRequest,
                        > for StreamSearchObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::search::StreamResponse;
                        type ResponseStream = T::StreamSearchObjectStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::payload::v1::search::ObjectRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_search_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamSearchObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Filter/InsertObject" => {
                    #[allow(non_camel_case_types)]
                    struct InsertObjectSvc<T: Filter>(pub Arc<T>);
                    impl<T: Filter>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::insert::ObjectRequest,
                        > for InsertObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Location;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::insert::ObjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).insert_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InsertObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Filter/StreamInsertObject" => {
                    #[allow(non_camel_case_types)]
                    struct StreamInsertObjectSvc<T: Filter>(pub Arc<T>);
                    impl<T: Filter>
                        tonic::server::StreamingService<
                            super::super::super::payload::v1::insert::ObjectRequest,
                        > for StreamInsertObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::StreamLocation;
                        type ResponseStream = T::StreamInsertObjectStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::payload::v1::insert::ObjectRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_insert_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamInsertObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Filter/MultiInsertObject" => {
                    #[allow(non_camel_case_types)]
                    struct MultiInsertObjectSvc<T: Filter>(pub Arc<T>);
                    impl<T: Filter>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::insert::MultiObjectRequest,
                        > for MultiInsertObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Locations;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::insert::MultiObjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).multi_insert_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiInsertObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Filter/UpdateObject" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateObjectSvc<T: Filter>(pub Arc<T>);
                    impl<T: Filter>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::update::ObjectRequest,
                        > for UpdateObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Location;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::update::ObjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Filter/StreamUpdateObject" => {
                    #[allow(non_camel_case_types)]
                    struct StreamUpdateObjectSvc<T: Filter>(pub Arc<T>);
                    impl<T: Filter>
                        tonic::server::StreamingService<
                            super::super::super::payload::v1::update::ObjectRequest,
                        > for StreamUpdateObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::StreamLocation;
                        type ResponseStream = T::StreamUpdateObjectStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::payload::v1::update::ObjectRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_update_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamUpdateObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Filter/MultiUpdateObject" => {
                    #[allow(non_camel_case_types)]
                    struct MultiUpdateObjectSvc<T: Filter>(pub Arc<T>);
                    impl<T: Filter>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::update::MultiObjectRequest,
                        > for MultiUpdateObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Locations;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::update::MultiObjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).multi_update_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiUpdateObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Filter/UpsertObject" => {
                    #[allow(non_camel_case_types)]
                    struct UpsertObjectSvc<T: Filter>(pub Arc<T>);
                    impl<T: Filter>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::upsert::ObjectRequest,
                        > for UpsertObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Location;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::upsert::ObjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).upsert_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpsertObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Filter/StreamUpsertObject" => {
                    #[allow(non_camel_case_types)]
                    struct StreamUpsertObjectSvc<T: Filter>(pub Arc<T>);
                    impl<T: Filter>
                        tonic::server::StreamingService<
                            super::super::super::payload::v1::upsert::ObjectRequest,
                        > for StreamUpsertObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::StreamLocation;
                        type ResponseStream = T::StreamUpsertObjectStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::payload::v1::upsert::ObjectRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_upsert_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamUpsertObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Filter/MultiUpsertObject" => {
                    #[allow(non_camel_case_types)]
                    struct MultiUpsertObjectSvc<T: Filter>(pub Arc<T>);
                    impl<T: Filter>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::upsert::MultiObjectRequest,
                        > for MultiUpsertObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Locations;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::upsert::MultiObjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).multi_upsert_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiUpsertObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Filter> Clone for FilterServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Filter> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Filter> tonic::transport::NamedService for FilterServer<T> {
        const NAME: &'static str = "vald.v1.Filter";
    }
}
#[doc = r" Generated client implementations."]
pub mod object_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Object service provides ways to fetch indexed vectors."]
    #[derive(Debug, Clone)]
    pub struct ObjectClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ObjectClient<tonic::transport::Channel> {
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
    impl<T> ObjectClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ObjectClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ObjectClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " A method to check whether a specified ID is indexed or not."]
        pub async fn exists(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::object::Id>,
        ) -> Result<tonic::Response<super::super::super::payload::v1::object::Id>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Object/Exists");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to fetch a vector."]
        pub async fn get_object(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::object::VectorRequest>,
        ) -> Result<tonic::Response<super::super::super::payload::v1::object::Vector>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Object/GetObject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to fetch vectors by bidirectional streaming."]
        pub async fn stream_get_object(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::payload::v1::object::VectorRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::object::StreamVector>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Object/StreamGetObject");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " A method to get all the vectors with server streaming"]
        pub async fn stream_list_object(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::object::list::Request>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::object::list::Response>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Object/StreamListObject");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod object_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ObjectServer."]
    #[async_trait]
    pub trait Object: Send + Sync + 'static {
        #[doc = " A method to check whether a specified ID is indexed or not."]
        async fn exists(
            &self,
            request: tonic::Request<super::super::super::payload::v1::object::Id>,
        ) -> Result<tonic::Response<super::super::super::payload::v1::object::Id>, tonic::Status>;
        #[doc = " A method to fetch a vector."]
        async fn get_object(
            &self,
            request: tonic::Request<super::super::super::payload::v1::object::VectorRequest>,
        ) -> Result<tonic::Response<super::super::super::payload::v1::object::Vector>, tonic::Status>;
        #[doc = "Server streaming response type for the StreamGetObject method."]
        type StreamGetObjectStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::object::StreamVector,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " A method to fetch vectors by bidirectional streaming."]
        async fn stream_get_object(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::payload::v1::object::VectorRequest>,
            >,
        ) -> Result<tonic::Response<Self::StreamGetObjectStream>, tonic::Status>;
        #[doc = "Server streaming response type for the StreamListObject method."]
        type StreamListObjectStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::object::list::Response,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " A method to get all the vectors with server streaming"]
        async fn stream_list_object(
            &self,
            request: tonic::Request<super::super::super::payload::v1::object::list::Request>,
        ) -> Result<tonic::Response<Self::StreamListObjectStream>, tonic::Status>;
    }
    #[doc = " Object service provides ways to fetch indexed vectors."]
    #[derive(Debug)]
    pub struct ObjectServer<T: Object> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Object> ObjectServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ObjectServer<T>
    where
        T: Object,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/vald.v1.Object/Exists" => {
                    #[allow(non_camel_case_types)]
                    struct ExistsSvc<T: Object>(pub Arc<T>);
                    impl<T: Object>
                        tonic::server::UnaryService<super::super::super::payload::v1::object::Id>
                        for ExistsSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Id;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::super::payload::v1::object::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).exists(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExistsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Object/GetObject" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectSvc<T: Object>(pub Arc<T>);
                    impl<T: Object>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::object::VectorRequest,
                        > for GetObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Vector;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::object::VectorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Object/StreamGetObject" => {
                    #[allow(non_camel_case_types)]
                    struct StreamGetObjectSvc<T: Object>(pub Arc<T>);
                    impl<T: Object>
                        tonic::server::StreamingService<
                            super::super::super::payload::v1::object::VectorRequest,
                        > for StreamGetObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::StreamVector;
                        type ResponseStream = T::StreamGetObjectStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::payload::v1::object::VectorRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_get_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamGetObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Object/StreamListObject" => {
                    #[allow(non_camel_case_types)]
                    struct StreamListObjectSvc<T: Object>(pub Arc<T>);
                    impl<T: Object>
                        tonic::server::ServerStreamingService<
                            super::super::super::payload::v1::object::list::Request,
                        > for StreamListObjectSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::list::Response;
                        type ResponseStream = T::StreamListObjectStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::object::list::Request,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_list_object(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamListObjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Object> Clone for ObjectServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Object> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Object> tonic::transport::NamedService for ObjectServer<T> {
        const NAME: &'static str = "vald.v1.Object";
    }
}
#[doc = r" Generated client implementations."]
pub mod remove_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Remove service provides ways to remove indexed vectors."]
    #[derive(Debug, Clone)]
    pub struct RemoveClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RemoveClient<tonic::transport::Channel> {
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
    impl<T> RemoveClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RemoveClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            RemoveClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " A method to remove an indexed vector."]
        pub async fn remove(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::remove::Request>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Remove/Remove");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to remove an indexed vector based on timestamp."]
        pub async fn remove_by_timestamp(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::remove::TimestampRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Remove/RemoveByTimestamp");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to remove multiple indexed vectors by bidirectional streaming."]
        pub async fn stream_remove(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::payload::v1::remove::Request,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::object::StreamLocation>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Remove/StreamRemove");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " A method to remove multiple indexed vectors in a single request."]
        pub async fn multi_remove(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::remove::MultiRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Remove/MultiRemove");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod remove_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RemoveServer."]
    #[async_trait]
    pub trait Remove: Send + Sync + 'static {
        #[doc = " A method to remove an indexed vector."]
        async fn remove(
            &self,
            request: tonic::Request<super::super::super::payload::v1::remove::Request>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        >;
        #[doc = " A method to remove an indexed vector based on timestamp."]
        async fn remove_by_timestamp(
            &self,
            request: tonic::Request<super::super::super::payload::v1::remove::TimestampRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        >;
        #[doc = "Server streaming response type for the StreamRemove method."]
        type StreamRemoveStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::object::StreamLocation,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " A method to remove multiple indexed vectors by bidirectional streaming."]
        async fn stream_remove(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::payload::v1::remove::Request>,
            >,
        ) -> Result<tonic::Response<Self::StreamRemoveStream>, tonic::Status>;
        #[doc = " A method to remove multiple indexed vectors in a single request."]
        async fn multi_remove(
            &self,
            request: tonic::Request<super::super::super::payload::v1::remove::MultiRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        >;
    }
    #[doc = " Remove service provides ways to remove indexed vectors."]
    #[derive(Debug)]
    pub struct RemoveServer<T: Remove> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Remove> RemoveServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RemoveServer<T>
    where
        T: Remove,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/vald.v1.Remove/Remove" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveSvc<T: Remove>(pub Arc<T>);
                    impl<T: Remove>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::remove::Request,
                        > for RemoveSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Location;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::remove::Request,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Remove/RemoveByTimestamp" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveByTimestampSvc<T: Remove>(pub Arc<T>);
                    impl<T: Remove>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::remove::TimestampRequest,
                        > for RemoveByTimestampSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Locations;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::remove::TimestampRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_by_timestamp(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveByTimestampSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Remove/StreamRemove" => {
                    #[allow(non_camel_case_types)]
                    struct StreamRemoveSvc<T: Remove>(pub Arc<T>);
                    impl<T: Remove>
                        tonic::server::StreamingService<
                            super::super::super::payload::v1::remove::Request,
                        > for StreamRemoveSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::StreamLocation;
                        type ResponseStream = T::StreamRemoveStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::payload::v1::remove::Request>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_remove(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamRemoveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Remove/MultiRemove" => {
                    #[allow(non_camel_case_types)]
                    struct MultiRemoveSvc<T: Remove>(pub Arc<T>);
                    impl<T: Remove>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::remove::MultiRequest,
                        > for MultiRemoveSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Locations;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::remove::MultiRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).multi_remove(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiRemoveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Remove> Clone for RemoveServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Remove> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Remove> tonic::transport::NamedService for RemoveServer<T> {
        const NAME: &'static str = "vald.v1.Remove";
    }
}
#[doc = r" Generated client implementations."]
pub mod upsert_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Upsert service provides ways to insert/update vectors."]
    #[derive(Debug, Clone)]
    pub struct UpsertClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UpsertClient<tonic::transport::Channel> {
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
    impl<T> UpsertClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UpsertClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            UpsertClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " A method to insert/update a vector."]
        pub async fn upsert(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::upsert::Request>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Upsert/Upsert");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A method to insert/update multiple vectors by bidirectional streaming."]
        pub async fn stream_upsert(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::payload::v1::upsert::Request,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::payload::v1::object::StreamLocation>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Upsert/StreamUpsert");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " A method to insert/update multiple vectors in a single request."]
        pub async fn multi_upsert(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::payload::v1::upsert::MultiRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/vald.v1.Upsert/MultiUpsert");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod upsert_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with UpsertServer."]
    #[async_trait]
    pub trait Upsert: Send + Sync + 'static {
        #[doc = " A method to insert/update a vector."]
        async fn upsert(
            &self,
            request: tonic::Request<super::super::super::payload::v1::upsert::Request>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Location>,
            tonic::Status,
        >;
        #[doc = "Server streaming response type for the StreamUpsert method."]
        type StreamUpsertStream: futures_core::Stream<
                Item = Result<
                    super::super::super::payload::v1::object::StreamLocation,
                    tonic::Status,
                >,
            > + Send
            + 'static;
        #[doc = " A method to insert/update multiple vectors by bidirectional streaming."]
        async fn stream_upsert(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::payload::v1::upsert::Request>,
            >,
        ) -> Result<tonic::Response<Self::StreamUpsertStream>, tonic::Status>;
        #[doc = " A method to insert/update multiple vectors in a single request."]
        async fn multi_upsert(
            &self,
            request: tonic::Request<super::super::super::payload::v1::upsert::MultiRequest>,
        ) -> Result<
            tonic::Response<super::super::super::payload::v1::object::Locations>,
            tonic::Status,
        >;
    }
    #[doc = " Upsert service provides ways to insert/update vectors."]
    #[derive(Debug)]
    pub struct UpsertServer<T: Upsert> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Upsert> UpsertServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UpsertServer<T>
    where
        T: Upsert,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/vald.v1.Upsert/Upsert" => {
                    #[allow(non_camel_case_types)]
                    struct UpsertSvc<T: Upsert>(pub Arc<T>);
                    impl<T: Upsert>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::upsert::Request,
                        > for UpsertSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Location;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::upsert::Request,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).upsert(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpsertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Upsert/StreamUpsert" => {
                    #[allow(non_camel_case_types)]
                    struct StreamUpsertSvc<T: Upsert>(pub Arc<T>);
                    impl<T: Upsert>
                        tonic::server::StreamingService<
                            super::super::super::payload::v1::upsert::Request,
                        > for StreamUpsertSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::StreamLocation;
                        type ResponseStream = T::StreamUpsertStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::super::payload::v1::upsert::Request>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_upsert(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamUpsertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/vald.v1.Upsert/MultiUpsert" => {
                    #[allow(non_camel_case_types)]
                    struct MultiUpsertSvc<T: Upsert>(pub Arc<T>);
                    impl<T: Upsert>
                        tonic::server::UnaryService<
                            super::super::super::payload::v1::upsert::MultiRequest,
                        > for MultiUpsertSvc<T>
                    {
                        type Response = super::super::super::payload::v1::object::Locations;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::payload::v1::upsert::MultiRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).multi_upsert(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiUpsertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Upsert> Clone for UpsertServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Upsert> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Upsert> tonic::transport::NamedService for UpsertServer<T> {
        const NAME: &'static str = "vald.v1.Upsert";
    }
}
