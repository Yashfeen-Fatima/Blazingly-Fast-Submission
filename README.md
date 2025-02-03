# URL Shortener with Rust

## Blazingly Fast Hackathon by HackathonRaptors

Welcome to our submission for the **Blazingly Fast Hackathon**! Our project is a **URL Shortener** built using **Rust**. We wanted to create a high-performance, reliable, and efficient URL shortener that can handle large volumes of requests with minimal latency — which is why we chose Rust for this project.

## Team

- **Yashfeen Fatima**: Yashfeen helped in managing data solutions, ensuring the system could scale and handle analytics in the future.
- **Syed Vilayat Ali Rizvi**: Focused on designing and implementing the URL shortener with Rust, building the core backend logic, and ensuring the system’s speed and efficiency.

## Features

- **URL Shortening**: Provides a simple API to shorten URLs.
- **Redirecting**: Shortened URLs automatically redirect to their original destination.
- **High-Performance**: Built using Rust for low-latency and high-concurrency handling.
- **Simple REST API**: Easily integrates into your projects or applications.
- **Data Management**: Optimized data storage for scalability, includes caching and db persistence.

## Tech Stack

- **Programming Language**: Rust
- **Backend Framework**: Axum (for building fast web servers in Rust)
- **Database**: Postgres (for data storage and URL mapping)
- **Other Tools**: 
    -  SQLX Query Builder (for Rust-based database interaction)
    -  Docker for containerization
    -  Docker Compose for container orchestration

## Installation

To get started with the URL shortener, clone the repository and run the project locally.

### Prerequisites

If you have docker installed in your computer then you can directly containerize the project and run on your computer!

### Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/Yashfeen-Fatima/Blazingly-Fast-Submission.git url-shortner-rust
   cd url-shortener-rust
   ```

2. Install SQLX_CLI

    ```bash
    cargo sqlx db create
    ```

    ```bash
    cargo sqlx migrate run
    ```

3. Run

    ```bash
    cargo r --release
    ```

## Contributing

We welcome contributions to this project! If you have ideas for improvements or additional features, feel free to fork the repo and create a pull request. Be sure to follow the general Rust best practices and maintain the performance-focused nature of the project.

## Acknowledgements

- Special thanks to HackathonRaptors for hosting the Blazingly Fast Hackathon.
- Thanks to Yashfeen Fatima for her contributions in data management and handling.

## License

This project is licensed under the MIT License - see the [LICENSE FILE](./LICENSE) file for details.