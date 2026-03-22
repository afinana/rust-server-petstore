# Agent Instructions for rust-server-petstore

This project is a Rust-based web server using Actix-Web, MongoDB, and Tokio.
When contributing to or modifying this repository, agents must adhere to the following rules:

## Error Handling
- **Avoid the use of `.unwrap()`**. Always use proper error handling, such as returning `Result`, using the `?` operator, or utilizing `.expect("...")` with a clear context message only if a crash is strictly intentional and justified.

## Project Context
- **API Spec:** This microservice implements the [Petstore Swagger API](https://petstore.swagger.io/v2/swagger.json).
- **Frameworks:** Actix-Web, Tokio
- **Database:** MongoDB
- **Serialization:** Serde, Bson
