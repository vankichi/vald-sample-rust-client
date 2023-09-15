PROTO_DIR = "./proto"
VALD = "vald"
PAYLOAD = "payload"
VALD_URL = "https://github.com/vdaas/${VALD}"
GOOGLE_API = "googleapis"
GOOGLE_PROTO = "google"
GOOGLE_API_URL = "https://github.com/googleapis/${GOOGLE_API}"
PROTOC_GEN_VALIDATE = "protoc-gen-validate"
PROTOC_GEN_VALIDATE_URL = "https://github.com/envoyproxy/${PROTOC_GEN_VALIDATE}"
VTPROTOBUF = "vtprotobuf"
VTPROTOBUF_URL = "https://github.com/planetscale/${VTPROTOBUF}"

.PHONY: fix/import/path
fix/import/path:
	@find proto/vald -type f -name "*.proto" | xargs sed -i "s/apis\/proto\/v1\///g"
	@find proto/vald -type f -name "*.proto" | xargs sed -i "s/github\.com\/googleapis\/googleapis\///g"
	@find proto/vald -type f -name "*.proto" | xargs sed -i "s/github\.com\/planetscale\/vtprotobuf\/include/vtprotobuf\/include/g"
	@find proto/payload -type f -name "*.proto" | xargs sed -i "s/github\.com\/googleapis\/googleapis\///g"
	@find proto/payload -type f -name "*.proto" | xargs sed -i "s/github\.com\/envoyproxy\///g"
	@find proto/payload -type f -name "*.proto" | xargs sed -i "s/github\.com\/planetscale\/vtprotobuf\/include/vtprotobuf\/include/g"

.PHONY: sync/proto/vald
sync/proto/vald:
	@git clone ${VALD_URL}
	@rm -rf ${PROTO_DIR}/${VALD} ${PROTO_DIR}/${PAYLOAD}
	@cp -R ${VALD}/apis/proto/v1/vald ${PROTO_DIR}/${VALD}
	@cp -R ${VALD}/apis/proto/v1/payload ${PROTO_DIR}/${PAYLOAD}
	@rm -rf ${VALD}

.PHONY: sync/proto/googleapis
sync/proto/googleapis:
	@rm -rf ${PROTO_DIR}/${GOOGLE_PROTO}
	@git clone ${GOOGLE_API_URL}
	@cp -R ${GOOGLE_API}/${GOOGLE_PROTO} ${PROTO_DIR}/${GOOGLE_PROTO}
	@rm -rf ${GOOGLE_API}

.PHONY: sync/proto/protoc-gen-validate
sync/proto/protoc-gen-validate:
	@rm -rf ${PROTO_DIR}/${PROTOC_GEN_VALIDATE}
	@git clone ${PROTOC_GEN_VALIDATE_URL}
	@mv ${PROTOC_GEN_VALIDATE} ${PROTO_DIR}/${PROTOC_GEN_VALIDATE}

.PHONY: sync/proto/vtproto
sync/proto/vtproto:
	@rm -rf ${PROTO_DIR}/${VTPROTOBUF}
	@git clone ${VTPROTOBUF_URL}
	@mv ${VTPROTOBUF} ${PROTO_DIR}/${VTPROTOBUF}


.PHONY: init
init:
	@rustup update stable
	@mkdir -p proto
	@$(MAKE) sync/proto/vald
	@$(MAKE) sync/proto/googleapis
	@$(MAKE) sync/proto/protoc-gen-validate
	@$(MAKE) sync/proto/vtproto
	@$(MAKE) fix/import/path
	@cargo build
	@wget http://ann-benchmarks.com/fashion-mnist-784-euclidean.hdf5

.PHONY: update
update:
	@rustup update stable
	@mkdir -p proto
	@$(MAKE) sync/proto/vald
	@$(MAKE) sync/proto/googleapis
	@$(MAKE) sync/proto/protoc-gen-validate
	@$(MAKE) sync/proto/vtproto
	@$(MAKE) fix/import/path
	@cargo build

.PHONY: run
run:
	@cargo run src/client.rs
