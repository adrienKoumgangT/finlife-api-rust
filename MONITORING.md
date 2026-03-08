# Local Kubernetes Monitoring Stack

This document outlines how to deploy a complete, "Infrastructure as Code" monitoring stack for our local Kubernetes environment (Docker Desktop).
The stack uses **Prometheus** for metric scraping/storage and **Grafana** for visualization, specifically targeting our local databases: **MinIO, MySQL, and Redis**.

## Architecture Overview

Prometheus is a pull-based monitoring system. It reaches out to services to scrape metrics.

- **Native Scraping:** Some services (like MinIO) expose Prometheus metrics natively.
Prometheus can scrape them directly.
- **Exporter Pattern:** Most databases (like MySQL and Redis) do not speak Prometheus natively.
We deploy tiny "translator" pods called **Exporters** alongside them.
The exporter securely connects to the database, reads its internal stats, and translates them into a format Prometheus can scrape.

---

## Prerequisites

1. **Docker Desktop** with Kubernetes enabled.
2. `kubectl` installed and configured to point to your local cluster (`docker-desktop` context).
3. Existing running database pods (`minio`, `mysql`, `redis`).

---

## Step 1: Deploy Database Exporters & Native Endpoints

### 1. Redis Exporter

We use the `oliver006/redis_exporter` to translate Redis metrics.
Save as `redis-monitor.yaml` and apply it using `kubectl apply -f redis-monitor.yaml`.


### 2. MySQL Exporter

We use the official `prom/mysqld-exporter`.
**Note:** Modern versions of this exporter require passing credentials via specific arguments and environment variables, avoiding the deprecated `DATA_SOURCE_NAME` string.
Save as `mysql-monitor.yaml` and apply.


### 3. MinIO (Native)

MinIO does not need an exporter.
It only requires the environment variable `MINIO_PROMETHEUS_AUTH_TYPE=public` added to its primary deployment manifest so Prometheus can scrape its `/minio/v2/metrics/cluster` path without authentication.

---

## Step 2: Deploy Prometheus

Prometheus acts as the brain. We use a `ConfigMap` to explicitly tell it where our services and exporters live.
Save as `prometheus.yaml` and run `kubectl apply -f prometheus.yaml`.


---

## Step 3: Fully Automated Grafana

We deploy Grafana using Infrastructure as Code. Data sources and Dashboards are automatically provisioned upon pod startup.

### 3.1 Download and Patch Dashboards (Mac CLI)

Run these commands in your terminal to download the community dashboards and patch their wildly inconsistent data source variables so they map to our exact Prometheus setup:

```bash
# 1. Download JSONs
curl -sL https://grafana.com/api/dashboards/13502/revisions/latest/download > minio.json
curl -sL https://grafana.com/api/dashboards/763/revisions/latest/download > redis.json
curl -sL https://grafana.com/api/dashboards/7362/revisions/latest/download > mysql.json

# 2. Patch Data Source Variables to standard "Prometheus" string
sed -i '' 's/${DS_PROMETHEUS}/Prometheus/g' minio.json
sed -i '' 's/${DS_FROM}/Prometheus/g' redis.json
sed -i '' 's/${DS_PROM}/Prometheus/g' redis.json
sed -i '' 's/${DB_PROMETHEUS}/Prometheus/g' mysql.json

# 3. Create the K8s ConfigMap
kubectl create configmap grafana-dashboard-jsons --from-file=redis.json --from-file=mysql.json --from-file=minio.json

```

### 3.2 Deploy Grafana

Save as `grafana.yaml` and run `kubectl apply -f grafana.yaml`. This maps the config maps we generated into the Grafana container.


## Accessing the Dashboards

Because we configured `LoadBalancer` services in Docker Desktop, you can access the UIs directly via `localhost`:

- **Prometheus:** `http://localhost:9090` (Verify target statuses under Status -> Targets).
- **Grafana:** `http://localhost:3000` (Login: `admin` / `admin123`).
Your Dashboards will be auto-populated in the "Dashboards" menu.

