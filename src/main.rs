use std::{
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
};

use futures::SinkExt;
use spotify_info::{SpotifyEvent, SpotifyListener, TrackState};
use warp::{
    ws::{Message, WebSocket},
    Filter,
};

#[derive(Debug)]

struct AppError {
    why: String,
}

impl AppError {
    fn new(why: String) -> Self {
        Self { why }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.why)
    }
}

impl std::error::Error for AppError {
    fn description(&self) -> &str {
        &self.why
    }
}

fn parse_json(path: &Path) -> Result<serde_json::Value, AppError> {
    let read = std::fs::read_to_string(path.to_str().unwrap());
    if let Ok(file_data) = read {
        if let Ok(suc) = serde_json::from_str(&file_data) {
            Ok(suc)
        } else {
            Err(AppError::new("unable to parse config".to_string()))
        }
    } else {
        Err(AppError::new("unable to read config".to_string()))
    }
}

fn get_field_from_cfg(cfg: &serde_json::Value, field: &str) -> Result<serde_json::Value, AppError> {
    if let Some(field_data) = cfg.get(&field) {
        Ok(field_data.clone())
    } else {
        Err(AppError::new(format!("unable to find field {}", field)))
    }
}

fn extract_value_str(val: &serde_json::Value) -> std::string::String {
    val.as_str().unwrap().to_string()
}

fn extract_value_u16(val: &serde_json::Value) -> u16 {
    val.as_str().unwrap().parse::<u16>().unwrap()
}

async fn ws_sendmessage(ws: &mut WebSocket, msg: String) -> Result<(), AppError> {
    let msg = Message::text(msg);
    if let Err(e) = ws.send(msg).await {
        return Err(AppError::new(format!("unable to send message: {}", e)));
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // setup filesystem dir n stuffs.
    let app_data = directories::BaseDirs::new().unwrap();
    match app_data.data_local_dir().try_exists() {
        Ok(_) => {
            let local_app_data = app_data.data_local_dir();
            let main = local_app_data.join(Path::new("spotify-np"));
            let themes = main.join(Path::new("themes"));
            let cfg = main.join(Path::new("config.json"));
            std::fs::create_dir_all(&themes).unwrap();

            if !cfg.exists() {
                std::fs::File::create(&cfg)
                    .unwrap()
                    .write_all(
                        serde_json::json!( {
                            "token": "",
                            "theme": "default",
                            "port_sv": "1273",
                            "port_ws": "1274"
                        })
                        .to_string()
                        .as_bytes(),
                    )
                    .unwrap();
            }

            let app_config = parse_json(&cfg).unwrap();
            let theme: String =
                extract_value_str(&get_field_from_cfg(&app_config, "theme").unwrap());

            let port_sv: u16 =
                extract_value_u16(&get_field_from_cfg(&app_config, "port_sv").unwrap());

            let port_ws: u16 =
                extract_value_u16(&get_field_from_cfg(&app_config, "port_ws").unwrap());

            let server = warp::get().and(warp::fs::dir(
                themes.join(Path::new(&theme))
            ));

            tokio::spawn(async move {
                warp::serve(server)
                    .run(SocketAddr::new(
                        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                        port_sv,
                    ))
                    .await;
            });

            let routes = warp::path::end()
                // The `ws()` filter will prepare the Websocket handshake.
                .and(warp::ws())
                .map(|ws: warp::ws::Ws| {
                    // And then our closure will be called when it completes...
                    ws.on_upgrade(|mut websocket| async move {
                        while let Ok(mut conn) = SpotifyListener::bind_default()
                            .await
                            .unwrap()
                            .get_connection()
                            .await
                        {
                            while let Some(Ok(event)) = conn.next().await {
                                match event {
                                    SpotifyEvent::TrackChanged(track) => {
                                        let websocket_send_result = ws_sendmessage(
                                            &mut websocket,
                                            serde_json::json!({
                                                "track": {
                                                    "name": track.title,
                                                    "artists": track.artist,
                                                    "length": track.duration,
                                                    "uri": track.uri,
                                                    "uid": track.uid,
                                                    "background": track.background_url,
                                                    "cover": track.cover_url,
                                                    "album": track.album,
                                                    "status": match track.state {
                                                        TrackState::Playing => {
                                                            "playing"
                                                        }
                                                        TrackState::Paused => {
                                                            "paused"
                                                        }
                                                        TrackState::Stopped => {
                                                            "stopped"
                                                        }
                                                    }
                                                }
                                            })
                                            .to_string(),
                                        );

                                        if let Err(err) = websocket_send_result.await {
                                            eprintln!(
                                                "Error sending message to WebSocket: {:?}",
                                                err
                                            );
                                        }
                                    }

                                    SpotifyEvent::ProgressChanged(progress) => {
                                        let websocket_send_result = ws_sendmessage(
                                            &mut websocket,
                                            serde_json::json!({
                                                "event": "progress",
                                                "progress": {
                                                    "position": progress,
                                                }
                                            })
                                            .to_string(),
                                        );

                                        if let Err(err) = websocket_send_result.await {
                                            eprintln!(
                                                "Error sending message to WebSocket: {:?}",
                                                err
                                            );
                                        }
                                    }

                                    _ => {} // We don't need any extra functionality outside of these two for right now.
                                }
                            }
                        }
                    })
                });

            warp::serve(routes).run(([127, 0, 0, 1], port_ws)).await;
        }
        Err(why) => {
            panic!("unable to access directory {:?} - {}", app_data, why);
        }
    }
}
