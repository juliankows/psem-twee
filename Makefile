

backend: frontend
	cargo b --release
.PHONY: backend

frontend:
	$(MAKE) -C web build
.PHONY: frontend

.DEFAULT_GOAL = backend
