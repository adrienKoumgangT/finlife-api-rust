#!/bin/bash

# Function to kill background processes when you press Ctrl+C
cleanup() {
    echo ""
    echo "Stopping all tunnels..."
    kill $MYSQL_PID $REDIS_PID $MINIO_PID 2>/dev/null
    echo "Done. Bye!"
    exit
}

# Trap the SIGINT signal (Ctrl+C) to run the cleanup function
trap cleanup SIGINT

echo "=================================================="
echo "🔌 Starting Kubernetes Tunnels for Local Dev..."
echo "=================================================="

# 1. MySQL
echo "Starting MySQL tunnel (3306)..."
kubectl port-forward svc/mysql 3306:3306 > /dev/null 2>&1 &
MYSQL_PID=$!

# 2. Redis
echo "Starting Redis tunnel (6379)..."
kubectl port-forward service/redis 6379:6379 > /dev/null 2>&1 &
REDIS_PID=$!

# 3. Minio (Browser + API)
echo "Starting Minio tunnel (9000, 9001)..."
kubectl port-forward svc/minio 9000:9000 9001:9001 > /dev/null 2>&1 &
MINIO_PID=$!

# 4. Grafana
echo "Starting Grafana tunnel (8080)..."
kubectl port-forward svc/monitoring-grafana 8080:80 -n monitoring > /dev/null 2>&1 &
MINIO_PID=$!



echo "=================================================="
echo "✅ Tunnels active!"
echo "   MySQL:   localhost:3306"
echo "   Redis:   localhost:6379"
echo "   Minio:   localhost:9001 (Browser) / :9000 (Bolt)"
echo "   Grafana:   localhost:8080"
echo ""
echo "Press Ctrl+C to stop everything."
echo "=================================================="

# Wait indefinitely keeps the script running
wait
