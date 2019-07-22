## Deployment instructions
There are several steps to do in order to deploy the app.

### Step 1
Clone this project

### Step 2
Build the ui project. From the ui directory, install the dependencies first:
```
npm install
```
And then run the build:
```
npm run build
```

### Step 3
Build the server project. There is script that uses Docker to build the project. 
From the server directory:
```
./build_prod.sh
```

### Step 4: Upload files to the server
Assuming the root project is at `~/project`, upload the following folders:
`~/project/ui/build` and `~/project/server/static`.
Finally, upload the binary in `./target/release/xmithd_backend` into the `~/project/server/` folder.

### Step 5: run with the desired log variables:
From the `~/project/server` directory, run:
```
RUST_LOG=xmithd_backend=debug ./xmithd_backend
```


That's it!
Note that the log outputs in the standard output.
Feel free to customize from here :)

## Note
On my setup, I have NGINX as a reverse proxy. NGINX is also hosting my static files from the React UI App. Cheers!
