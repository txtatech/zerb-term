# zerb-term
A Rust http server that boots two memory resident WebTerm instances in the browser simultaneously.

This is a port of the WebTerm project found here: https://github.com/minie4/WebTerm

**This is not a production server. It is intended for sandboxed testing purposes only.**

There are four available terminals in this server but only two are displayed at: http://127.0.0.1:8080

**Note:**

Each time a page is opened it launches a new instance of the chosen terminal from memory.

There is an embedded html site in the server.rs file itself which contains a 1115 second refresh timer. The timer is not required and can be adjusted/deleted.

Also within the embeded site located in the server.rs file there are two i-frames to simplify interacting with the terminals.

All files for the server are loaded as/from a byte array at boot time.

The default directory the server serves from is 'static' but is not required for the server to operate.

This server has open routes, handles and static variables for the following files:

vgabios.bin: Path: /vgabios.bin, Content-Type: application/octet-stream
seabios.bin: Path: /seabios.bin, Content-Type: application/octet-stream
index.html: Path: /index.html, Content-Type: text/html
index.js: Path: /index.js, Content-Type: application/javascript
libv86.js: Path: /libv86.js, Content-Type: application/javascript
os.iso: Path: /os.iso, Content-Type: application/octet-stream
v86.wasm: Path: /v86.wasm, Content-Type: application/wasm
style.css: Path: /style.css, Content-Type: text/css
xterm.css: Path: /xterm.css, Content-Type: text/css
xterm.js: Path: /xterm.js, Content-Type: application/javascript
xterm.js.map: Path: /xterm.js.map, Content-Type: application/json
xterm-addon-fit.js: Path: /xterm-addon-fit.js, Content-Type: application/javascript
xterm-addon-fit.js.map: Path: /xterm-addon-fit.js.map, Content-Type: application/json
favicon.jpg: Path: /favicon.jpg, Content-Type: image/jpeg
favicon.svg: Path: /favicon.svg, Content-Type: image/svg+xml

The server operates on the following addresses and ports:

http://127.0.0.1:8081

http://127.0.0.1:8080 <---Used to boostrap--->

http://127.0.0.1:8080/index.html

http://127.0.0.1:8081/index1.html

**BUILD NOTE:** Built with Rustc 1.72.0-nightly and Cargo v0.9.4

```markdown
# zerb-term

## Description
`zerb-term` is a Rust package for handling web server functionality. It leverages the Actix framework along with other dependencies to provide a lightweight and efficient web server implementation.

## Installation

### Prerequisites
- Rust programming language (minimum version: 1.55.0)

### Steps

1. Install Rust: Follow the official Rust installation guide based on your operating system. Visit [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) for detailed instructions.

2. Clone the `zerb-term` repository:

   ```plaintext
   git clone https://github.com/your-username/zerb-term.git
   ```

   Alternatively, if you have already downloaded the package, navigate to its root directory.

3. Build and run the package:

   cargo run

This command will download and compile all the necessary dependencies, and then start the web server.

4. Access the web server:

Open your web browser and visit:
`http://localhost:8080`, `http://localhost:8081`, `http://localhost:8080/index.html`, `http://localhost:8081/index1.html`

## Dependencies

- `actix-files` version 0.6.2: A library for serving files with Actix Web.
- `actix-web` version 4: A powerful, yet lightweight framework for building web applications in Rust.
- `hyper` version 0.14.23: A fast and correct HTTP implementation in Rust.
- `rocket` version 0.4.11: A web framework for Rust that focuses on ease of use, stability, and performance.
- `tokio` version 1.24.1: An asynchronous runtime for Rust that provides a framework for writing reliable, asynchronous, and concurrent applications.


