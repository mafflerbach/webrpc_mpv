use actix_web_actors::ws;
use actix::{Actor, StreamHandler};
use serde::{Serialize, Deserialize};
use actix_web::{HttpRequest,web, Error, HttpResponse, Result};
use crate::mpv;
use crate::api_structs::VolumeControl;
use crate::api_structs::PropertyComand;

use super::player::property_handle;

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

#[derive( Debug, Serialize, Deserialize)]
struct Message {
    body: serde_json::Value,
    message_type: String

}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
        ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => 
            {

                let ws_request : Message= serde_json::from_str(&text.as_str()).unwrap();
                match ws_request.message_type.as_ref() {

                    "set_volume" => 
                    {
                        let result : VolumeControl = serde_json::from_value(ws_request.body).unwrap();
                        let volume_response = mpv::mpv::event_volume_change(result);
                        let response = serde_json::to_string(&volume_response).unwrap();
                        ctx.text(response);
                    },
                    "get_volume" => 
                    {
                        let volume_response = mpv::mpv::event_volume();
                        let response = serde_json::to_string(&volume_response).unwrap();
                        ctx.text(response);
                    },
                    "property" => {
                        let result : PropertyComand = serde_json::from_value(ws_request.body).unwrap();

                        ctx.text(serde_json::to_string(&result).unwrap());
    println!("XXXXX {:?}", result);
                        let response = serde_json::to_string(&property_handle(result)).unwrap();
                        ctx.text(response);

                    },
                    

                    _ => {
                        ctx.text(format!("{{ \"error\": \"method not allowed\" }}"));
                    },

                }
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }

}

pub async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    resp
}
