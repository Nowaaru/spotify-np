use std::{
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
};

use owo_colors::OwoColorize;
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

fn extract_value_bool(val: &serde_json::Value) -> bool {
    val.as_bool().unwrap()
}

fn extract_value_u16(val: &serde_json::Value) -> u16 {
    val.as_str().unwrap().parse::<u16>().unwrap()
}

async fn ws_sendmessage(ws: &mut WebSocket, msg: String) -> Result<(), AppError> {
    let msg = Message::text(msg);
    if let Err(e) = ws.send(msg).await {
        return Err(AppError::new(format!("unable to send message - {}", e)));
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // setup filesystem dir n stuffs.
    println!("{}{}", "spotify-np v".cyan().bold(), env!("CARGO_PKG_VERSION").cyan().bold());

    let base = Path::new("./");
    let themes = base.join(Path::new("themes"));
    let cfg = base.join(Path::new("config.json"));

    let cfg_defaults = serde_json::json!( {
        "theme": "",
        "port_sv": "1273",
        "port_ws": "1274",
        "errors_ws": false,
        "no_ws": false,
    });

    std::fs::create_dir_all(&themes).unwrap();

    if !cfg.exists() {
        std::fs::File::create(&cfg)
            .unwrap()
            .write_all(
                serde_json::to_string_pretty(&cfg_defaults.clone())
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
    }

    let app_config = parse_json(&cfg).unwrap();
    let theme: String = extract_value_str(
        &get_field_from_cfg(&app_config, "theme")
            .unwrap_or_else(|_| get_field_from_cfg(&cfg_defaults, "theme").unwrap()),
    );

    let port_sv: u16 = extract_value_u16(
        &get_field_from_cfg(&app_config, "port_sv")
            .unwrap_or_else(|_| get_field_from_cfg(&cfg_defaults, "port_sv").unwrap()),
    );

    let port_ws: u16 = extract_value_u16(&get_field_from_cfg(&app_config, "port_ws").unwrap());

    let errors_ws: bool = extract_value_bool(
        &get_field_from_cfg(&app_config, "errors_ws")
            .unwrap_or_else(|_| get_field_from_cfg(&cfg_defaults, "errors_ws").unwrap()),
    );

    let no_ws: bool = extract_value_bool(
        &get_field_from_cfg(&app_config, "no_ws")
            .unwrap_or_else(|_| get_field_from_cfg(&cfg_defaults, "no_ws").unwrap()),
    );

    // Warn if a dangerous field is used.
    if !errors_ws {
        eprintln!("{} errors_ws is set to false, this is dangerous! if anything goes wrong, please be sure to {} before making an issue/pr!", "!".red(), "turn this field back to true".yellow());
    }

    if no_ws {
        eprintln!("{} no_ws is set to true. not starting the webserver this run.", "!".red());
    }

    println!(); // newline

    // Check if the theme they're looking for exists. If not, throw an error.
    let theme_path = themes.join(Path::new(&theme));
    if !theme_path.exists() || !theme_path.is_dir() || theme.is_empty() {
        panic!("theme \"{}\" not found", theme);
    }

    let server = warp::get()
        .and(warp::fs::dir(themes.join(Path::new(&theme))))
        .or(warp::path("config").and(warp::fs::file(cfg)));

    let (tx, _rx) = tokio::sync::broadcast::channel::<String>(24);
    let tx2 = tx.clone();
    tokio::spawn(async move {
        println!("Starting {} server...", "main".cyan().bold());
        warp::serve(server)
            .run(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                port_sv,
            ))
            .await;
    });

    tokio::spawn(async move {
        while let Ok(mut conn) = SpotifyListener::bind_default()
            .await
            .unwrap()
            .get_connection()
            .await
        {
            while let Some(Ok(event)) = conn.next().await {
                match event {
                    SpotifyEvent::TrackChanged(track) => {
                        let json = serde_json::json!({
                            "event": "track",
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
                        .to_string();

                        if let Err(err) = tx.send(json) {
                            eprintln!("An error occured when sending a message to the websocket server: {}", err);
                        }
                    }

                    SpotifyEvent::StateChanged(state) => {
                        let json = serde_json::json!({
                            "event": "state",
                            "state": match state {
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
                        })
                        .to_string();

                        if let Err(err) = tx.send(json) {
                            eprintln!("An error occured when sending a message to the websocket server: {}", err);
                        }
                    }

                    SpotifyEvent::ProgressChanged(progress) => {
                        let json = serde_json::json!({
                            "event": "progress",
                            "progress": {
                                "position": progress,
                            }
                        })
                        .to_string();

                        if let Err(err) = tx.send(json) {
                            eprintln!("An error occured when sending a message to the websocket server: {}", err);
                        }
                    }
                }
            }
        }
    });

    if !no_ws {
        let routes = warp::path::end()
            .and(warp::ws())
            .map(move |ws: warp::ws::Ws| {
                let mut rx = tx2.subscribe();
                ws.on_upgrade(move |mut websocket| async move {
                    while let Ok(v) = rx.recv().await {
                        if let Err(e) = ws_sendmessage(&mut websocket, v).await {
                            if errors_ws {
                                eprintln!("{}", e);
                            }
                            break;
                        }
                    }
                })
            });

        println!("Starting {} server...", "websocket".cyan().bold());
        warp::serve(routes).run(([127, 0, 0, 1], port_ws)).await;
    }
}
