# Deploy local database (MySQL) in Kubernetes in Docker

Prerequisite: Enable Kubernetes
Ensure Kubernetes is running in Docker Desktop:
1. Open Docker Desktop Dashboard
2. Go to **Settings** (gear icon) > **Kubernetes**
3. Check **Enable Kubernetes** and click **Apply and Restart**


## Step 1: Create a Password Secret

Instead of putting the password in plain text in the deployment file, we use a Kubernetes secret to store the password.

Run this command in terminal:
```bash
kubectl create secret generic mysql-pass --from-literal=password=yourpassword
```
Replace `yourpassword` with your actual password.
Example:
```bash
kubectl create secret generic mysql-pass --from-literal=password=approot123
```


## Step 2: Create Persistent Storage (PVC)

We need to claim a slice of storage so your database files survive if the Pod dies.
Docker Desktop come with a default storage class that handles this automatically.

Create a file named `mysql-pvc.yaml` with the following content:
```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: mysql-pvc
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
  storageClassName: standard
```

Apply it:
```bash
kubectl apply -f mysql-pvc.yaml
```

To delete the stuck pvc:
```bash
kubectl delete pvc mysql-pvc
```


## Step 3: Create the MySQL Deployment

This defines the MySQL "application" itself. It connects the Secret (password) and the PVC (storage) to the database container.

Create a file named `mysql-deployment.yaml` with the following content:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: mysql
spec:
  selector:
    matchLabels:
      app: mysql
  strategy:
    type: Recreate
  template:
    metadata:
      labels:
        app: mysql
    spec:
      containers:
        - image: mysql:8.0
          name: mysql
          env:
            # Use the secret we created in Step 1
            - name: MYSQL_ROOT_PASSWORD
              valueFrom:
                secretKeyRef:
                  name: mysql-pass
                  key: password
          ports:
            - containerPort: 3306
              name: mysql
          volumeMounts:
            # Mount the persistent storage to the data directory
            - name: mysql-persistent-storage
              mountPath: /var/lib/mysql
      volumes:
        - name: mysql-persistent-storage
          persistentVolumeClaim:
            claimName: mysql-pvc
```

Apply it:
```bash
kubectl apply -f mysql-deployment.yaml
```


## Step 4: Create a Service

To access the database, you need a Service. This creates an internal IP address for other apps in the cluster to talk to MySQL.

Create a file named `mysql-service.yaml` with the following content:
```yaml
apiVersion: v1
kind: Service
metadata:
  name: mysql
spec:
  ports:
    - port: 3306
  selector:
    app: mysql
```

Apply it:
```bash
kubectl apply -f mysql-service.yaml
```


## Step 5: Verify and Connect

1. **Check status**: Wait a moment for the container to start, then run:
```bash
kubectl get pods
```
You should see `mysql-xxxxxxx` with the status `Running`.
To delete a pod:
```bash
kubectl delete pod mysql-xxxxxxx
```

2. **Connect from your local machine (Port Forwarding)**: Since the database is inside the Kubernetes cluster
, you cannot access it directly from your local terminal (localhost) by default.
Use port-forwarding to bridge the connection:
```bash
kubectl port-forward svc/mysql 3306:3306
```
Now, open your favorite SQL client and connect using:
- Host: `localhost`
- Port: `3306`
- Username: `root`
- Password: (The password you set in Step 1)




```mysql
-- Create the user for the specific IP if they don't exist
CREATE USER 'your_username'@'127.0.0.1' IDENTIFIED BY 'your_password';

-- Grant privileges (adjust privileges as needed, e.g., ALL PRIVILEGES)
GRANT ALL PRIVILEGES ON *.* TO 'your_username'@'127.0.0.1' WITH GRANT OPTION;

-- Apply changes
FLUSH PRIVILEGES;
```
example:
```mysql
-- Create the user for the specific IP if they don't exist
CREATE USER 'appuser'@'127.0.0.1' IDENTIFIED BY 'appuser123';

-- Grant privileges (adjust privileges as needed, e.g., ALL PRIVILEGES)
GRANT ALL PRIVILEGES ON *.* TO 'appuser'@'127.0.0.1' WITH GRANT OPTION;

-- Apply changes
FLUSH PRIVILEGES;
```



