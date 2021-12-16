PY_DAYS = $(shell seq 1 1 10)
RS_DAYS = $(shell seq 11 1 15)
GO_DAYS = $(shell seq 16 1 16)
PY_TASKS = $(patsubst %,day%,$(PY_DAYS))
RS_TASKS = $(patsubst %,day%,$(RS_DAYS))
GO_TASKS = $(patsubst %,day%,$(GO_DAYS))

.PHONY: all $(PY_TASKS) $(RS_TASKS) $(GO_TASKS)
$(PY_TASKS):
	cd $@ && mypy main.py && python3.10 main.py
$(RS_TASKS):
	cd $@ && cargo test && cargo build --release && ./target/release/aoc
$(GO_TASKS):
	cd $@ && go test && go build -o main . && ./main
all: $(PY_TASKS) $(RS_TASKS) $(GO_TASKS)

GO_VERSION = 1.17.5
deps:
	sudo add-apt-repository -y ppa:deadsnakes/ppa
	sudo apt install -y python3.10 python3-pip
	sudo pip3 install mypy
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/installer.sh
	sh /tmp/installer.sh -y
	rustup default nightly
	cd /tmp && wget https://golang.org/dl/go$(GO_VERSION).linux-amd64.tar.gz
	sudo tar -C /usr/local -xzf /tmp/go$(GO_VERSION).linux-amd64.tar.gz
	sudo ln -sf /usr/local/go/bin/go /usr/bin/go
	go version
