# Rust Petstore Microservice

A premium, high-performance Petstore API built with **Rust**, **Actix-Web**, and **MongoDB**.

## 🚀 Features

- **Full CRUD**: Manage pets and users with ease.
- **Search**: Advanced search by status, tags, and name.
- **Security**: CORS-ready and environment-based configuration.
- **Robustness**: Proper error handling and logging.
- **Dockerized**: Optimized multi-stage Docker builds.

## 🛠 Tech Stack

- **Backend**: [Rust](https://www.rust-lang.org/)
- **Web Framework**: [Actix-Web](https://actix.rs/)
- **Database**: [MongoDB](https://www.mongodb.com/)
- **Serialization**: [Serde](https://serde.rs/)

## 📋 Requirements

- Rust 1.80+
- MongoDB 6.0+

## ⚙️ Setup

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/afinana/rust-server-petstore.git
    cd rust-server-petstore
    ```

2.  **Environment Variables:**

    Create a `.env` file or export the following variables:

    ```bash
    DATABASE_URI=mongodb://root:example@localhost:27017/?authSource=admin
    SERVER_ADDR=127.0.0.1:8080
    RUST_LOG=actix_web=info,rust_server_petstore=debug
    ```

3.  **Run with Cargo:**

    ```bash
    cargo run
    ```

4.  **Run with Docker:**

    ```bash
    docker build -t petstore-rust .
    docker run -p 8080:8080 --env-file .env petstore-rust
    ```

## 🛣 API Endpoints

All endpoints are prefixed with `/v2`.

### Pets
- `GET /v2/pet` - List all pets
- `POST /v2/pet` - Add a new pet
- `PUT /v2/pet` - Update an existing pet
- `GET /v2/pet/{id}` - Find pet by ID
- `DELETE /v2/pet/{id}` - Remove a pet
- `GET /v2/pet/findByStatus?status=available` - Find pets by status
- `GET /v2/pet/findByTags?tags=tag1,tag2` - Find pets by tags
- `GET /v2/pet/name/{name}` - Find pets by name

### Users
- `GET /v2/user` - List all users
- `POST /v2/user` - Create user
- `POST /v2/user/createWithList` - Create multiple users
- `GET /v2/user/login?username=...&password=...` - User login
- `GET /v2/user/logout?username=...` - User logout
- `GET /v2/user/{username}` - Get user by username
- `PUT /v2/user/{username}` - Update user
- `DELETE /v2/user/{username}` - Delete user

## 🧪 Testing

You can test the API using `curl`:

```bash
# Add a pet
curl -X POST http://localhost:8080/v2/pet \
  -H "Content-Type: application/json" \
  -d '{"id": 101, "name": "Buster", "category": {"id": 1, "name": "Dog"}, "photoUrls": [], "tags": [], "status": "available"}'

# Get pet
curl http://localhost:8080/v2/pet/101
```

## 📄 License

This project is licensed under the MIT License.