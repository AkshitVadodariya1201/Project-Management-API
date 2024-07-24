# Project Management API

The Project Management API is a comprehensive Rust-based web application designed to streamline and enhance the management of projects. It is built on the robust Rocket web framework, known for its speed and reliability, and utilizes Async-GraphQL, a powerful library for building GraphQL APIs in Rust. This combination offers a high-performance backend capable of handling complex project management tasks with ease.

## Key Features

- **GraphQL API**: Leverages Async-GraphQL to provide a flexible and efficient API for querying and mutating project data, enabling clients to request exactly what they need and nothing more.
- **Project and Owner Management**: Supports comprehensive management capabilities for projects and their owners, allowing for the creation, update, and retrieval of project information and associated owners.
- **Robust Data Handling**: Utilizes custom JSON handling for persisting project and owner data, ensuring data integrity and ease of access.
- **Integration Testing**: Includes a suite of integration tests covering GraphQL mutations and queries, as well as JSON data handling, ensuring the reliability and stability of the application.

## Architecture

The application is structured around the Rocket web framework and Async-GraphQL library, with the following key components:

- **GraphQL Handler**: Central to the application, the GraphQL handler (`src/handler/graphql_handler.rs`) defines the schema, queries, and mutations for the GraphQL API.
- **Data Models**: Defines the data structures for projects and owners, along with JSON serialization and deserialization logic for persistent storage (`src/config/`, `src/schema/`).
- **Integration Tests**: Ensures the application's functionality through comprehensive integration tests, covering GraphQL operations and JSON data handling (`tests/`).

## Getting Started

To get started with the Project Management API, ensure you have Rust and Cargo installed. Then, you can clone the repository and run the application using Rocket's built-in server.

1. Clone the repository:

   ```sh
   git clone https://github.com/AkshitVadodariya1201/Project-Management-API.git
   ```

2. Change into the project directory:

   ```sh
   cd project-management-api
   ```

3. Run the application using Cargo:
   ```sh
   cargo run
   ```


4. Access the GraphQL playground at `http://localhost:8000` to interact with the API.

## Usage

The Project Management API provides a GraphQL interface for managing projects and their owners. You can perform the following operations using the provided queries and mutations:

- **Queries**:
  - `projects`: Retrieve a list of all projects.
  - `project(id: ID!)`: Retrieve a specific project by ID.
  - `owners`: Retrieve a list of all owners.
  - `owner(id: ID!)`: Retrieve a specific owner by ID.

- **Mutations**:
    - `createProject(input: ProjectInput!)`: Create a new project with the specified input.
    - `updateProject(id: ID!, input: ProjectInput!)`: Update an existing project with the specified ID and input.
    - `deleteProject(id: ID!)`: Delete a project with the specified ID.
    - `createOwner(input: OwnerInput!)`: Create a new owner with the specified input.
    - `updateOwner(id: ID!, input: OwnerInput!)`: Update an existing owner with the specified ID and input.
    - `deleteOwner(id: ID!)`: Delete an owner with the specified ID.

For detailed information on the available queries and mutations, refer to the GraphQL schema in the GraphQL playground.

## Testing

The Project Management API includes a suite of integration tests to ensure the correctness and reliability of the application. You can run the tests using Cargo:

```sh
cargo test
```

The tests cover GraphQL queries and mutations, as well as JSON data handling, to validate the application's functionality.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

The Project Management API was developed as part of a project management course and is intended for educational purposes. It leverages the Rocket web framework and Async-GraphQL library to provide a high-performance backend for managing projects and owners.

For more information on Rocket, Async-GraphQL, and other libraries used in this project, refer to the official documentation:

- [Rocket](https://rocket.rs/)
- [Async-GraphQL](https://async-graphql.github.io/async-graphql/en/index.html)
- [Serde JSON](https://serde.rs/)
- [reqwest](https://docs.rs/reqwest/0.11.4/reqwest/)
- [tokio](https://tokio.rs/)

Happy coding!🦀🚀

