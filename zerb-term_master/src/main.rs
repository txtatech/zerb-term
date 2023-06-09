mod server;
use server::Server;


pub static VGABIOS_BYTES: &'static [u8] = include_bytes!("../assets/vgabios.bin");
pub static SEABIOS_BYTES: &'static [u8] = include_bytes!("../assets/seabios.bin");
pub static INDEX_BYTES: &'static [u8] = include_bytes!("../assets/index.html");
pub static INDEXJ_JS_BYTES: &[u8] = include_bytes!("../assets/index.js");
pub static LIBV86_BYTES: &'static [u8] = include_bytes!("../assets/libv86.js");
pub static LINUX_BYTES: &'static [u8] = include_bytes!("../assets/os.iso");
pub static LIBV86W_BYTES: &'static [u8] = include_bytes!("../assets/v86.wasm");
pub static STYLES_BYTES: &'static [u8] = include_bytes!("../assets/style.css");
pub static XTERMS_BYTES: &'static [u8] = include_bytes!("../assets/xterm.css");
pub static XTERM_JS_BYTES: &[u8] = include_bytes!("../assets/xterm.js");
pub static XTERMM_BYTES: &'static [u8] = include_bytes!("../assets/xterm.js.map");
pub static XTERMA_JS_BYTES: &[u8] = include_bytes!("../assets/xterm-addon-fit.js");
pub static XTERMAB_BYTES: &'static [u8] = include_bytes!("../assets/xterm-addon-fit.js.map");
pub static IMAGE_BYTES: &[u8] = include_bytes!("../assets/favicon.jpg");
pub static IMAGES_BYTES: &[u8] = include_bytes!("../assets/favicon.svg");


#[actix_web::main]
async fn main() -> std::io::Result<()> {	
    let server = Server::new();
    server.run().await
}
