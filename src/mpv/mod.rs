pub mod mpv {

    extern crate execute;
    use crate::api_structs::VolumeControl;
    use crate::settings;
    use rocket_contrib::json::Json;
    use std::io::prelude::*;
    use std::os::unix::net::UnixStream;
    use std::process::Command;

    pub fn event_play_from_list(target: String) -> std::io::Result<String> {
        let tjson = json!({ "command": ["loadlist", format!("{}",target)] });
        write_to_socket(tjson.to_string() + "\n")
    }

    pub fn event_play(target: String) -> &'static str {
        let mut mpv = Command::new("mpv");
        let settings = settings::init();
        let ipc_param = format!("--input-ipc-server={}", settings.socket);
        mpv.arg(target).arg(ipc_param).spawn().expect("OK");
        "Hello, world!"
    }

    pub fn event_resume() -> std::io::Result<String> {
        let tjson = json!({ "command": ["set_property", "pause", false] });
        write_to_socket(tjson.to_string() + "\n")
    }

    pub fn event_load(target: String) -> std::io::Result<String> {
        let tjson = json!({ "command": ["loadfile", format!("{}",target)] });
        write_to_socket(tjson.to_string() + "\n")
    }
    pub fn event_pause() -> std::io::Result<String> {
        let tjson = json!({ "command": ["set_property", "pause", true] });
        write_to_socket(tjson.to_string() + "\n")
    }
    pub fn event_quit() -> std::io::Result<String> {
        let tjson = json!({ "command": ["quit"] });
        write_to_socket(tjson.to_string() + "\n")
    }

    fn update_video_status() {
        use mpv_webrpc::models::NewVideoStatus;
        let path = event_get_property("path".to_string());

        if path.is_ok() {
            let time_json: Value =
                serde_json::from_str(event_get_property("time-pos".to_string()).unwrap().as_str())
                    .expect("cannot read propery");

            let path_json: Value =
                serde_json::from_str(path.unwrap().as_str()).expect("cannot handle path");

            // serde json supports only f64 - diesel supports only f32 for fields - *sigh*
            let time = time_json.get("data").unwrap().as_f64().unwrap();
            let convert = time as f32;
            if !path_json.get("data").is_none() {
                let video_status = NewVideoStatus {
                    path: &path_json.get("data").unwrap().to_string().replace("\"", ""),
                    time: &convert,
                };
                video_status.upsert();
            }
        }
    }
    use serde_json::Value;
    pub fn event_stop() -> std::io::Result<String> {
        update_video_status();
        let tjson = json!({ "command": ["stop"] });
        write_to_socket(tjson.to_string() + "\n")
    }
    pub fn event_volume() -> std::io::Result<String> {
        let tjson = json!({ "command": ["get_property", "volume"] });
        write_to_socket(tjson.to_string() + "\n")
    }

    pub fn event_volume_change(volume_control: Json<VolumeControl>) -> std::io::Result<String> {
        let tjson = json!({ "command": ["set_property", "volume", volume_control.value] });
        write_to_socket(tjson.to_string() + "\n")
    }

    pub fn event_set_property(propery: String, value: String) -> std::io::Result<String> {
        let tjson = json!({ "command": ["set_property", propery, value] });
        write_to_socket(tjson.to_string() + "\n")
    }

    pub fn event_get_property(propery: String) -> std::io::Result<String> {
        let tjson = json!({ "command": ["get_property", propery] });
        write_to_socket(tjson.to_string() + "\n")
    }

    pub fn init() {
        let settings = settings::init();

        let mut mpv = Command::new("mpv");
        let ipc_param = format!("--input-ipc-server={}", settings.socket);
        println!("Starting parameter for mpv: {}", ipc_param);
        mpv.arg("--idle=yes")
            .arg(ipc_param)
            .arg("--hwdec=mmal-copy")
            .arg("--fs=yes")
            .arg("--vo=gpu")
            .spawn()
            .expect("OK");
    }

    pub fn write_to_socket(content: String) -> std::io::Result<String> {
        let settings = settings::init();
        let socket = settings.socket;
        let mut stream = match UnixStream::connect(&socket) {
            Err(e) => panic!("could not connect to socket {} - {}", e, &socket),
            Ok(stream) => stream,
        };

        stream.write_all(&content.as_bytes())?;
        let mut buf = [0; 1024];
        let count = stream.read(&mut buf).unwrap();
        let response = String::from_utf8(buf[..count].to_vec()).unwrap();

        Ok(response)
    }
}
