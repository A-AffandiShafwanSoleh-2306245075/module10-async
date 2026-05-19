use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div style="
                display: flex;
                width: 100vw;
                height: 100vh;
                background: linear-gradient(135deg, #a8d8f0 0%, #c8eaff 40%, #e0f4ff 70%, #b8e4f9 100%);
                font-family: 'Segoe UI', sans-serif;
                overflow: hidden;
            ">
                // Sidebar
                <div style="
                    width: 220px;
                    flex-shrink: 0;
                    height: 100vh;
                    background: linear-gradient(180deg, rgba(255,255,255,0.65) 0%, rgba(180,225,255,0.5) 100%);
                    backdrop-filter: blur(16px);
                    border-right: 1.5px solid rgba(255,255,255,0.7);
                    box-shadow: 2px 0 16px rgba(80,160,220,0.1);
                    display: flex;
                    flex-direction: column;
                ">
                    // Sidebar header
                    <div style="
                        padding: 20px 16px 12px;
                        border-bottom: 1px solid rgba(100,180,240,0.2);
                    ">
                        <div style="
                            font-size: 16px;
                            font-weight: 700;
                            color: #1a6fa8;
                            text-shadow: 0 1px 2px rgba(255,255,255,0.8);
                        ">{"👥 Online Users"}</div>
                    </div>

                    // User list
                    <div style="flex: 1; overflow-y: auto; padding: 8px;">
                        {
                            self.users.clone().iter().map(|u| {
                                html!{
                                    <div style="
                                        display: flex;
                                        align-items: center;
                                        margin: 6px 0;
                                        padding: 10px;
                                        background: linear-gradient(135deg, rgba(255,255,255,0.8) 0%, rgba(200,235,255,0.6) 100%);
                                        border-radius: 14px;
                                        border: 1px solid rgba(255,255,255,0.9);
                                        box-shadow: 0 2px 8px rgba(80,160,220,0.1);
                                    ">
                                        <img
                                            style="width: 38px; height: 38px; border-radius: 50%; border: 2px solid rgba(100,180,240,0.5); box-shadow: 0 2px 6px rgba(42,159,214,0.2);"
                                            src={u.avatar.clone()}
                                            alt="avatar"
                                        />
                                        <div style="margin-left: 10px;">
                                            <div style="font-size: 13px; font-weight: 600; color: #1a5f8a;">{u.name.clone()}</div>
                                            <div style="font-size: 11px; color: #5ba3cc;">{"🟢 Online"}</div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>

                // Main chat area
                <div style="
                    flex: 1;
                    display: flex;
                    flex-direction: column;
                    height: 100vh;
                    overflow: hidden;
                ">
                    // Header
                    <div style="
                        height: 56px;
                        flex-shrink: 0;
                        background: linear-gradient(90deg, rgba(255,255,255,0.7) 0%, rgba(200,235,255,0.5) 100%);
                        backdrop-filter: blur(12px);
                        border-bottom: 1.5px solid rgba(255,255,255,0.7);
                        display: flex;
                        align-items: center;
                        padding: 0 20px;
                        box-shadow: 0 2px 12px rgba(80,160,220,0.1);
                    ">
                        <div style="font-size: 18px; font-weight: 700; color: #1a6fa8; text-shadow: 0 1px 2px rgba(255,255,255,0.8);">
                            {"💬 YewChat!"}
                        </div>
                        <div style="margin-left: 12px; font-size: 12px; color: #5ba3cc;">
                            {"✨ YewChat!"}
                        </div>
                    </div>

                    // Messages area
                    <div style="
                        flex: 1;
                        overflow-y: auto;
                        padding: 16px 20px;
                    ">
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                html!{
                                    <div style="
                                        display: flex;
                                        align-items: flex-end;
                                        margin-bottom: 16px;
                                        max-width: 60%;
                                    ">
                                        <img
                                            style="width: 34px; height: 34px; border-radius: 50%; border: 2px solid rgba(255,255,255,0.9); box-shadow: 0 2px 6px rgba(42,159,214,0.25); margin-right: 10px; flex-shrink: 0;"
                                            src={user.avatar.clone()}
                                            alt="avatar"
                                        />
                                        <div style="
                                            background: linear-gradient(135deg, rgba(255,255,255,0.85) 0%, rgba(210,240,255,0.7) 100%);
                                            border: 1px solid rgba(255,255,255,0.9);
                                            border-radius: 4px 16px 16px 16px;
                                            padding: 10px 14px;
                                            box-shadow: 0 2px 12px rgba(80,160,220,0.12);
                                            backdrop-filter: blur(8px);
                                        ">
                                            <div style="font-size: 12px; font-weight: 600; color: #1a6fa8; margin-bottom: 4px;">
                                                {m.from.clone()}
                                            </div>
                                            <div style="font-size: 13px; color: #2d7aa8;">
                                                if m.message.ends_with(".gif") {
                                                    <img style="max-width: 200px; border-radius: 8px;" src={m.message.clone()}/>
                                                } else {
                                                    {m.message.clone()}
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>

                    // Input area
                    <div style="
                        flex-shrink: 0;
                        padding: 12px 20px;
                        background: linear-gradient(90deg, rgba(255,255,255,0.65) 0%, rgba(200,235,255,0.5) 100%);
                        backdrop-filter: blur(12px);
                        border-top: 1.5px solid rgba(255,255,255,0.7);
                        display: flex;
                        align-items: center;
                        gap: 10px;
                    ">
                        <input
                            ref={self.chat_input.clone()}
                            type="text"
                            placeholder="🌊 Ketik pesan..."
                            style="
                                flex: 1;
                                padding: 11px 20px;
                                border-radius: 50px;
                                border: 1.5px solid rgba(100,180,240,0.4);
                                background: rgba(255,255,255,0.7);
                                color: #1a5f8a;
                                font-size: 14px;
                                outline: none;
                                box-shadow: 0 2px 8px rgba(80,160,220,0.08) inset;
                                font-family: 'Segoe UI', sans-serif;
                            "
                            name="message"
                            required=true
                        />
                        <button
                            onclick={submit}
                            style="
                                width: 42px; height: 42px;
                                border-radius: 50%;
                                border: 1.5px solid rgba(42,159,214,0.5);
                                background: radial-gradient(circle at 35% 35%, #7ed4f7 0%, #2a9fd6 60%, #1a7ab8 100%);
                                box-shadow: 0 3px 12px rgba(42,159,214,0.4), 0 1px 3px rgba(255,255,255,0.6) inset;
                                cursor: pointer;
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                flex-shrink: 0;
                            "
                        >
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" style="width: 18px; height: 18px; fill: white;">
                                <path d="M0 0h24v24H0z" fill="none"></path>
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}