# MG Software Tickets Desktop App

A Tauri-based desktop application for MG Software Tickets that supports multiple environments with a native transparent titlebar on macOS.

## Features

- **Environment Support**: Development, Staging, and Production environments
- **Transparent Titlebar**: Native macOS transparent titlebar with traffic light buttons
- **Responsive Design**: Minimum window size of 1450x900 for optimal experience

## Environment Configuration

The desktop app supports three environments, each loading a different URL:

- **Development**: `http://localhost:3001`
- **Staging**: `https://beta.mgsoftware.nl`
- **Production**: `https://tickets.mgsoftware.nl`

## Running the App

### Development Mode
```bash
# Run in development environment (loads localhost:3001)
bun run tauri:dev
```

### Staging Mode
```bash
# Run in staging environment (loads beta.mgsoftware.nl)
bun run tauri:staging
```

### Production Mode
```bash
# Run in production environment (loads app.refront.ai)
bun run tauri:prod
```

## Building the App

### Development Build
```bash
bun run tauri:build
```

### Staging Build
```bash
bun run tauri:build:staging
```

### Production Build
```bash
bun run tauri:build:prod
```

## Environment Variable

The environment is controlled by the `REFRONT_ENV` environment variable:

- `development` or `dev` → `http://localhost:3001`
- `staging` → `https://beta.mgsoftware.nl`
- `production` or `prod` → `https://tickets.mgsoftware.nl`

If no environment is specified, it defaults to development mode.

## Manual Environment Setting

You can also set the environment manually:

```bash
# macOS/Linux
REFRONT_ENV=staging tauri dev

# Windows (PowerShell)
$env:REFRONT_ENV="staging"; tauri dev

# Windows (Command Prompt)
set REFRONT_ENV=staging && tauri dev
```
