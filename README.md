# rust-rust-playground

This is a Proof of Concept to interact with the Rust Playground via a simple Rust-backed page.

The user should be able to enter code either through the RESTful interface, or via the simple page provided to send code to the Rust Playground, and see the results of code execution.

## Usage

From the project root:

```
$ cargo run --release
```

The server should start on the default port of `8000`. You can change this port (and the host) in `config/env.toml`.

To access the REST endpoint, open a browser session to `localhost:8000`.

To execute using `curl`:

```
$ curl -v -d 'edition=2018&channel=nightly&mode=debug&operation=run&code=fn main() { println!("Hello, world!"); }' -H "Content-Type: application/json" localhost:8000/execute
```
Note that the following params are all **mandatory**:

   - `edition` (possible values are `2015` and `2018`)
   - `channel` (possible values are `stable`, `beta`, `nightly`)
   - `mode` (possible values are `debug` and `release`)
   - `operation` (possible values are `run`, `build`, or `test`), and
   - `code` - your code goes here

### Sample run

```
$ curl -v -d 'edition=2018&channel=nightly&mode=debug&operation=run&code=fn main() { println!("Hello, world!"); }' -H "Content-Type: application/json" localhost:8000/execute
*   Trying ::1...
* TCP_NODELAY set
* Connected to localhost (::1) port 8000 (#0)
> POST /execute HTTP/1.1
> Host: localhost:8000
> User-Agent: curl/7.54.0
> Accept: */*
> Content-Type: application/json
> Content-Length: 99
>
* upload completely sent off: 99 out of 99 bytes
< HTTP/1.1 200 OK
< Content-Length: 204
< Content-Type: application/json
< Date: Sat, 17 Nov 2018 11:21:20 GMT
<
* Connection #0 to host localhost left intact
{"stderr":"   Compiling playground v0.0.1 (/playground)\n    Finished dev [unoptimized + debuginfo] target(s) in 0.49s\n     Running `target/debug/playground`\n","stdout":"Hello, world!\n","success":true}
```

Copyright © 2018 Timmy Jose

