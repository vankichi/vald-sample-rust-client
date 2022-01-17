# Problem

```bash
vankichi:/home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client $ cargo build                                                                                                              (git)-[]-+[main] (current-context is not set)
   Compiling vald-sample-rust-client v0.1.0 (/home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client)
error[E0433]: failed to resolve: there are too many leading `super` keywords
  --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:65:60
   |
65 |             request: impl tonic::IntoRequest<super::super::super::payload::v1::insert::Request>,
   |                                                            ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
  --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:67:43
   |
67 |             tonic::Response<super::super::super::payload::v1::object::Location>,
   |                                           ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
  --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:84:41
   |
84 |                 Message = super::super::super::payload::v1::insert::Request,
   |                                         ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
  --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:88:55
   |
88 |                 tonic::codec::Streaming<super::super::super::payload::v1::object::StreamLocation>,
   |                                                       ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:107:60
    |
107 |             request: impl tonic::IntoRequest<super::super::super::payload::v1::insert::MultiRequest>,
    |                                                            ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:109:43
    |
109 |             tonic::Response<super::super::super::payload::v1::object::Locations>,
    |                                           ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:134:51
    |
134 |             request: tonic::Request<super::super::super::payload::v1::insert::Request>,
    |                                                   ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:136:43
    |
136 |             tonic::Response<super::super::super::payload::v1::object::Location>,
    |                                           ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:142:35
    |
142 |                     super::super::super::payload::v1::object::StreamLocation,
    |                                   ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:151:48
    |
151 |                 tonic::Streaming<super::super::super::payload::v1::insert::Request>,
    |                                                ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:157:51
    |
157 |             request: tonic::Request<super::super::super::payload::v1::insert::MultiRequest>,
    |                                                   ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:159:43
    |
159 |             tonic::Response<super::super::super::payload::v1::object::Locations>,
    |                                           ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:208:43
    |
208 | ...                   super::super::super::payload::v1::insert::Request,
    |                                     ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:211:55
    |
211 |                         type Response = super::super::super::payload::v1::object::Location;
    |                                                       ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:216:47
    |
216 | ...                   super::super::super::payload::v1::insert::Request,
    |                                     ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:245:43
    |
245 | ...                   super::super::super::payload::v1::insert::Request,
    |                                     ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:248:55
    |
248 |                         type Response = super::super::super::payload::v1::object::StreamLocation;
    |                                                       ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:255:64
    |
255 | ...                   tonic::Streaming<super::super::super::payload::v1::insert::Request>,
    |                                                      ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:284:43
    |
284 | ...                   super::super::super::payload::v1::insert::MultiRequest,
    |                                     ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:287:55
    |
287 |                         type Response = super::super::super::payload::v1::object::Locations;
    |                                                       ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:292:47
    |
292 | ...                   super::super::super::payload::v1::insert::MultiRequest,
    |                                     ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:415:60
    |
415 |             request: impl tonic::IntoRequest<super::super::super::payload::v1::upsert::Request>,
    |                                                            ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:417:43
    |
417 |             tonic::Response<super::super::super::payload::v1::object::Location>,
    |                                           ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:434:41
    |
434 |                 Message = super::super::super::payload::v1::upsert::Request,
    |                                         ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:438:55
    |
438 |                 tonic::codec::Streaming<super::super::super::payload::v1::object::StreamLocation>,
    |                                                       ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:457:60
    |
457 |             request: impl tonic::IntoRequest<super::super::super::payload::v1::upsert::MultiRequest>,
    |                                                            ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:459:43
    |
459 |             tonic::Response<super::super::super::payload::v1::object::Locations>,
    |                                           ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:484:51
    |
484 |             request: tonic::Request<super::super::super::payload::v1::upsert::Request>,
    |                                                   ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:486:43
    |
486 |             tonic::Response<super::super::super::payload::v1::object::Location>,
    |                                           ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:492:35
    |
492 |                     super::super::super::payload::v1::object::StreamLocation,
    |                                   ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:501:48
    |
501 |                 tonic::Streaming<super::super::super::payload::v1::upsert::Request>,
    |                                                ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:507:51
    |
507 |             request: tonic::Request<super::super::super::payload::v1::upsert::MultiRequest>,
    |                                                   ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:509:43
    |
509 |             tonic::Response<super::super::super::payload::v1::object::Locations>,
    |                                           ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:558:43
    |
558 | ...                   super::super::super::payload::v1::upsert::Request,
    |                                     ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:561:55
    |
561 |                         type Response = super::super::super::payload::v1::object::Location;
    |                                                       ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:566:47
    |
566 | ...                   super::super::super::payload::v1::upsert::Request,
    |                                     ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:595:43
    |
595 | ...                   super::super::super::payload::v1::upsert::Request,
    |                                     ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:598:55
    |
598 |                         type Response = super::super::super::payload::v1::object::StreamLocation;
    |                                                       ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:605:64
    |
605 | ...                   tonic::Streaming<super::super::super::payload::v1::upsert::Request>,
    |                                                      ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:634:43
    |
634 | ...                   super::super::super::payload::v1::upsert::MultiRequest,
    |                                     ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:637:55
    |
637 |                         type Response = super::super::super::payload::v1::object::Locations;
    |                                                       ^^^^^ there are too many leading `super` keywords

error[E0433]: failed to resolve: there are too many leading `super` keywords
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:642:47
    |
642 | ...                   super::super::super::payload::v1::upsert::MultiRequest,
    |                                     ^^^^^ there are too many leading `super` keywords

error[E0658]: `&mut InsertClient<T>` cannot be used as the type of `self` without the `arbitrary_self_types` feature
  --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:64:13
   |
64 |             &mut self,
   |             ^^^^^^^^^
   |
   = note: see issue #44874 <https://github.com/rust-lang/rust/issues/44874> for more information
   = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)

error[E0658]: `&mut InsertClient<T>` cannot be used as the type of `self` without the `arbitrary_self_types` feature
  --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:82:13
   |
82 |             &mut self,
   |             ^^^^^^^^^
   |
   = note: see issue #44874 <https://github.com/rust-lang/rust/issues/44874> for more information
   = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)

error[E0658]: `&mut InsertClient<T>` cannot be used as the type of `self` without the `arbitrary_self_types` feature
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:106:13
    |
106 |             &mut self,
    |             ^^^^^^^^^
    |
    = note: see issue #44874 <https://github.com/rust-lang/rust/issues/44874> for more information
    = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)

error[E0658]: `&mut UpsertClient<T>` cannot be used as the type of `self` without the `arbitrary_self_types` feature
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:414:13
    |
414 |             &mut self,
    |             ^^^^^^^^^
    |
    = note: see issue #44874 <https://github.com/rust-lang/rust/issues/44874> for more information
    = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)

error[E0658]: `&mut UpsertClient<T>` cannot be used as the type of `self` without the `arbitrary_self_types` feature
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:432:13
    |
432 |             &mut self,
    |             ^^^^^^^^^
    |
    = note: see issue #44874 <https://github.com/rust-lang/rust/issues/44874> for more information
    = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)

error[E0658]: `&mut UpsertClient<T>` cannot be used as the type of `self` without the `arbitrary_self_types` feature
   --> /home/vankichi/go/src/github.com/vankichi/vald-sample-rust-client/target/debug/build/vald-sample-rust-client-6efae29d89bec62c/out/vald.v1.rs:456:13
    |
456 |             &mut self,
    |             ^^^^^^^^^
    |
    = note: see issue #44874 <https://github.com/rust-lang/rust/issues/44874> for more information
    = help: consider changing to `self`, `&self`, `&mut self`, `self: Box<Self>`, `self: Rc<Self>`, `self: Arc<Self>`, or `self: Pin<P>` (where P is one of the previous types except `Self`)

Some errors have detailed explanations: E0433, E0658.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `vald-sample-rust-client` due to 48 previous errors
```
