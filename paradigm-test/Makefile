PORT         ?= 8080
LOCAL_API    = http://localhost:3000
EXTERNAL_API = https://gtweb.uit.no/lingtools-api

URL_LOCAL    = http://localhost:$(PORT)/paradigm-test/paradigm-test.html?api=$(LOCAL_API)
URL_EXTERNAL = http://localhost:$(PORT)/paradigm-test/paradigm-test.html?api=$(EXTERNAL_API)

.PHONY: run-local run

run-local:
	@echo "Open: $(URL_LOCAL)"
	@xdg-open "$(URL_LOCAL)" 2>/dev/null &
	python3 -m http.server $(PORT) --directory ..

run:
	@echo "Open: $(URL_EXTERNAL)"
	@xdg-open "$(URL_EXTERNAL)" 2>/dev/null &
	python3 -m http.server $(PORT) --directory ..
