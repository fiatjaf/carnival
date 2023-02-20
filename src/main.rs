use async_tungstenite::{gio::connect_async, tungstenite::Message as WebSocketMessage};
use futures::prelude::*;
use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
struct Model {}

#[derive(Clone, Debug)]
enum Msg {
    Exit,
    DoStuff,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            Msg::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
            Msg::DoStuff => {
                println!("doing async stuff");
                UpdateAction::defer(async {
                    let res = ws().await;
                    println!("res: {:?}", res);
                    Msg::Exit
                })
            }
        }
    }

    fn view(&self) -> VNode<Model> {
        gtk! {
          <Application::new_unwrap(Some("com.fiatjaf.carnival"), ApplicationFlags::empty())>
            <Window border_width=20 on destroy=|_| Msg::Exit>
              <Label label="carnival" />
              <Button label="do stuff" on clicked=|_| Msg::DoStuff />
            </Window>
          </Application>
        }
    }
}

async fn ws() -> Option<()> {
    let url = "wss://relay.nostr.bg";
    let (mut ws_stream, _) = connect_async(url).await.ok()?;
    let message = "[\"REQ\", \"_\", {\"authors\": [\"3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d\"], \"limit\": 1}]";

    ws_stream.send(WebSocketMessage::text(message)).await.ok()?;

    let msg = ws_stream
        .next()
        .await
        .ok_or("didn't receive anything")
        .ok()?;

    println!("Received: {:?}", msg);
    Some(())
}

fn main() {
    pretty_env_logger::init();
    std::process::exit(run::<Model>());
}
