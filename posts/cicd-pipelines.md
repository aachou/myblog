+++
title = "CI/CD Pipelines"
date = "2022-06-12"
tags = ["devops", "ci-cd", "automation"]
excerpt = "A practical guide to building continuous integration and deployment pipelines. Covers GitHub Actions, testing strategies, and deployment patterns."
+++

Continuous Integration and Continuous Deployment (CI/CD) automates the software delivery process. Every commit triggers an automated pipeline that builds, tests, and deploys the application.

## Key Concepts

- **Continuous Integration** 鈥?merge code changes frequently; each merge triggers automated builds and tests
- **Continuous Delivery** 鈥?every passing build is deployable to production
- **Continuous Deployment** 鈥?every passing build is automatically deployed to production

## GitHub Actions Example

```yaml
name: CI/CD Pipeline

on:
  push:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: 18
      - run: npm ci
      - run: npm test

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: npm ci
      - run: npm run build
      - uses: actions/upload-artifact@v3
        with:
          name: build
          path: dist/

  deploy:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v3
        with:
          name: build
      - run: ./deploy.sh
```

## Testing Stages

A robust pipeline includes multiple testing layers:

1. **Lint** 鈥?static analysis and formatting
2. **Unit tests** 鈥?test individual functions
3. **Integration tests** 鈥?test components together
4. **E2E tests** 鈥?test the full system

```bash
# Running the pipeline stages locally
npm run lint
npm run test:unit
npm run test:integration
npm run test:e2e
```

## Deployment Patterns

| Pattern | Description | Risk |
|---------|-------------|------|
| Rolling | Replace instances gradually | Low |
| Blue/Green | Run two identical environments | Medium |
| Canary | Roll out to a subset of users | Low |

## Environment Variables and Secrets

Never hardcode secrets in pipeline files. Use repository secrets:

```yaml
- run: echo "${{ secrets.DEPLOY_KEY }}" | base64 --decode > key.pem
```

## Pipeline Metrics

Track these metrics to measure pipeline health:

- **Build time** 鈥?how long the pipeline takes
- **Failure rate** 鈥?percentage of failed builds
- **Deployment frequency** 鈥?how often you ship

A well-tuned CI/CD pipeline reduces manual error, catches bugs early, and accelerates delivery cycles.
