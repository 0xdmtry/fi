COMPOSE_FILES = \
	-f dc.authorizer.fe.yml \
	-f dc.authorizer.yml \
	-f dc.emailer.yml \
	-f dc.observability.yml \
	-f dc.solana.aggregator.yml \
	-f dc.solana.walletor.yml

.PHONY: up down restart test

up:
	docker compose $(COMPOSE_FILES) up --build

down:
	docker compose $(COMPOSE_FILES) down

restart: down up

test:
	cargo test --manifest-path ../../app/authorizer/Cargo.toml
	cargo test --manifest-path ../../app/emailer/Cargo.toml
	cargo test --manifest-path ../../dapp/solana/walletor/Cargo.toml
