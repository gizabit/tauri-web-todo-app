# Tauri/Web Todo App

- [Tauri/Web Todo App](#tauriweb-todo-app)
  - [Overview](#overview)
  - [Tech Stack](#tech-stack)
  - [Features](#features)
  - [Prerequisites](#prerequisites)
  - [Project Structure](#project-structure)
  - [Running the Project for Development](#running-the-project-for-development)
    - [Web Application](#web-application)
    - [Desktop Application](#desktop-application)
  - [Deploying the Project](#deploying-the-project)
    - [Web Application (as a Docker Container)](#web-application-as-a-docker-container)
    - [Desktop Application](#desktop-application-1)
  - [Troubleshooting](#troubleshooting)
    - [Error while Building Tauri on Linux](#error-while-building-tauri-on-linux)
    - [Blank Window when Starting the Tauri App on Linux](#blank-window-when-starting-the-tauri-app-on-linux)

## Overview

This project demonstrates how to use a single codebase to build both a web application and a desktop application using [Tauri](https://tauri.app), with the frontend written in [Svelte](https://svelte.dev). The backend logic, written in Rust, is shared across both environments.

For simplicity, the Todo tasks are managed in memory. This means that the tasks will not persist between application restarts.

## Tech Stack

- **Business Logic**: Rust
- **Frontend**: [Svelte](https://svelte.dev) with TypeScript and [TailwindCSS](https://tailwindcss.com)
- **Web Backend**: Rust using [axum](https://github.com/tokio-rs/axum) for the REST API
- **Desktop App**: [Tauri](https://tauri.app) for desktop app development

## Features

- **Single Codebase**: Shared business logic for both web and desktop platforms
- **Docker Support**: Ability to deploy the web application as a Docker container
- **Tauri Desktop Application**: A lightweight desktop app
- **Cross-Platform Compatibility**: Desktop app works on Windows, Linux and more.

## Prerequisites

Before running the project, ensure you have the following installed:

- [Node.js](https://nodejs.org) for building the frontend and the Tauri application
- [Rust](https://www.rust-lang.org) for building the backend and the Tauri application
- [Tauri prerequisites](https://tauri.app/start/prerequisites) for setting up the Tauri environment

## Project Structure

```shell
├── backend/          # Rust backend code (Axum-based API)
│   ├── src/
│   ├── Cargo.toml
│   └── ...
├── frontend/         # Svelte frontend code
│   ├── src/
│   ├── ...
│   └── tauri/        # Tauri desktop app
│       ├── src/
│       ├── Cargo.toml
│       └── ...
├── logic/            # Shared business logic (Rust library)
│   ├── src/
│   ├── Cargo.toml
│   └── ...
├── Dockerfile        # Docker configuration for building the project
└── ...
```

Note that the tauri directory needs to be a subdirectory of the frontend.

## Running the Project for Development

### Web Application

To run the web application for development, follow these steps:

1. Run the backend:

```shell
    cd backend
    cargo run
```

2. Install frontend dependencies (if not yet done):

```shell
    cd frontend
    npm install
```

3. Run the frontend development server:

```shell
    npm run dev
```

Note: By default, the backend listens on `localhost:3366`. To change the port, you can set the `LOCAL_PORT` environment variable for the backend and `VITE_APP_BACKEND_URL` for the frontend. For example, to run the backend on a different port (e.g. 1234) on Linux:

- Start the backend:

```shell
    LOCAL_PORT=1234 cargo run
```

- Start the frontend:

```shell
    VITE_APP_BACKEND_URL='http://localhost:1234' npm run dev
```

### Desktop Application

To build and run the Tauri desktop application in a development environment, use the following command:

```shell
    cd frontend
    npm run tauri dev
```

## Deploying the Project

### Web Application (as a Docker Container)

To build the docker container, use the following command in the root of the repository:

```shell
    docker build -t demo-app .
```

To start the container, use:

```shell
    docker run -p 1234:80 -e BACKEND_URL=http://localhost:1234 demo-app
```

The `BACKEND_URL` specifies the URL where the frontend connects to the REST API. Make sure the port in `BACKEND_URL` matches the mapped port in the docker run command.

### Desktop Application

To build the desktop application, run the following command:

```shell
    npm run tauri build
```

This will generate the package file or installer, depending on your operating system. The output can be found under `frontend/tauri/target/release/bundle/...`.

## Troubleshooting

### Error while Building Tauri on Linux

**Issue:**

```shell
failed to bundle project: error running build_appimage.sh: `failed to run /.../frontend/tauri/target/release/bundle/appimage/build_appimage.sh`
    Error failed to bundle project: error running build_appimage.sh: `failed to run /.../frontend/tauri/target/release/bundle/appimage/build_appimage.sh`
```

**Workaround:**

Build the application with `NO_STRIP=true`:

```shell
    NO_STRIP=true npm run tauri build
```

See https://github.com/tauri-apps/tauri/issues/8929

### Blank Window when Starting the Tauri App on Linux

**Issue:**

When starting the application, the window remains blank, and the following message appears in the terminal:

```shell
Failed to create GBM buffer of size 1280x695: Invalid argument
```

**Workaround:**

Start the application with the following command:

```shell
WEBKIT_DISABLE_COMPOSITING_MODE=1 demo-app.AppImage
```

See https://github.com/tauri-apps/tauri/issues/8254
