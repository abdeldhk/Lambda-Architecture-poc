package com.lambda;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.kafka.common.serialization.Serdes;
import org.apache.kafka.streams.*;
import org.apache.kafka.streams.kstream.*;

import java.util.Properties;

public class FlightStreamApp {

    public static void main(String[] args) {
        // Kafka Streams config
        Properties props = new Properties();
        props.put(StreamsConfig.APPLICATION_ID_CONFIG, "flight-stream-app");
        props.put(StreamsConfig.BOOTSTRAP_SERVERS_CONFIG, "localhost:9092");
        props.put(StreamsConfig.DEFAULT_KEY_SERDE_CLASS_CONFIG, Serdes.String().getClass());
        props.put(StreamsConfig.DEFAULT_VALUE_SERDE_CLASS_CONFIG, Serdes.String().getClass());

        StreamsBuilder builder = new StreamsBuilder();
        ObjectMapper mapper = new ObjectMapper();

        KStream<String, String> stream = builder.stream("flight-events");

        stream
            .mapValues(value -> {
                try {
                    JsonNode json = mapper.readTree(value);
                    String origin = json.get("origin").asText();
                    String destination = json.get("destination").asText();
                    return origin + "-" + destination;
                } catch (Exception e) {
                    return "invalid";
                }
            })
            .filter((key, route) -> !route.equals("invalid"))
            .groupBy((key, route) -> route, Grouped.with(Serdes.String(), Serdes.String()))
            .count(Materialized.as("route-counts"))
            .toStream()
            .foreach((route, count) -> System.out.println("Route: " + route + " | Flights: " + count));

        KafkaStreams streams = new KafkaStreams(builder.build(), props);
        streams.start();

        // Graceful shutdown
        Runtime.getRuntime().addShutdownHook(new Thread(streams::close));
    }
}
