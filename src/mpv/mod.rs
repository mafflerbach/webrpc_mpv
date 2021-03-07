pub mod mpv {

    extern crate execute;
    use crate::api_structs::VolumeControl;
    use crate::settings;
    use std::io::prelude::*;
    use std::os::unix::net::UnixStream;
    use std::process::Command;
    use serde_json::json;
    use serde::{Serialize, Deserialize};

    fn send_command(command: serde_json::Value) -> serde_json::Value {
        let id = rand::random::<u32>();
        let json = json!({"command": command, "request_id": id});
        let bytes = &(json.to_string() + "\n");

        for line in write_to_socket(bytes).unwrap().trim().split("\n") {
            let object : serde_json::Value = serde_json::from_str(line).unwrap();
            if object["request_id"] == id {
                return object;
            }
        }

        return serde_json::from_str("{}").unwrap();
    }

    pub fn event_resume() -> Property {
        let command = json!(["set_property", "pause", false]);

        let result = send_command(command);
        let me = Property {
            error : result["error"].to_string().replace("\"", ""),
            data : result["data"].to_string()
        };

        return me;
    }

    pub fn event_load(target: &str, mode: &str) -> Property {
        let command = json!(["loadfile", target, mode]);
        let result = send_command(command);
        let me = Property {
            error : String::from("success"),
            data : result["event"].to_string()
        };
        event_resume();
        return me;
    }

    pub fn event_pause() -> Property {
        let command = json!(["set_property", "pause", true]);
        let result = send_command(command);

        let me = Property {
            error : result["error"].to_string().replace("\"", ""),
            data : result["data"].to_string()
        };

        return me;
    }

    fn update_video_status() {
        use mpv_webrpc::models::NewVideoStatus;
        let path = event_property("path", None);

        if path.error == String::from("success") {
            let time_json:String = event_property("time-pos", None).data;
            let path_json:String = event_property("path", None).data;

            // serde json supports only f64 - diesel supports only f32 for fields - *sigh*
            let time : f64= time_json.parse().unwrap();
            let convert = time as f32;
            let video_status = NewVideoStatus {
                path: &path_json.replace("\"", ""),
                time: &convert,
            };
            video_status.upsert();
        }
    }

    pub fn event_stop() -> Property {
        update_video_status();
        // Show the next playlist item (the backdrop image) instead of stopping
        let command = json!(["playlist-next"]);
        let result = send_command(command);
        let me = Property {
            error : String::from("success"),
            data : result["event"].to_string()
        };
        return me;
    }

    pub fn event_volume() -> Property {
        event_property("volume", None)
    }

    pub fn event_volume_change(volume_control: VolumeControl) -> Property {
        event_property("volume", Some(volume_control.value.to_string()))
    }

    pub fn event_property(property: &str, value: Option<String>) -> Property {
        let command = match value {
            None => {
                json!(["get_property", property])
            }, 
            Some(value) => {
                json!(["set_property", property, value])
            },
        };
        let result = send_command(command);

        let me = Property {
            error : result["error"].to_string().replace("\"", ""),
            data : result["data"].to_string()
        };

        return me;
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Property {
        pub error : String,
        pub data : String
    }

    pub fn init() {
        let settings = settings::init();
        let title = std::env::var("TITLE").unwrap_or("Media Mate Player".to_string());

        let mut mpv = Command::new("mpv");
        let ipc_param = format!("--input-ipc-server={}", settings.socket);
        println!("Starting parameter for mpv: {}", ipc_param);
        mpv.arg("--idle=yes")
            .arg("--title=".to_owned() + &title)
            .arg(ipc_param)
            .arg("--hwdec=mmal-copy")
            .arg("--fullscreen")
            .arg("--vo=gpu")
            .arg("--keep-open")
            .arg("--image-display-duration=inf")
            .spawn()
            .expect("OK");

        loop {
            match UnixStream::connect(&settings.socket) {
                Ok(_) => break,
                Err(_) => {},
            };
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    pub fn write_to_socket(content: &str) -> std::io::Result<String> {
        let settings = settings::init();
        let socket = settings.socket;
        let mut stream = match UnixStream::connect(&socket) {
            Err(e) => panic!("could not connect to socket {} - {}", e, &socket),
            Ok(stream) => stream,
        };

        stream.write_all(content.as_bytes())?;
        let mut buf = [0; 1024];
        let count = stream.read(&mut buf).unwrap();
        let response = String::from_utf8(buf[..count].to_vec()).unwrap();

        Ok(response)
    }
}
