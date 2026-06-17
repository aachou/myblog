+++
title = "Kubernetes Package Management with Helm"
date = "2023-05-18"
tags = ["kubernetes", "helm", "devops"]
excerpt = "Helm is the package manager for Kubernetes. Learn how to create charts, manage releases, template resources, and build reusable infrastructure components."
+++

Helm simplifies deploying and managing applications on Kubernetes. Instead of writing dozens of YAML files by hand, you define a reusable chart that packages all your resources together with powerful templating.

## Chart Structure

A Helm chart has a well-defined directory layout:

```
mychart/
├── Chart.yaml          # Metadata
├── values.yaml         # Default configuration values
├── charts/             # Sub-chart dependencies
└── templates/          # Kubernetes resource templates
    ├── _helpers.tpl    # Named template definitions
    ├── deployment.yaml
    ├── service.yaml
    ├── ingress.yaml
    └── hpa.yaml
```

## Chart.yaml

Every chart needs a `Chart.yaml` file:

```yaml

apiVersion: v2
name: myapp
description: A production-ready web application
type: application
version: 1.0.0
appVersion: "1.16.0"
dependencies:
    - name: redis
        version: "17.x"
        repository: "https://charts.bitnami.com/bitnami"
```

## Templating with Go Templates

Helm uses Go templates to generate Kubernetes manifests dynamically. Access values from `values.yaml`:


```yaml
# values.yaml
replicaCount: 3
image:
  repository: nginx
  tag: stable
service:
  port: 80
  type: ClusterIP
```

```yaml
# templates/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
    name: {{ include "mychart.fullname" . }}
spec:
    replicas: {{ .Values.replicaCount }}
    selector:
        matchLabels:
            app: {{ include "mychart.name" . }}
    template:
        metadata:
            labels:
                app: {{ include "mychart.name" . }}
        spec:
            containers:
                - name: {{ .Chart.Name }}
                    image: "{{ .Values.image.repository }}:{{ .Values.image.tag }}"
                    ports:
                        - containerPort: {{ .Values.service.port }}

```
| Template Function | Purpose |
|-------------------|---------|
| `.Values.*` | Access user-supplied values |
| `.Chart.*` | Access chart metadata |
| `.Release.*` | Access release information |
| `include` | Call a named template |
| `default` | Provide fallback values |

## Built-in Objects

Helm provides several built-in objects for templates:

- `{{ .Release.Name }}` — Name of the release
- `{{ .Release.Namespace }}` — Target namespace
- `{{ .Chart.Name }}` — Chart name from Chart.yaml
- `{{ .Files.Get "config.json" }}` — Access chart files

## Managing Releases

```bash
# Install a chart
helm install myapp ./mychart --values production.yaml

# Upgrade a release
helm upgrade myapp ./mychart --values production.yaml

# Rollback if something goes wrong
helm rollback myapp 1

# List releases
helm list -n production
```

## CI/CD Integration

Helm works seamlessly in pipelines:

```yaml
# GitHub Actions step
- name: Deploy with Helm
    run: |
        helm upgrade --install myapp ./mychart \
            --namespace production \
            --values values/production.yaml \
            --set image.tag=${{ github.sha }}

```
Helm's three-way strategic merge patching ensures upgrades are idempotent and rollbacks are reliable. It is the standard way to package and deploy Kubernetes applications across most organizations.
