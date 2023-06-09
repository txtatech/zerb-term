use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files as files;
use tokio::try_join;

#[get("/vgabios.bin")]
async fn get_vgabios() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/octet-stream"))
        .body(crate::VGABIOS_BYTES)
}

#[get("/seabios.bin")]
async fn get_seabios() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/octet-stream"))
        .body(crate::SEABIOS_BYTES)
}

#[get("/index.html")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/html"))
        .body(crate::INDEX_BYTES)
}

#[get("/index.js")]
async fn get_indexj_js() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/javascript"))
        .body(crate::INDEXJ_JS_BYTES)
}

#[get("/libv86.js")]
async fn get_libv86() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/javascript"))
        .body(crate::LIBV86_BYTES)
}

#[get("/os.iso")]
async fn get_linux() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/octet-stream"))
        .body(crate::LINUX_BYTES)
}

#[get("/v86.wasm")]
async fn get_libv86w() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/wasm"))
        .body(crate::LIBV86W_BYTES)
}

#[get("/style.css")]
async fn get_styles() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/css"))
        .body(crate::STYLES_BYTES)
}

#[get("/xterm.css")]
async fn get_xterms() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/css"))
        .body(crate::XTERMS_BYTES)
}

#[get("/xterm.js")]
async fn get_xterm_js() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/javascript"))
        .body(crate::XTERM_JS_BYTES)
}

#[get("/xterm.js.map")]
async fn get_xtermm() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .body(crate::XTERMM_BYTES)
}

#[get("/xterm-addon-fit.js")]
async fn get_xterma_js() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/javascript"))
        .body(crate::XTERMA_JS_BYTES)
}

#[get("/xterm-addon-fit.js.map")]
async fn get_xtermab() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .body(crate::XTERMAB_BYTES)
}

#[get("/favicon.jpg")]
async fn get_image() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "image/jpeg"))
        .body(crate::IMAGE_BYTES)
}

#[get("/favicon.svg")]
async fn get_images() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "image/svg+xml"))
        .body(crate::IMAGES_BYTES)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(r#"	
        <html>
        <head>
            <meta http-equiv="refresh" content="1115; URL='index.html'">
			<a href="http://127.0.0.1:8081">localhost:8081-Backbone</a>
            <a href="http://127.0.0.1:8080">localhost:8080-Backbone</a>
			<a href="http://127.0.0.1:8080/index.html">localhost:8080-Index</a>
            <a href="http://127.0.0.1:8081/index.html">localhost:8081-Index</a>			           
        </head>
<body>
    <div class="screen-container" id="screen-container1">
        <iframe src="http://127.0.0.1:8080/index.html" width="650" height="390" frameborder="0"></iframe>
		</div>
		    <div class="screen-container" id="screen-container2">
        <iframe src="http://127.0.0.1:8081/index.html" width="650" height="390" frameborder="0"></iframe>
		</div>
</body>			
</html>
    "#)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub struct Server;

impl Server {
    pub fn new() -> Server {
        Server
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let server1 = HttpServer::new(|| {
            App::new()
                .service(get_vgabios)
                .service(get_seabios)
                .service(get_index)
                .service(get_image)
                .service(get_images)
                .service(get_indexj_js)
                .service(get_libv86)
                .service(get_linux)
                .service(get_libv86w)
                .service(get_styles)
                .service(get_xterms)
                .service(get_xterm_js)
                .service(get_xtermm)
                .service(get_xterma_js)
                .service(get_xtermab)
				.service(hello)
                .service(echo)
                .route("/hey", web::get().to(manual_hello))
                .default_service(files::Files::new("/", "static").index_file("index.html"))
        })
        .bind("127.0.0.1:8080")?
        .run();

        let server2 = HttpServer::new(|| {
            App::new()
                .service(get_vgabios)
                .service(get_seabios)
                .service(get_index)
                .service(get_indexj_js)
                .service(get_libv86)
                .service(get_linux)
                .service(get_libv86w)
                .service(get_styles)
                .service(get_xterms)
                .service(get_xterm_js)
                .service(get_xtermm)
                .service(get_xterma_js)
                .service(get_xtermab)
				.service(hello)
                .service(echo)
                .route("/hey", web::get().to(manual_hello))
                .default_service(files::Files::new("/", "static").index_file("index.html"))
        })
        .bind("127.0.0.1:8081")? // Bind to a different port
        .run();

        match tokio::try_join!(server1, server2) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
