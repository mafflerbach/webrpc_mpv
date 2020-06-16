

pub mod mpv{
    extern crate execute;
    use std::process::Command;
    use execute::Execute;
    use rocket::response::content;

pub fn event_play_from_list(target: String) -> content::Json<String> {
    // http://localhost:8000/load?target=/tmp/playlist
    let mut mpv = Command::new("mpv");
        let param = format!("--playlist={}", target);
    mpv.arg(&target).
        arg("--input-ipc-server=/tmp/mpvsocket").
        arg(param);

        let output = mpv.execute_output().unwrap();
 
        return content::Json(String::from_utf8(output.stdout).unwrap());

    }

pub fn event_load(target: String) -> content::Json<String> {
    // http://localhost:8000/load?target=/home/maren/Downloads/ytFiles/The Best Way To Practice Chords.webm
    // http://localhost:8000/load?target=%2Fhome%2Fmaren%2FDownloads%2FytFiles%2FThe%20Best%20Way%20To%20Practice%20Chords.webm
    // http://localhost:8000/load?target=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DP3UIpTlFtc4
    // 
    // will FAIL: http://localhost:8000/load?target=https://www.youtube.com/watch?v=P3UIpTlFtc4
    let mut mpv = Command::new("mpv");
    mpv.arg(&target).
        arg("--input-ipc-server=/tmp/mpvsocket").
        spawn().expect("OK");
        let mut command1 = Command::new("echo");
        let json = format!("{{ \"command\": [\"playlist\", \"{}\"] }}", target);

        command1.arg(json);

        let mut command2 = Command::new("socat");
        command2.arg("-").arg("/tmp/mpvsocket");

        let output = command1.execute_multiple_output(&mut [&mut command2]).unwrap();

       
        return content::Json(String::from_utf8(output.stdout).unwrap());

    }


pub fn event_play(target: String) -> &'static str {
    // http://localhost:8000/load?target=/home/maren/Downloads/ytFiles/The Best Way To Practice Chords.webm
    let mut mpv = Command::new("mpv");
    mpv.arg(target).
        arg("--input-ipc-server=/tmp/mpvsocket").
        spawn().expect("OK");
    "Hello, world!"
    }


pub fn event_resume() -> content::Json<String> { 
        let mut command1 = Command::new("echo");
        command1.arg("{ \"command\": [\"set_property\", \"pause\", false] }");

        let mut command2 = Command::new("socat");
        command2.arg("-").arg("/tmp/mpvsocket");

        let output = command1.execute_multiple_output(&mut [&mut command2]).unwrap();
        return content::Json(String::from_utf8(output.stdout).unwrap());

    }

    pub fn event_pause() -> content::Json<String>{
        let mut command1 = Command::new("echo");
        command1.arg("{ \"command\": [\"set_property\", \"pause\", true] }");

        let mut command2 = Command::new("socat");
        command2.arg("-").arg("/tmp/mpvsocket");

        let output = command1.execute_multiple_output(&mut [&mut command2]).unwrap();
        return content::Json(String::from_utf8(output.stdout).unwrap());

    }

}
