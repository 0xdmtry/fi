COMPOSE_FILES = \
	-f dc.authorizer.fe.yml \
	-f dc.authorizer.yml \
	-f dc.emailer.yml \
	-f dc.observability.yml \
	-f dc.solana.aggregator.yml \
	-f dc.solana.transactor.yml \
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


# Individual stack commands
up-auth-fe:
	docker compose -f dc.authorizer.fe.yml up --build

up-auth:
	docker compose -f dc.authorizer.yml up --build

up-emailer:
	docker compose -f dc.emailer.yml up --build

up-ob:
	docker compose -f dc.observability.yml up --build

up-sol-agg:
	docker compose -f dc.solana.aggregator.yml up --build

up-sol-tx:
	docker compose -f dc.solana.transactor.yml up --build

up-sol-wal:
	docker compose -f dc.solana.walletor.yml up --build