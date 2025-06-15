from pyspark.sql import SparkSession
from pyspark.sql.functions import col, count, avg, to_timestamp
import os


# This script processes flight data in batch mode, calculating average delays, flight counts per route, and cancelled flights.
# It supports both Databricks and local Spark environments, saving results to Delta tables or JSON files accordingly.
# This script processes flight data in batch mode, calculating average delays, flight counts per route, and cancelled flights.      


# Detect if we're running inside Databricks
IS_DATABRICKS = os.environ.get("DATABRICKS_RUNTIME_VERSION") is not None

# Init Spark
spark = SparkSession.builder \
    .appName("Flight Batch Processing") \
    .enableHiveSupport() if IS_DATABRICKS else SparkSession.builder.appName("Flight Batch Processing") \
    .getOrCreate()

# Load data from file system (could be DBFS, S3, or local depending on env)
input_path = "dbfs:/mnt/flight_data/" if IS_DATABRICKS else "data/flights/*.json"
df = spark.read.json(input_path)

# Convert timestamps
df = df.withColumn("scheduled_departure", to_timestamp("scheduled_departure")) \
       .withColumn("actual_departure", to_timestamp("actual_departure"))

# Calculate delay
df = df.withColumn("delay_min", 
                   (col("actual_departure").cast("long") - col("scheduled_departure").cast("long")) / 60)

# Aggregation 1: Average delay per airline
avg_delay = df.groupBy("airline").agg(avg("delay_min").alias("avg_delay_min"))

# Aggregation 2: Flights per route
route_counts = df.groupBy("origin", "destination").agg(count("*").alias("flight_count"))

# Aggregation 3: Cancelled flights
cancelled = df.filter(col("status") == "cancelled") \
              .groupBy("airline").agg(count("*").alias("cancelled_flights"))

# Output
if IS_DATABRICKS:
    # Save as managed Delta tables and register in Hive Metastore
    avg_delay.write.mode("overwrite").format("delta").saveAsTable("flight_analytics.avg_delay")
    route_counts.write.mode("overwrite").format("delta").saveAsTable("flight_analytics.route_counts")
    cancelled.write.mode("overwrite").format("delta").saveAsTable("flight_analytics.cancelled_flights")
else:
    # Save locally as JSON for testing/dev
    avg_delay.write.mode("overwrite").json("data/output/avg_delay")
    route_counts.write.mode("overwrite").json("data/output/routes")
    cancelled.write.mode("overwrite").json("data/output/cancelled")
# Stop Spark session
spark.stop()
# End of batch_job.py
