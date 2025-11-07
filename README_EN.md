# R2 Gallery

A lightweight image hosting service using Cloudflare R2 for storage.

## Deployment Guide

### Prerequisites

- [Docker](https://www.docker.com/get-started) installed.
- [Docker Compose](https://docs.docker.com/compose/install/) installed.

### Step 1: Get the Project Files

```bash
# Clone the repository
git clone <repository-url>
cd r2-gallery

# Create your environment file from the template
cp .env.example .env
```

### Step 2: Generate Password Hash

Run the following command to generate a secure password hash.

```bash
docker-compose run --rm app cargo run --bin hash_password --features build-tools
```

After entering your password, copy the resulting hash. You will use it to update the `.env` file.

### Step 3: Configure the `.env` File

Open the `.env` file you created in Step 1 and fill in all the configuration details.

- `PASSWORD`: Paste the password hash you generated in Step 2.
- `USERNAME`, `JWT_SECRET`, `R2_*`: Fill in your personal configuration.
- `HOST_PORT`: The port on your machine through which you want to access the application (e.g., `8080`).
- Adjust other variables as needed.

### Step 4: Start the Application

Once configured, run the following command in the project's root directory to build and start your application:

```bash
docker-compose up --build -d
```

- `--build`: Required on the first launch or after code updates to build the image.
- `-d`: Runs the containers in detached mode (in the background).

The application should now be running successfully. You can access it in your browser via the `HOST_PORT` you configured in the `.env` file.

For example: `http://localhost:8080`

## Management Commands

- **View real-time logs:**

  ```bash
  docker-compose logs -f
  ```

- **Stop and remove containers:**

  ```bash
  docker-compose down
  ```

- **Start services in the background:**
  ```bash
  docker-compose up -d
  ```
