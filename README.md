# 🚀 rust-api-gateway

A high-performance, lightweight API Gateway built with Rust and Actix-web. Designed for microservices architecture, this gateway handles request forwarding, routing, and can be extended with authentication, rate limiting, caching, and observability.

---

## ✨ Features

- 🔁 **Request forwarding** to multiple microservices
- 🧠 **Path-based routing**: `/api/users/...` → `http://localhost:3001/...`
- 🛡 **CORS enabled** (for frontend integrations)
- 📊 **Health check** endpoint at `/health`
- ⚙️ Easily extensible (JWT, rate limiting, caching, etc.)

---

## 🧪 Example Routing

| API Gateway URL                           | Routed to Target Service              |
| ----------------------------------------- | ------------------------------------- |
| `http://localhost:8080/api/users/me`      | `http://localhost:3001/users/me`      |
| `http://localhost:8080/api/products/list` | `http://localhost:3002/products/list` |
| `http://localhost:8080/api/orders/123`    | `http://localhost:3003/orders/123`    |

---

## 🛠️ Getting Started

### 1. Clone the Repo

```bash
git clone https://github.com/your-username/rust-api-gateway.git
cd rust-api-gateway
```

### 2. Install Rust (if needed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3. Build and Run

```bash
cargo run
```

### 📄 API Endpoints

- ✅ Health Check
  GET /hc

### 🚧 To Do / Extend

Configurable service map via config.toml or .env

JWT authentication middleware

Rate limiting via Redis or memory

Circuit breaker and retry logic

Tracing and metrics (Prometheus / OpenTelemetry)
