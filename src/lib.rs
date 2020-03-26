#![recursion_limit = "256"]
extern crate tokio;

#[macro_use]
mod result;

mod canvas;
mod web;
mod websocket;
pub use result::*;

use async_std::sync;
pub use canvas::*;
use tokio::sync::mpsc;
use websocket::WSMessage;

pub fn start_server(web_addr: &str, websocket_addr: &str) -> Result<CanvasGenerator> {
    let (sender, receiver) = sync::channel::<mpsc::Receiver<WSMessage>>(1024);
    tokio::spawn(websocket::start_server(
        websocket_addr.to_string(),
        receiver,
    ));
    tokio::spawn(web::start_server(
        web_addr.to_string(),
        websocket_addr.to_string(),
    ));
    Ok(CanvasGenerator { sender })
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger::Builder;
    use log::LevelFilter;
    use log::*;
    use std::time::Duration;
    use tokio::time::delay_for;

    async fn test_thread(server: CanvasGenerator) {
        info!("making canvas");
        let mut canvas = server
            .make_2d_canvas(Default::default(), 100, 100)
            .await
            .unwrap();
        info!("got canvas");
        canvas.set_fill_style("#cccccc").await.unwrap();
        canvas.fill_rect(30.0, 30.0, 30.0, 30.0).await.unwrap();
        info!(
            "data url: {}",
            canvas.to_data_url("image/png", None).await.unwrap()
        );
        drop(canvas);
        info!("drop canvas");
    }

    #[actix_rt::test]
    async fn test() -> Result<()> {
        Builder::from_default_env()
            .filter_level(LevelFilter::Debug)
            .init();

        let started_server = start_server("127.0.0.1:7070", "127.0.0.1:7071")?;
        loop {
            tokio::spawn(test_thread(started_server.clone()));
            delay_for(Duration::from_millis(5)).await;
        }
    }
}
