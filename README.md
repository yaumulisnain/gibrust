# Gibrust

A command-line tool to scaffold and manage Rust REST API projects with best practices. Gibrust helps you quickly create new projects, generate domain-driven design (DDD) structures, handle database migrations, and manage API documentation.

## Development Setup

1. Install development dependencies:
```bash
make dev-deps
```

This will install:
- cargo-watch (for development watching)
- cargo-edit (for dependency management)
- cargo-update (for updating dependencies)

2. Available development commands:
```bash
make build      # Build the project in debug mode
make build-rel  # Build the project in release mode
make run        # Run the project in debug mode
make run-rel    # Run the project in release mode
make test       # Run all tests
make clean      # Remove build artifacts
make check      # Run cargo check
make fmt        # Format code using rustfmt
make lint       # Run clippy lints
make watch      # Watch for changes and rebuild
```

## CLI Usage

Gibrust provides several commands to help you manage your Rust REST API projects:

### Project Management
```bash
# Create a new REST API project
gibrust create:project --name <project-name> [--dir <directory>]

# Generate a new domain (DDD folder structure)
gibrust create:domain --name <domain-name> [--dir <directory>]
```

### Database Management
```bash
# Generate new migration files
gibrust migrate:generate [--name <migration-name>] [--dir <directory>]

# Run database migrations
gibrust migrate:run [--dir <directory>]
```

### Development & Deployment
```bash
# Run server in development mode
gibrust run:dev [--dir <directory>]

# Build and run in production mode
gibrust run:prod [--dir <directory>]

# Build project binary
gibrust run:build [--dir <directory>]
```

### Documentation
```bash
# Generate OpenAPI documentation
gibrust create:docs [--dir <directory>]
```

Note: For all commands, if `--dir` is not specified, it defaults to the current directory.