from pyspark.sql import SparkSession
from pyspark.sql.functions import col, count, to_json, struct, from_json
from pyspark.sql.types import StructType, StringType


# Note: This script assumes Kafka is running locally on port 9092 and the topic 'flight-events' exists.
# It also assumes the Kafka topic 'flight-route-counts' is set up to receive the output. 


# Create Spark session
spark = SparkSession.builder \
    .appName("Flight Route Stream Processor") \
    .getOrCreate()

# Define schema of flight event JSON
schema = StructType() \
    .add("flight_id", StringType()) \
    .add("airline", StringType()) \
    .add("origin", StringType()) \
    .add("destination", StringType()) \
    .add("scheduled_departure", StringType()) \
    .add("actual_departure", StringType()) \
    .add("status", StringType()) \
    .add("aircraft", StructType()
         .add("model", StringType())
         .add("registration", StringType()))

# Read from Kafka topic 'flight-events'
df_raw = spark.readStream \
    .format("kafka") \
    .option("kafka.bootstrap.servers", "localhost:9092") \
    .option("subscribe", "flight-events") \
    .option("startingOffsets", "latest") \
    .load()

# Extract JSON value and parse it
df = df_raw.selectExpr("CAST(value AS STRING) as json_str") \
    .select(from_json(col("json_str"), schema).alias("data")) \
    .select("data.*")

# Compute count per route
route_counts = df.groupBy("origin", "destination").agg(count("*").alias("flight_count"))

# Convert to JSON for Kafka output
route_counts_json = route_counts.select(
    to_json(struct(col("origin"), col("destination"), col("flight_count"))).alias("value")
)

# Write to Kafka topic 'flight-route-counts'
query_kafka = route_counts_json.writeStream \
    .format("kafka") \
    .outputMode("update") \
    .option("kafka.bootstrap.servers", "localhost:9092") \
    .option("topic", "flight-route-counts") \
    .option("checkpointLocation", "checkpoints/spark-route-counts") \
    .start()

query_kafka.awaitTermination()
# Stop Spark session
spark.stop()
# End of stream_job.py
   