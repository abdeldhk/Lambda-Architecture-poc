#!/bin/bash

KAFKA_BROKER=localhost:9092

TOPICS=(
  "flight-events"
  "flight-route-counts"
  "connect-configs"
  "connect-offsets"
  "connect-status"
)

for topic in "${TOPICS[@]}"; do
  echo "Creating topic: $topic"
  docker exec kafka kafka-topics.sh \
    --create \
    --if-not-exists \
    --bootstrap-server $KAFKA_BROKER \
    --replication-factor 1 \
    --partitions 1 \
    --topic $topic

done