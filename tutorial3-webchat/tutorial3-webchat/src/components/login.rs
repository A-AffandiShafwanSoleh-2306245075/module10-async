use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div style="
            min-height: 100vh;
            width: 100vw;
            background: linear-gradient(135deg, #a8d8f0 0%, #c8eaff 30%, #e8f7ff 60%, #b8e4f9 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            font-family: 'Segoe UI', sans-serif;
            position: relative;
            overflow: hidden;
        ">
            <div style="
                position: absolute; top: -80px; left: -80px;
                width: 300px; height: 300px; border-radius: 50%;
                background: radial-gradient(circle, rgba(255,255,255,0.6) 0%, rgba(135,206,250,0.3) 70%);
                filter: blur(2px);
            "/>
            <div style="
                position: absolute; bottom: -100px; right: -60px;
                width: 400px; height: 400px; border-radius: 50%;
                background: radial-gradient(circle, rgba(255,255,255,0.5) 0%, rgba(100,180,240,0.2) 70%);
                filter: blur(3px);
            "/>
            <div style="
                position: absolute; top: 40%; left: 10%;
                width: 150px; height: 150px; border-radius: 50%;
                background: radial-gradient(circle, rgba(255,255,255,0.7) 0%, rgba(180,230,255,0.2) 70%);
            "/>

            // Main card
            <div style="
                background: linear-gradient(160deg, rgba(255,255,255,0.75) 0%, rgba(200,235,255,0.55) 100%);
                backdrop-filter: blur(20px);
                -webkit-backdrop-filter: blur(20px);
                border: 1.5px solid rgba(255,255,255,0.8);
                border-radius: 24px;
                padding: 48px 40px;
                box-shadow: 0 8px 32px rgba(80,160,220,0.18), 0 1.5px 8px rgba(255,255,255,0.5) inset;
                display: flex;
                flex-direction: column;
                align-items: center;
                min-width: 360px;
                z-index: 1;
            ">
                // Logo orb
                <div style="
                    width: 80px; height: 80px; border-radius: 50%;
                    background: radial-gradient(circle at 35% 35%, #ffffff 0%, #7ecef4 40%, #2a9fd6 100%);
                    box-shadow: 0 4px 24px rgba(42,159,214,0.4), 0 1px 4px rgba(255,255,255,0.8) inset;
                    margin-bottom: 20px;
                    display: flex; align-items: center; justify-content: center;
                    font-size: 36px;
                ">
                    {"💬"}
                </div>

                <h1 style="
                    color: #1a6fa8;
                    font-size: 28px;
                    font-weight: 700;
                    margin: 0 0 6px 0;
                    text-shadow: 0 1px 2px rgba(255,255,255,0.8);
                    letter-spacing: -0.5px;
                ">{"YewChat!"}</h1>

                <p style="
                    color: #5ba3cc;
                    font-size: 13px;
                    margin: 0 0 28px 0;
                    text-align: center;
                ">{"✨ Selamat datang! Masukkan username untuk mulai"}</p>

                <div style="display: flex; width: 100%; gap: 0;">
                    <input
                        oninput={oninput}
                        placeholder="🌊 Username kamu..."
                        style="
                            flex: 1;
                            padding: 12px 18px;
                            border-radius: 50px 0 0 50px;
                            border: 1.5px solid rgba(100,180,240,0.5);
                            border-right: none;
                            background: rgba(255,255,255,0.7);
                            color: #1a5f8a;
                            font-size: 14px;
                            outline: none;
                            font-family: 'Segoe UI', sans-serif;
                        "
                    />
                    <Link<Route> to={Route::Chat}>
                        <button
                            onclick={onclick}
                            disabled={username.len() < 1}
                            style="
                                padding: 12px 22px;
                                border-radius: 0 50px 50px 0;
                                border: 1.5px solid rgba(42,159,214,0.6);
                                background: linear-gradient(180deg, #5bc8f5 0%, #2a9fd6 50%, #1a7ab8 100%);
                                color: white;
                                font-weight: 700;
                                font-size: 13px;
                                cursor: pointer;
                                letter-spacing: 0.5px;
                                box-shadow: 0 2px 8px rgba(42,159,214,0.4), 0 1px 2px rgba(255,255,255,0.5) inset;
                                font-family: 'Segoe UI', sans-serif;
                            "
                        >{"GO! 🚀"}</button>
                    </Link<Route>>
                </div>

                <p style="
                    color: #88c0dc;
                    font-size: 11px;
                    margin: 20px 0 0 0;
                ">{"🌐 YewChat!"}</p>
            </div>
        </div>
    }
}