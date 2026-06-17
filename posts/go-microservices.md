+++
title = "Go Microservices"
date = "2022-11-05"
tags = ["go", "microservices", "golang"]
excerpt = "Building microservices with Go 鈥?HTTP servers, middleware patterns, gRPC communication, and production deployment strategies."
+++

Go is a popular language for building microservices because of its simple concurrency model, fast compilation, and small binary footprint. This post covers patterns for building production-ready services.

## HTTP Server

A basic HTTP server in Go takes only a few lines:

```go
package main

import (
    "encoding/json"
    "log"
    "net/http"
)

type HealthResponse struct {
    Status string `json:"status"`
}

func healthHandler(w http.ResponseWriter, r *http.Request) {
    w.Header().Set("Content-Type", "application/json")
    json.NewEncoder(w).Encode(HealthResponse{Status: "ok"})
}

func main() {
    http.HandleFunc("/health", healthHandler)
    log.Fatal(http.ListenAndServe(":8080", nil))
}
```

## Middleware Pattern

Go's `http.Handler` interface makes middleware composition clean:

```go
func loggingMiddleware(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        start := time.Now()
        next.ServeHTTP(w, r)
        log.Printf("%s %s %s", r.Method, r.URL.Path, time.Since(start))
    })
}

func authMiddleware(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        token := r.Header.Get("Authorization")
        if !isValidToken(token) {
            http.Error(w, "Unauthorized", http.StatusUnauthorized)
            return
        }
        next.ServeHTTP(w, r)
    })
}

mux := http.NewServeMux()
mux.Handle("/api/", authMiddleware(loggingMiddleware(apiHandler)))
```

## gRPC Communication

For inter-service communication, gRPC is more efficient than REST:

```protobuf
syntax = "proto3";

service UserService {
    rpc GetUser (GetUserRequest) returns (User);
}

message GetUserRequest {
    string user_id = 1;
}

message User {
    string id = 1;
    string name = 2;
    string email = 3;
}

```
```go
func (s *server) GetUser(ctx context.Context, req *pb.GetUserRequest) (*pb.User, error) {
    user, err := s.db.FindUser(req.UserId)
    if err != nil {
        return nil, status.Error(codes.NotFound, "user not found")
    }
    return &pb.User{Id: user.ID, Name: user.Name, Email: user.Email}, nil
}
```

## Service Discovery

| Method | Tool | Pros |
|--------|------|------|
| DNS | CoreDNS | Simple, standard |
| Service mesh | Istio | Rich features |
| Key-value store | Consul | Health checks |

## Configuration

Use environment variables or YAML files:

```go
type Config struct {
    Port     string `envconfig:"PORT" default:"8080"`
    Database string `envconfig:"DATABASE_URL" required:"true"`
}

var cfg Config
err := envconfig.Process("", &cfg)
```

## Graceful Shutdown

```go
ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
defer stop()

srv := &http.Server{Addr: ":8080"}
go srv.ListenAndServe()

<-ctx.Done()
shutdownCtx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
defer cancel()
srv.Shutdown(shutdownCtx)
```

Go's standard library and thriving ecosystem make it an excellent choice for microservice architectures.
