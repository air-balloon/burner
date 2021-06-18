.PHONY: build docker-build run test format migrate-status

IMAGE_TAG ?= air-balloon-burner

# All: linux/amd64,linux/arm64,linux/riscv64,linux/ppc64le,linux/s390x,linux/386,linux/mips64le,linux/mips64,linux/arm/v7,linux/arm/v6
PLATFORM ?= linux/amd64
ACTION ?= load
PROGRESS_MODE ?= plain
BUILDX_CLI_ARGS ?=

CONTAINER_NAME ?= air-balloon-burner
CONTAINER_NETWORK ?= local_williamdeslocal
CONTAINER_BIND_PORT ?= 8081
# Internal port, keep it the same as CONTAINER_BIND_PORT or you will get confused
ROCKET_PORT ?= 8081
DOCKER_SOCKET ?= /var/run/docker.sock

migrate:
	@diesel migration run

migrate-status:
	@diesel migration list

build:
	@cargo build

test:
	@cargo test

format:
	@cargo fmt -- --emit files

docker-build:
	# https://github.com/docker/buildx#building
	docker buildx build \
		--file ./docker/Dockerfile \
		--tag $(IMAGE_TAG) \
		--progress $(PROGRESS_MODE) \
		--platform $(PLATFORM) \
		--build-arg VCS_REF=`git rev-parse HEAD` \
		--build-arg BUILD_DATE=`date -u +"%Y-%m-%dT%H:%M:%SZ"` \
		--$(ACTION) \
		$(BUILDX_CLI_ARGS) \
		./

run:
	@docker kill $(CONTAINER_NAME) || echo 'skip kill'
	@docker run --name $(CONTAINER_NAME) --network $(CONTAINER_NETWORK) -t --rm --env-file ./.env -p $(CONTAINER_BIND_PORT):$(ROCKET_PORT) -e ROCKET_PORT=$(ROCKET_PORT) -e ROCKET_ADDRESS="0.0.0.0" -v $(DOCKER_SOCKET):/var/run/docker.sock $(IMAGE_TAG)
