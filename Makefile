DAYS = $(shell seq 1 1 9)
TASKS = $(patsubst %,day%,$(DAYS))

.PHONY: all $(TASKS)
$(TASKS):
	cd $@ && mypy main.py && python3.10 main.py
all: $(TASKS)

deps:
	sudo add-apt-repository -y ppa:deadsnakes/ppa
	sudo apt install -y python3.10 python3-pip
	sudo pip3 install mypy
