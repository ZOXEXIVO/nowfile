# NOWFILE

This repo contains the code that helps creating http proxy under you on-premise AWS S3 account written on pure Rust

### How to run

```bash

export ENDPOINT = {AWS S3 endpoint} (default - http://localhost:4572 (localstack)
export ACCESS_KEY = {AWS S3 ACCESS_KEY} (default - 123)
export SECRET_KEY = {AWS S3 SECRET_KEY} (default - 321)
export BUCKET_NAME = {default bucket name for upload | download files} (default - test-bucket)
export TOKEN_KEY = {Key to sign you file token} (default - 123456789)
export POOL_SIZE = {Size for intenal s3 client pool} (default - 50)

 ./nowfile 
```
Docker image will be soon...

### Features

- [x]  Upload file

- [x]  Download file

- [ ]  Create one-time link to file

### Api

##### Upload 
```
POST http:://nowfile

Upload file as multipart form-data

Return {FILE_ID}
```
##### Download 
```
GET http:://nowfile/{FILE_ID}

Download file
```

### Internals

[actix-web](https://github.com/actix/actix-web) http server

### License

Apache License 2.0