#!/bin/bash

set -e

echo "Registering Kafka Connect File Sink..."
curl -X POST http://localhost:8083/connectors \
  -H "Content-Type: application/json" \
  -d @kafka-connect/flight-file-sink.json
