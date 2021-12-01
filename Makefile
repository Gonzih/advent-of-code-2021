TASKS = day1 day2

.PHONY: all $(TASKS)
$(TASKS):
	cd $@ && python3.10 main.py
all: $(TASKS)

deps:
	sudo add-apt-repository -y ppa:deadsnakes/ppa
	sudo apt install -y python3.10
