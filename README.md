# My First Rust Webservice (Iron) and Client (Hyper)

Just a small experiment for learning Rust with Iron for implementing the webservice and Hyper for implementing the client. 

By default the client is posting messages to theserver (`POST http://localhost:3000/set`), which then can be retrieved (`GET http://localhost:3000/`).

## Starting the Server

```
 cargo run --bin my_webservice
```

## Running the Client

```
cargo run --bin client 
```
