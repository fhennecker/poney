# poney server


### Build and run using docker

Note that the first time you run the build (and subsequent times when your `Cargo.toml`
file changes) will be longer due to the download and build of dependencies.

First, build the image.
```
docker build -t poneyserver .
```

You have two options to run it:
- Interactive:
  ```
  docker run -p 9001:9001 -it --init poneyserver
  ```
- Daemon:
  ```
  docker run -p 9001:9001 -dit --init poneyserver
  ```
