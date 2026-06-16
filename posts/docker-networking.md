+++
title = "Docker Networking"
date = "2022-03-10"
tags = ["docker", "networking", "devops"]
excerpt = "Understanding Docker networking modes: bridge, host, overlay, and macvlan. Learn how containers communicate with each other and the outside world."
+++

Docker networking is one of the most important concepts to master when running containerised applications. Without proper networking, containers are isolated islands that cannot communicate.

## Network Drivers

Docker ships with several built-in network drivers:

| Driver | Use Case | Scope |
|--------|----------|-------|
| bridge | Single-host container communication | Local |
| host | No network isolation, use host stack | Local |
| overlay | Multi-host container communication | Swarm |
| macvlan | Assign MAC addresses to containers | Local |
| none | Disable networking | Local |

## Bridge Networks

By default, Docker creates a bridge network called `bridge`. Containers attached to it can communicate using IP addresses. For service discovery, you should create a user-defined bridge, which provides DNS resolution.

```bash
docker network create my-network
docker run -d --name web --network my-network nginx
docker run -d --name db --network my-network postgres
# web can now resolve hostname "db"
```

## Exposing Ports

Mapping container ports to host ports is done with the `-p` flag:

```bash
docker run -d -p 8080:80 nginx
```

This binds host port 8080 to container port 80. Traffic hitting `http://localhost:8080` reaches the nginx container.

## Overlay Networks

Overlay networks enable communication between containers on different Docker hosts. They are essential for Docker Swarm and multi-host deployments.

```yaml
version: "3.8"
services:
  app:
    image: myapp
    networks:
      - overlay-net

networks:
  overlay-net:
    driver: overlay
    attachable: true
```

## Network Security

- Place only containers that need to communicate on the same network
- Use internal networks for backend services
- Never expose a database port directly to the internet

```bash
# Create an internal network with no external access
docker network create --internal backend-net
```

## Debugging

Use `docker network inspect` and `docker container exec` for troubleshooting:

```bash
docker network inspect my-network
docker container exec web ping db
```

Understanding Docker networking is critical for building secure, scalable microservice architectures.
