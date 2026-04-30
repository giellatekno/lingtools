PORT ?= 8080
URL   = http://localhost:$(PORT)/paradigm-test/paradigm-test.html

.PHONY: run

run:
	@echo "Open page at:"
	@echo "$(URL)"
	@echo ""
	python3 -m http.server $(PORT) --directory ..
