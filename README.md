# Rust Demo App

A simple Rust application designed for testing Tekton CI/CD pipelines.

## Features

- Simple console application with multiple functions
- Unit tests using Rust's built-in test framework
- Dockerfile for containerization
- Compatible with Tekton pipelines

## Project Structure

```
rust-demo-app/
├── Cargo.toml          # Rust package manifest
├── Dockerfile          # Multi-stage Docker build
├── .dockerignore       # Docker ignore file
├── src/
│   └── main.rs         # Main application with tests
└── README.md           # This file
```

## Local Development

### Prerequisites

- Rust 1.75 or later
- Cargo (comes with Rust)

### Running the Application

```bash
cargo run
```

### Running Tests

```bash
cargo test
```

### Running Tests with Verbose Output

```bash
cargo test -- --nocapture
```

## Building Docker Image

```bash
docker build -t rust-demo-app:latest .
```

## Running Docker Container

```bash
docker run --rm rust-demo-app:latest
```

## Testing with Tekton

This application is designed to work with the Tekton pipelines in the parent directory.

### Prerequisites

1. Kubernetes cluster with Tekton installed
2. Tekton tasks and pipelines configured (see parent directory)
3. GitHub repository containing this code
4. AWS ECR repository (if pushing images)

### Pipeline Compatibility

This application works with the `rust-build-and-push` pipeline which:
1. Clones the repository
2. Runs `cargo test` to execute all tests
3. Builds a Docker image using the Dockerfile
4. Pushes the image to AWS ECR

### Example PipelineRun

Create a PipelineRun to test this application:

```yaml
apiVersion: tekton.dev/v1beta1
kind: PipelineRun
metadata:
  name: rust-demo-app-build
spec:
  pipelineRef:
    name: rust-build-and-push
  params:
    - name: repo-url
      value: "https://github.com/YOUR_USERNAME/YOUR_REPO.git"
    - name: repo-name
      value: "rust-demo-app"
    - name: revision
      value: "main"
    - name: ecr-image
      value: "123456789012.dkr.ecr.us-east-1.amazonaws.com/rust-demo-app"
    - name: image-tag
      value: "latest"
    - name: dockerfile-path
      value: "rust-demo-app/Dockerfile"
    - name: test-command
      value: "cd rust-demo-app && cargo test"
  workspaces:
    - name: shared-workspace
      persistentVolumeClaim:
        claimName: source-pvc
    - name: aws-credentials
      secret:
        secretName: aws-ecr-credentials
```

## What Gets Tested

The application includes tests for:
- **Addition function**: Tests basic arithmetic
- **Even number checker**: Tests modulo operations
- **Fibonacci calculator**: Tests recursive logic
- **Greeting function**: Tests string formatting

All tests must pass for the Tekton pipeline to succeed.

## Extending the Application

You can extend this application to test various Tekton pipeline scenarios:

1. **Add more complex tests**: Test failure scenarios
2. **Add dependencies**: Test dependency resolution
3. **Add integration tests**: Test external service calls
4. **Add benchmarks**: Test performance testing in pipelines
5. **Add multiple binaries**: Test multi-binary projects

## License

MIT
# rust-demo
