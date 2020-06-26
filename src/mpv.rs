

pub mod mpv{
    extern crate execute;
    use std::process::Command;
    use execute::Execute;
    use rocket::response::content;

    /// spawn a mpv process with ipc socket server and loads a playlist
    ///
    /// # Example 
    /// ```sh
    /// mpv --playlist=/tmp/playlist --input-ipc-server=/tmp/mpvsocket
    /// ```
    pub fn event_play_from_list(target: String) -> content::Json<String> {
        let param = format!("--playlist={}", target);
        let mut mpv = Command::new("mpv");
        mpv.arg(&target).
            arg("--input-ipc-server=/tmp/mpvsocket").
            arg(param);

        let output = mpv.execute_output().unwrap();

        return content::Json(String::from_utf8(output.stdout).unwrap());

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
    pub fn event_resume() -> content::Json<String> { 
        let mut command1 = get_echo_arg( "{ \"command\": [\"set_property\", \"pause\", false] }");
        let mut command2 = get_socket_arg();

        let output = command1.execute_multiple_output(&mut [&mut command2]).unwrap();
        return content::Json(String::from_utf8(output.stdout).unwrap());

    }

    /// Stops the current video and starts with the new source
    /// generates 
    /// ```sh
    /// echo "{ \"command\": [\"loadfile\", \"<target>\"] }}" | socat - /tmp/mpvsocket
    /// ```
    pub fn event_load(target: String) -> content::Json<String> {
        let json = format!("{{ \"command\": [\"loadfile\", \"{}\"] }}", target);
        let mut command1 = get_echo_arg(&json);
        let mut command2 = get_socket_arg();

        let output = command1.execute_multiple_output(&mut [&mut command2]).unwrap();
        return content::Json(String::from_utf8(output.stdout).unwrap());

    }
    /// Pause a video
    /// generates 
    /// ```sh
    /// echo "{ \"command\": [\"set_property\", \"pause\", true] }}" | socat - /tmp/mpvsocket
    /// ```
    pub fn event_pause() -> content::Json<String>{
        let mut command1 = get_echo_arg( "{ \"command\": [\"set_property\", \"pause\", true] }");
        let mut command2 = get_socket_arg();

        let output = command1.execute_multiple_output(&mut [&mut command2]).unwrap();
        return content::Json(String::from_utf8(output.stdout).unwrap());

    }


    fn get_echo_arg(json: &str) -> Command {
        let mut command = Command::new("echo");
        command.arg(json);

        return command;
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
            spawn().expect("OK");


    }
}
