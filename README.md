# rust_csv_stream
An attempt at streaming a csv file, line by line from a hyper body in rust

### Run without streaming
Terminal 1: `cd no-stream && cargo run`

Terminal 2: `curl 127.0.0.1:8085`

### Run attempt at streaming with csv crate
Terminal 1: `cd with-stream && cargo run`

Terminal 2: `curl 127.0.0.1:8085`

### Run solution for streaming with csv_async crate
Terminal 1: `cd solution && cargo run`

Terminal 2: `curl 127.0.0.1:8085`


This is code discussed in this thread: [Stream CSV file from hyper body into Deserializer](https://users.rust-lang.org/t/stream-csv-file-from-hyper-body-into-deserializer/54565)