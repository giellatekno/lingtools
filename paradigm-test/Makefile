PORT ?= 8080
URL   = http://localhost:$(PORT)/paradigm-test/paradigm-test.html

.PHONY: run

run:
	@echo "Open: $(URL)"
	python3 -m http.server $(PORT) --directory ..
