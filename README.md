# 🔐 Auth System — Microservice-Based Architecture

This project is a modular, testable authentication and passcode delivery system built with Rust. It includes multiple microservices, each focused on a single responsibility, and is orchestrated with Docker Compose.

---

## 🧱 Architecture Overview

- **Authorizer**  
  Handles user join/signup requests and passcode management.

- **Emailer**  
  Responsible for delivering emails via pluggable providers (MailHog, Mailgun, etc.).

- **PostgreSQL**  
  Each microservice has its own dedicated DB instance.

- **MailHog**  
  Development email testing interface (SMTP + Web UI).

---

## 🧪 Environments

| Environment | Stack                               | Compose File                    |
|-------------|--------------------------------------|----------------------------------|
| Dev         | Live reload, local volume mounts     | `docker-compose.yml`            |
| Test        | Isolated containers + test DB        | `builder/test/docker-compose.yml` |

---

## 🦀 Tech Stack

- **Language**: Rust 1.85+
- **Framework**: Axum
- **ORM**: SeaORM
- **Validation**: `validator`
- **Database**: PostgreSQL 17.4
- **Mail Dev**: MailHog
- **Live Reload**: `cargo-watch`
- **Dev/Test Isolation**: Docker Compose

---

## 🚀 Getting Started

### Dev

```bash
make up        # Builds and runs dev environment
make down      # Stops dev environment
```

### Test

```bash
make up    # Builds and runs isolated test stack
make down  # Stops test containers
```

---

## 📁 Project Layout

```
/
├── app/
│   ├── authorizer/
│   └── emailer/
├── builder/
│   └── dev/
│       ├── docker-compose.yml
│       └── Makefile
│   └── test/
│       ├── docker-compose.yml
│       └── Makefile
└── README.md
```

---
