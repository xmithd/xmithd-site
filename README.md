## Deployment instructions
Docker has been added to ease deployment.
Copy config.json.example to config.json and add your custom information.

## How to run locally
To run locally for development, follow these instructions.

### Step 1
Clone this project

### Step 2
Copy the `config.json.example` to `config.json`. Change `config.json` to customize it to your needs.

### Step 3: Compile the source
Use cargo to compile the source code.
```
cargo build
```
Add a `--release` option for release.

### Step 4: run with the desired log variables:
From the root directory, run:
```
RUST_LOG=xmithd_backend=debug,actix_web=info cargo run
```

That's it!
Note that the log outputs in the standard output.
Feel free to customize from here :)

## Note
On my setup, I have NGINX as a reverse proxy. NGINX can host SPA apps and use this project to serve requests.
