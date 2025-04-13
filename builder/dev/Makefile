# Default compose file
COMPOSE_FILE=docker-compose.yml

.PHONY: up down restart

# Make sure entrypoint scripts are executable and start dev stack
up:
	docker compose -f $(COMPOSE_FILE) up --build

# Stop dev stack
down:
	docker compose -f $(COMPOSE_FILE) down

# Restart dev stack
restart: down up
