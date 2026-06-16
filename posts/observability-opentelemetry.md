+++
title = "Observability with OpenTelemetry: Traces, Metrics, and Logs"
date = "2023-11-28"
tags = ["opentelemetry", "observability", "monitoring"]
excerpt = "OpenTelemetry provides a unified standard for collecting telemetry data. Learn how to instrument your applications with distributed tracing, custom metrics, and structured logging."
+++

OpenTelemetry (OTel) has become the industry standard for observability. It provides a single set of APIs, SDKs, and tools for generating, collecting, and exporting telemetry data from your applications.

## The Three Pillars

| Pillar | What it tells you | Example |
|--------|-------------------|---------|
| Traces | How requests flow through services | Request path, latency per span |
| Metrics | Aggregate measurements | Request rate, error rate, CPU usage |
| Logs | Discrete events with context | Error messages, audit events |

## Setting Up the SDK

```javascript
const { NodeSDK } = require("@opentelemetry/sdk-node");
const { ConsoleSpanExporter } = require("@opentelemetry/sdk-trace-node");
const { getNodeAutoInstrumentations } = require("@opentelemetry/auto-instrumentations-node");
const { OTLPTraceExporter } = require("@opentelemetry/exporter-trace-otlp-http");

const sdk = new NodeSDK({
    traceExporter: new OTLPTraceExporter({
        url: "http://localhost:4318/v1/traces"
    }),
    instrumentations: [getNodeAutoInstrumentations()]
});

sdk.start();
```

## Creating Custom Spans

Add instrumentation to critical code paths:

```javascript
const { trace } = require("@opentelemetry/api");
const tracer = trace.getTracer("payment-service");

async function processPayment(orderId, amount) {
    const span = tracer.startSpan("process-payment", {
        attributes: {
            "order.id": orderId,
            "payment.amount": amount,
            "payment.currency": "USD"
        }
    });

    try {
        const result = await paymentGateway.charge(amount);
        span.setStatus({ code: SpanStatusCode.OK });
        span.setAttribute("payment.status", result.status);
        return result;
    } catch (error) {
        span.recordException(error);
        span.setStatus({
            code: SpanStatusCode.ERROR,
            message: error.message
        });
        throw error;
    } finally {
        span.end();
    }
}
```

## Propagating Context

Context propagation ensures traces span service boundaries:

```javascript
// Service A — outbound HTTP
const { context, propagation } = require("@opentelemetry/api");

async function callServiceB(payload) {
    const headers = {};
    propagation.inject(context.active(), headers);
    await axios.post("http://service-b/api", payload, { headers });
}

// Service B — inbound HTTP (auto-instrumentation handles extraction)
app.post("/api", (req, res) => {
    const span = tracer.startSpan("handle-request");
    // Headers already extracted via auto-instrumentation
    processRequest(req).finally(() => span.end());
});
```

## Custom Metrics

Measure application-specific metrics:

```javascript
const { metrics } = require("@opentelemetry/api");
const meter = metrics.getMeter("order-service");

const orderCounter = meter.createCounter("orders.created", {
    description: "Total number of orders created"
});

const activeOrders = meter.createUpDownCounter("orders.active", {
    description: "Currently active orders"
});

const orderDuration = meter.createHistogram("order.processing.duration", {
    description: "Time to process an order",
    unit: "ms"
});

// Record metrics
orderCounter.add(1, { "order.type": type });
activeOrders.add(1);
const endTimer = orderDuration.startTimer();
// ... process ...
endTimer({ "order.type": type, "status": "success" });
```

## Structured Logging with Trace Context

Correlate logs with traces by including trace IDs:

```javascript
const { trace } = require("@opentelemetry/api");
const winston = require("winston");

const logger = winston.createLogger({
    format: winston.format.combine(
        winston.format.timestamp(),
        winston.format.printf(({ timestamp, level, message, ...meta }) => {
            const spanContext = trace.getSpanContext(trace.getActiveSpan());
            const traceId = spanContext?.traceId;
            const spanId = spanContext?.spanId;

            return JSON.stringify({
                timestamp,
                level,
                message,
                trace_id: traceId,
                span_id: spanId,
                ...meta
            });
        })
    ),
    transports: [new winston.transports.Console()]
});
```

## Sampling Strategies

Sampling controls costs by exporting only a fraction of traces:

- **Head-based**: Sample decision at the root span (e.g., 10% of all requests)
- **Tail-based**: Sample based on final result (e.g., all failed requests)
- **Probability sampling**: Consistent random sampling

```javascript
const { ParentBasedSampler, TraceIdRatioBasedSampler } = require("@opentelemetry/sdk-trace-node");

const sampler = new ParentBasedSampler({
    root: new TraceIdRatioBasedSampler(0.1) // 10% sampling
});
```

## Exporting to Backends

OTel supports multiple backends through exporters:

- **OTLP** (native OTel protocol) to collectors like OpenTelemetry Collector
- **Jaeger** for distributed tracing
- **Prometheus** for metrics
- **Elasticsearch / Loki** for logs
- **Datadog, New Relic, Honeycomb** via exporters or collectors

OpenTelemetry is vendor-neutral by design. You can start with the open-source Collector and switch backends later without changing instrumentation. Adopt OTel early to avoid vendor lock-in and build a robust observability culture.
