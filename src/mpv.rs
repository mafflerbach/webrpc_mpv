

pub mod mpv{
    extern crate execute;
    use std::process::Command;
    use execute::Execute;
    use rocket::response::content;

    use rocket_contrib::json::{ JsonValue};
    use std::os::unix::net::UnixStream;
    use std::io::prelude::*;
    /// spawn a mpv process with ipc socket server and loads a playlist
    ///
    /// # Example 
    /// ```sh
    /// mpv --playlist=/tmp/playlist --input-ipc-server=/tmp/mpvsocket
    /// ```
    pub fn event_play_from_list(target: String) -> JsonValue {

    let tjson = json!({ "command": ["loadlist", format!("{}",target)] }); 
        write_to_socket(tjson.to_string()+"\n");

        return   json!({
            "status": "ok",
            "reason": "play from url "
        });

    }

    /// starts a player with a target to play
    /// generates 
    /// ```sh
    /// mpv <target> --input-ipc-server=/tmp/mpvsocket
    /// ```
    pub fn event_play(target: String) -> &'static str {
        let mut mpv = Command::new("mpv");
        mpv.arg(target).
            arg("--input-ipc-server=/tmp/mpvsocket").
            spawn().expect("OK");
        "Hello, world!"
    }

    /// resume a running instance
    /// generates 
    /// ```sh
    /// echo "{ \"command\": [\"set_property\", \"pause\", false] }}" | socat - /tmp/mpvsocket
    /// ```

    pub fn event_resume() -> JsonValue { 
        let tjson = json!({ "command": ["set_property", "pause", false] }); 
        write_to_socket(tjson.to_string()+"\n");

        return   json!({
            "status": "ok",
            "reason": "play from url "
        });


    }

    /// Stops the current video and starts with the new source
    /// generates 
    /// ```sh
    /// echo "{ \"command\": [\"loadfile\", \"<target>\"] }}" | socat - /tmp/mpvsocket
    /// ```
    pub fn event_load(target: String) -> JsonValue {
        let tjson = json!({ "command": ["loadfile", format!("{}",target)] }); 
        write_to_socket(tjson.to_string()+"\n");

        return   json!({
            "status": "ok",
            "reason": "play from url "
        });

    }
    /// Pause a video
    /// generates 
    /// ```sh
    /// echo "{ \"command\": [\"set_property\", \"pause\", true] }}" | socat - /tmp/mpvsocket
    /// ```
    pub fn event_pause() -> JsonValue {

        let tjson = json!({ "command": ["set_property", "pause", true] }); 
        write_to_socket(tjson.to_string()+"\n");

        return   json!({
            "status": "ok",
            "reason": "play from url "
        });
    }


    
    fn get_socket_arg() -> Command {
        let mut command = Command::new("socat");
        command.arg("-").arg("/tmp/mpvsocket");

        return command;
    }


    /// Init mpv in idle mode
    pub fn init() {
        let mut mpv = Command::new("mpv");
        mpv.arg("--idle=yes").
            arg("--input-ipc-server=/tmp/mpvsocket").
            arg("--fs=yes").
            spawn().expect("OK");


    }




    fn write_to_socket(content: String) -> std::io::Result<()> {

        let mut stream = UnixStream::connect("/tmp/mpvsocket")?;
        stream.write_all(&content.as_bytes());
        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        println!("{}", response);
        Ok(())
    }


}
