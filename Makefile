PROTO_DIR = "./proto"
GOOGLE_API = "googleapis"
GOOGLE_API_URL = "https://github.com/googleapis/googleapis"

.PHONY: fix/import/path
fix/import/path:
	@find proto/vald -type f -name "*.proto" | xargs sed -i "s/apis\/proto\/v1\///g"
	@find proto/vald -type f -name "*.proto" | xargs sed -i "s/github\.com\/googleapis\/googleapis\///g"
	@find proto/payload -type f -name "*.proto" | xargs sed -i "s/github\.com\/googleapis\/googleapis\///g"
	@find proto/payload -type f -name "*.proto" | xargs sed -i "s/github\.com\/envoyproxy\///g"

.PHONY: sync/proto/vald
sync/proto/vald:
	@git clone https://github.com/vdaas/vald
	@cp -R vald/apis/proto/v1/vald proto/vald
	@cp -R vald/apis/proto/v1/payload proto/payload
	@rm -rf vald

.PHONY: sync/proto/googleapis
sync/proto/googleapis:
	@rm -rf ${PROTO_DIR}/${GOOGLE_API}
	@git clone ${GOOGLE_API_URL}
	@cp -R ${GOOGLE_API}/google proto/${GOOGLE_API}
	@rm -rf ${GOOGLE_API}

.PHONY: sync/proto/protoc-gen-validate
sync/proto/protoc-gen-validate:
	@git clone https://github.com/envoyproxy/protoc-gen-validate
	@mv protoc-gen-validate proto/protoc-gen-validate

.PHONY: init
init:
	@rustup update stable
	@mkdir -p proto
	@$(call sync/proto/vald)
	@$(call sync/proto/googleapis)
	@$(call sync/proto/protoc-gen-validate)
	@$(call fix/import/path)
	@cargo build
	@wget http://ann-benchmarks.com/fashion-mnist-784-euclidean.hdf5

.PHONY: run
run:
	@cargo run src/client.rs
