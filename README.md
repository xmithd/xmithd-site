This is the repository for my personal site: xmithd.com

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
Note that the log outputs in the standard error output.
Feel free to customize from here :)

## Note
On my setup, I have NGINX as a reverse proxy. NGINX can host SPA apps and use this project to serve requests.

## Deployment

### On the build machine

Make sure `config.json` is set with the right values. It will get copied to the docker image.

Build the docker image using the following command (replace the version number):
```
$ docker image build -t website:1.0.1 .
```
Tag the changes:
```
$ docker image tag website:1.0.1 xmithd/website:1.0.1
```
Making sure you're logged in to the registry and push:
```
$ docker image push xmithd/website:1.0.1
```

### On the server

Pull the image (make sure you're logged in):
```
$ docker pull xmithd/website:1.0.1
```

Stop the running container:
```
$ docker container stop xmithd.com
```

Delete it (optional: make sure to name the new one something else if not deleting it)
```
$ docker container rm xmithd.com
```

Run the container 😁
(notice, I run it with 2 volumes - one is for external files (images, static assets that I don't want in git, etc...) and the other one is for the database.)
```
$ docker run -d --restart=unless-stopped --name xmithd.com -p 3001:3001 -v $HOME/database:/path/to/container/database -v $HOME/external_files:/path/to/container/external_files xmithd/website:1.0.1
```
Get the logs
```
$ docker logs xmithd.com
```

Get some stats
```
$ docker stats xmithd.com
```
