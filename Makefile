PROTO_DIR = "./proto"
GOOGLE_API = "googleapis"
GOOGLE_API_URL = "https://github.com/googleapis/googleapis"

.PHONY: fix/import/path
fix/import/path:
	@find proto/vald -type f -name "*.proto" | xargs sed -i "s/apis\/proto\/v1\///g"
	@find proto/vald -type f -name "*.proto" | xargs sed -i "s/github\.com\/googleapis\/googleapis\///g"
	@find proto/payload -type f -name "*.proto" | xargs sed -i "s/github\.com\/googleapis\/googleapis\///g"
	@find proto/payload -type f -name "*.proto" | xargs sed -i "s/github\.com\/envoyproxy\///g"

.PHONY: build/proto
build/proto:
	@cargo build

.PHONY: sync/googleapis
sync/googleapis:
	@rm -rf ${PROTO_DIR}/${GOOGLE_API}
	@git clone ${GOOGLE_API_URL}
	@cp -R ${GOOGLE_API}/google proto/${GOOGLE_API}
	@rm -rf ${GOOGLE_API}
