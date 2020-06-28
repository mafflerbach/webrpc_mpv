pub mod mpv {
    extern crate execute;

    use std::io::prelude::*;
    use std::os::unix::net::UnixStream;
    use std::process::Command;
    /// spawn a mpv process with ipc socket server and loads a playlist
    ///
    /// # Example
    /// ```sh
    /// mpv --playlist=/tmp/playlist --input-ipc-server=/tmp/mpvsocket
    /// ```
    pub fn event_play_from_list(target: String) -> std::io::Result<String> {
        let tjson = json!({ "command": ["loadlist", format!("{}",target)] });
        write_to_socket(tjson.to_string() + "\n")
    }

    /// starts a player with a target to play
    /// generates
    /// ```sh
    /// mpv <target> --input-ipc-server=/tmp/mpvsocket
    /// ```
    pub fn event_play(target: String) -> &'static str {
        let mut mpv = Command::new("mpv");
        mpv.arg(target)
            .arg("--input-ipc-server=/tmp/mpvsocket")
            .spawn()
            .expect("OK");
        "Hello, world!"
    }

    /// resume a running instance
    /// generates
    /// ```sh
    /// echo "{ \"command\": [\"set_property\", \"pause\", false] }}" | socat - /tmp/mpvsocket
    /// ```

    pub fn event_resume() -> std::io::Result<String> {
        let tjson = json!({ "command": ["set_property", "pause", false] });
        write_to_socket(tjson.to_string() + "\n")
    }

    /// Stops the current video and starts with the new source
    /// generates
    /// ```sh
    /// echo "{ \"command\": [\"loadfile\", \"<target>\"] }}" | socat - /tmp/mpvsocket
    /// ```
    pub fn event_load(target: String) -> std::io::Result<String> {
        let tjson = json!({ "command": ["loadfile", format!("{}",target)] });
        write_to_socket(tjson.to_string() + "\n")
    }
    /// Pause a video
    /// generates
    /// ```sh
    /// echo "{ \"command\": [\"set_property\", \"pause\", true] }}" | socat - /tmp/mpvsocket
    /// ```
    pub fn event_pause() -> std::io::Result<String> {
        let tjson = json!({ "command": ["set_property", "pause", true] });
        write_to_socket(tjson.to_string() + "\n")
    }

    pub fn event_volume() -> std::io::Result<String> {
        let tjson = json!({ "command": ["get_property", "volume"] });
        write_to_socket(tjson.to_string() + "\n")
    }

    /// Init mpv in idle mode
    pub fn init() {
        let mut mpv = Command::new("mpv");
        mpv.arg("--idle=yes")
            .arg("--input-ipc-server=/tmp/mpvsocket")
            .arg("--fs=yes")
            .spawn()
            .expect("OK");
    }

    #[derive(Serialize, Deserialize)]
    struct VolumResponse {
        data: String,
        error: String,
        request_id: i32,
    }

    fn write_to_socket(content: String) -> std::io::Result<String> {
        let mut stream = match UnixStream::connect("/tmp/mpvsocket") {
            Err(_) => panic!("could not connect to socket"),
            Ok(stream) => stream,
        };

        stream.write_all(&content.as_bytes())?;
        let mut buf = [0; 1024];
        let count = stream.read(&mut buf).unwrap();
        let response = String::from_utf8(buf[..count].to_vec()).unwrap();

        Ok(response)
    }
}
