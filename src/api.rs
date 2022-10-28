use warp::{Filter};

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let index = serve_index();

    index
        .or(config())
        .or(syncs())
        .or(filters())
}

#[cfg(not(debug_assertions))]
fn serve_index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::dir("./www"));
    index
        .or(warp::path("assets")
            .and(warp::get())
            .and(warp::fs::dir("./www/assets")))
}

#[cfg(debug_assertions)]
fn serve_index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .map(|| {
            warp::redirect(warp::http::Uri::from_static("http://127.0.0.1:5173/"))
        })
}

fn config() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get = warp::path("api")
        .and(warp::path("config"))
        .and(warp::get())
        .and_then(handlers::get_config);
    let put = warp::path("api")
        .and(warp::path("config"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(handlers::update_config);

    get.or(put)
}

fn syncs() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get = warp::path("api")
        .and(warp::path("syncs"))
        .and(warp::get())
        .and_then(handlers::get_syncs);
    let put = warp::path("api")
        .and(warp::path("syncs"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(handlers::update_syncs);

    get.or(put)
}

fn filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get = warp::path("api")
        .and(warp::path("filters"))
        .and(warp::get())
        .and_then(handlers::get_filters);
    let put = warp::path("api")
        .and(warp::path("filters"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(handlers::update_filters);

    get.or(put)
}

mod handlers {
    use std::convert::Infallible;
    use warp::http::StatusCode;
    use crate::config;
    use crate::config::{Config, SearchFilters, SyncDir};

    pub async fn get_config() -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::json(&Config::read()))
    }

    pub async fn update_config(config: Config) -> Result<impl warp::Reply, Infallible> {
        match config.persist() {
            Ok(()) => Ok(StatusCode::OK),
            Err(e) => {
                eprintln!("[API PUT CONFIG] Error updating config: {:?}", e);
                Ok(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    pub async fn get_syncs() -> Result<impl warp::Reply, Infallible> {
        let syncs = config::get_sync_folders();
        Ok(warp::reply::json(&syncs))
    }

    pub async fn update_syncs(syncs: Vec<SyncDir>) -> Result<impl warp::Reply, Infallible> {
        match config::update_sync_folders(syncs) {
            Ok(()) => Ok(StatusCode::OK),
            Err(e) => {
                eprintln!("[API PUT SYNCS] Error updating syncs: {:?}", e);
                Ok(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    pub async fn get_filters() -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::json(&config::get_filters()))
    }

    pub async fn update_filters(filters: SearchFilters) -> Result<impl warp::Reply, Infallible> {
        match config::update_filters(filters) {
            Ok(()) => Ok(StatusCode::OK),
            Err(e) => {
                eprintln!("[API PUT FILTERS] Error updating filters: {:?}", e);
                Ok(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}