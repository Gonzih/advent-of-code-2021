TASKS = day1 day2

.PHONY: all $(TASKS)
$(TASKS):
	cd $@ && python3 main.py
all: $(TASKS)
