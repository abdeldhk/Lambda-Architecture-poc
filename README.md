# ✈️ Lambda Architecture PoC — Flight Event Analytics

Welcome to the official Proof of Concept (PoC) for a full-stack **Lambda Architecture** project built by [@abdeldhk](https://github.com/abdeldhk). This system processes flight event data in **real-time and batch**, leveraging a modern big data stack, containerized via Docker.

---

## 🧱 Architecture Overview

This PoC implements the classical Lambda Architecture model:

- **Ingestion Layer** → Rust-based Kafka producer simulating flight events
- **Batch Layer** → PySpark job processes historical data into analytics-ready formats
- **Speed Layer** → Real-time aggregation with Kafka Streams, Flink, and Spark Streaming
- **Serving Layer** → Rust REST API delivers fast access to metrics

---

## ⚙️ Stack Components

| Layer            | Technology                                                  |
|------------------|-------------------------------------------------------------|
| Ingestion        | Rust + Apache Kafka                                         |
| Batch Layer      | PySpark + Hive-compatible S3 (MinIO)                        |
| Speed Layer      | Kafka Streams (Java), Apache Flink (Java), Spark Streaming (PySpark) |
| Serving Layer    | Rust (Actix Web)                                            |
| Storage          | Cassandra, MinIO, Elasticsearch                             |
| Containerization | Docker + Docker Compose                                     |

---

## 🚀 Getting Started

### 🐳 1. Start the Infrastructure
```bash
cd docker
docker compose up -d
```

### 🪄 2. Create Kafka Topics
```bash
docker exec kafka bash /kafka/create-topics.sh
```

### 🔌 3. (Optional) Register Kafka File Sink Connector
```bash
./docker/scripts/register-kafka-sink.sh
```

### ✈️ 4. Generate Flight Event Samples
```bash
cargo run --bin generate_flights
```

### 📤 5. Start the Kafka Producer
```bash
cargo run --bin kafka-producer
```

### 📦 6. Run the Batch ETL Job (PySpark)
```bash
spark-submit batch-layer/etl-job.py
```

---

## 🌐 REST API (Serving Layer)

Start the Rust-based API server:
```bash
cd serving-layer/rest-api
cargo run
```

### Example Endpoints:
- `GET /health` → Health check
- `GET /route-counts?origin=DXB&destination=ZRH` → Returns flight count for route

---

## 📁 Directory Overview

```
Lambda-Architecture-poc/
├── config/                # Kafka and topic configs
├── data/                  # Generated flight events
├── docker/                # Docker setup (Kafka, Spark, etc.)
├── ingestion/             # Rust-based Kafka producer
├── batch-layer/           # PySpark ETL job
├── speed-layer/           # Spark, Flink, and Kafka Streams jobs
├── serving-layer/         # Rust Actix REST API
├── architecture.md        # System design documentation
└── README.md              # This file
```

---

## 🛠️ Useful UIs

| Tool        | URL                     |
|-------------|--------------------------|
| Spark UI    | http://localhost:8080    |
| Kafka UI    | http://localhost:8082 *(if enabled)* |
| Kibana      | http://localhost:5601 *(optional)* |

---

## 📌 Roadmap

- [ ] Query Cassandra or Elasticsearch from REST API
- [ ] Add windowed aggregations (e.g. per 5 minutes)
- [ ] Add Prometheus + Grafana
- [ ] Package each job as an independent service

---

## 🧠 License

MIT © [Abdelhamid Dahak](https://github.com/abdeldhk)