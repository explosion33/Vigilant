use std::path::{Path, PathBuf};

use rocket::{
    self,
    Config,
    fs::NamedFile,
};

use rocket_dyn_templates::Template;


#[rocket::get("/")]
fn index() -> Template {
    Template::render("index", rocket_dyn_templates::context!{})
}


#[rocket::get("/static/<file>")]
async fn get_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}

pub fn start_api() {
    rocket::tokio::runtime::Builder::new_multi_thread()
        .worker_threads(Config::from(Config::figment()).workers)
        // NOTE: graceful shutdown depends on the "rocket-worker" prefix.
        .thread_name("rocket-worker-thread")
        .enable_all()
        .build()
        .expect("create tokio runtime")
        .block_on(async move {
            let _ = rocket::build()
            .mount("/", rocket::routes![index, get_file])
            .attach(Template::fairing())
            //.manage()
            .launch()
            .await;
        });
}