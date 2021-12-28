.DEFAULT_GOAL    := help

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
IMAGE="pulsar256/matrix2mqtt"

build: ## builds the binary (./target/release/ws-to-mqtt)
	cargo build --release

docker_image: ## builds a docker container
	docker build . -t $(IMAGE)

push_docker: docker_image ## builds and and pushes the docker image
	docker push $(IMAGE)

_push_multiarch_docker: ## experimental, builds and publishes docker images for i386, amd64 and arm64. prepare buildx environment using `docker buildx create --use`
	docker run --privileged --rm tonistiigi/binfmt --install all
	#toolchain for armv7 is borked ATM, sorry, no RPI builds yet.
	#docker buildx build --platform linux/arm64,linux/amd64,linux/arm/v7 -t registry.k8s.betalabs.rocks/ws-2-mqtt-multiarch . --push
	docker buildx build --platform linux/arm64,linux/amd64 -t $(IMAGE) . --push

help: ## print help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
