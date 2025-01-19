# Rust Petstore Microservice

This is a simple microservice implemented in Rust using Actix-Web framework, with Redis as the database backend. The microservice provides basic CRUD operations for managing pets.

## Features

- Add a new pet
- Retrieve all pets
- Retrieve a specific pet by ID
- Delete a pet by ID

## Requirements

- Rust programming language
- Redis server

## Setup

 **Clone the repository:**

   ```bash
   git clone https://github.com/your_username/your_repository.git
   ```
1. Install Rust:

If you haven't already installed Rust, you can do so using Rustup, Rust's official toolchain installer.

2. Install Redis:

Install and run Redis server on your local machine or a remote server. You can download Redis from here or install it using your package manager.

3. Set up your environment variables:

Create a .env file in the root directory of the project and configure your Redis connection settings. Example:

```bash
redisURI=redis://127.0.0.1
serverAddr=localhost:8080
```

4. Build the project:

Navigate to the project directory and run:

```bash
cargo build --release
```

5. Run the microservice:

After building the project, you can run the microservice using:

```bash
cargo run --release
```

6. Access the API endpoints:

Once the microservice is running, you can access the API endpoints using tools like cURL or Postman. Here are some example requests:

- Add a pet:

```bash

curl -X POST -H "Content-Type: application/json" -d '{"id": 1, "name": "Fluffy", "category": "Dog"}' http://127.0.0.1:8080/pet

```
- Retrieve all pets:

```bash
curl http://127.0.0.1:8080/
```

- Retrieve a specific pet (with ID 1 in this example):

```bash
curl http://127.0.0.1:8080/pet/1
```
- Delete a pet (with ID 1 in this example):

```bash
curl -X DELETE http://127.0.0.1:8080/pet/1
```


# Contributing
Contributions are welcome! Feel free to open issues or pull requests for any improvements or bug fixes.

# License
This project is licensed under the MIT License.