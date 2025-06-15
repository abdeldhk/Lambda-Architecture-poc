🏗️ Architecture Overview
This project implements a complete Lambda Architecture for real-time and batch processing of flight event data, using a mix of Rust, Python, Java, and Dockerized open-source tools.

📐 Architecture Layers
1. Ingestion Layer
Language: Rust

Component: Kafka Producer

Function: Generates meaningful flight events and publishes them to Kafka topic flight-events.

2. Batch Layer
Language: Python (PySpark)

Component: batch-layer/etl-job.py

Function: Reads raw data from local storage or S3 and performs heavy batch aggregations (e.g., full route counts).

Optional Output: Push to Hive metastore when running on Databricks.

3. Speed Layer
Real-time processing using 3 parallel engines:

Engine	Language	Path	Output Topic
Spark Streaming	PySpark	speed-layer/spark/stream_job.py	flight-route-counts
Kafka Streams	Java	speed-layer/kafka-streams/	flight-route-counts
Apache Flink	Java	speed-layer/flink/	flight-route-counts

Each reads from flight-events, aggregates in-stream, and publishes live metrics.

4. Serving Layer
Language: Rust

Component: REST API built with Actix

Endpoints:

GET /health

GET /route-counts?origin=XXX&destination=YYY

Backends: Placeholder for Cassandra or Elasticsearch queries

📦 Technologies Used
Layer	Tech Stack
Ingestion	Rust, Kafka
Batch	PySpark, Hive (optional)
Speed	Kafka Streams (Java), Flink (Java), Spark Streaming (PySpark)
Serving	Rust (Actix-web)
Storage	MinIO (S3 compatible), Cassandra, Elasticsearch
Orchestration	Docker, Docker Compose
Monitoring	(Pluggable: Kafka UI, Kibana)

📁 Key Files
docker/docker-compose.yml: Brings up all infra

docker/kafka-connect/flight-file-sink.json: Sink Kafka topic to file

config/topics.env: Reusable topic names

data/flights/*.json: Raw flight event files

ingestion/kafka-producer/: Rust Kafka producer

serving-layer/rest-api/: Rust API server

