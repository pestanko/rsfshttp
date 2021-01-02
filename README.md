# Simple HTTP file accessor with multiple mappings

This is my toy project to learn basics of Rust programming language.
As http webserver is used `actix`, for logging `slog`.


## API Routes

The service provides these routes:

- GET `/api/v1/map` Get all available mappings, this might be disabled (see config)
- GET `/api/v1/map/<NAME>` Get information about the mapping, with name `<NAME>`
- GET `/api/v1/map/<NAME>/tree` Get FS tree - all files and directories, multiple output formats
- GET `/api/v1/map/<NAME>/info?path=<PATH>` Get file/directory information
- GET `/api/v1/map/<NAME>/content?path=<PATH>` Get file content

Manipulation endpoints (needs to be explicitly allowed):

- POST `/api/v1/map/<NAME>/content?path=<PATH>` Upload a new file content, disabled by default
- DELETE `/api/v1/map/<NAME>/content?path=<PATH>` Deletes a file, disabled by default
 



## Usage

The main sub-command is the `serve` or alias `s`. That will serve the provided directory content


```bash
$ rsfshttp -c ~/config.yml serve -M 'logs:/tmp/app/logs'
- logs: https://localhost:8080/api/v1/map/logs/
```

Serve the directory `/tmp/app/logs` on the mapping: *logs*. 

