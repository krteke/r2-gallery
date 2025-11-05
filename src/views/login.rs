use crate::utils::get_browser_origin;
use crate::{
    components::{Dialog, InlineSvg},
    models::LoginInfo,
    server::Route,
};
use dioxus::prelude::*;
use futures::StreamExt;

const EYE_SHOW_ICON: &str = include_str!("../../assets/svg/eye-show.svg");
const EYE_OFF_ICON: &str = include_str!("../../assets/svg/eye-off.svg");

#[component]
pub fn Login() -> Element {
    let navigator = use_navigator();
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut is_password_visible = use_signal(|| false);
    let mut is_username_empty = use_signal(|| true);
    let mut is_password_empty = use_signal(|| true);
    let mut error_message = use_signal(|| String::new());
    let mut is_open = use_signal(|| false);
    let mut is_verifying = use_signal(|| false);

    let input_class =
        "w-full py-3.5 pl-4 pr-4 rounded-lg border border-transparent bg-[#333333] outline-0
    text-white text-[16px] focus:border-[#3498db] focus:shadow-[0_0_6px_rgba(52,152,219,0.6)]
    transition-[border-color_box-shadow] duration-300 ease-in-out";

    let eye_class =
        "absolute w-5 h-5 text-white select-none transition-opacity duration-200 ease-in-out";

    let on_login_success = use_callback(move |_| {
        tracing::info!("Login successful");
        navigator.push(Route::Home {});
    });

    let login_coroutine = use_coroutine(move |mut rx: UnboundedReceiver<LoginInfo>| {
        let on_login_success = on_login_success.clone();

        async move {
            let client = reqwest::Client::new();
            let base_url = get_browser_origin().unwrap_or_else(|| "".to_string());
            tracing::debug!("{}", base_url);

            while let Some(action) = rx.next().await {
                is_verifying.set(true);

                let request_url = format!("{}/api/login", base_url);
                let response = client.post(&request_url).json(&action).send().await;
                tracing::debug!("response: {:?}", response);

                match response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            on_login_success.call(());
                        } else {
                            let status = resp.status();
                            let text = resp.text().await.unwrap_or_default();

                            let error_text = match status.as_u16() {
                                401 | 403 => "用户名或密码错误".to_string(),
                                _ => format!("登录失败! 状态码: {}, 信息: {}", status, text),
                            };

                            error_message.set(error_text);
                            is_open.set(true);
                        }
                    }
                    Err(err) => {
                        error_message.set(format!("请求失败: {}", err));
                        is_open.set(true);
                    }
                }

                is_verifying.set(false);
            }
        }
    });

    rsx! {
        Dialog {
            open: is_open,
            title: "登录失败".to_string(),
            description: error_message.clone(),
        }
        div { class: "justify-center items-center flex min-w-dvh min-h-dvh",
            div { class: "bg-[rgba(35,35,45,0.8)] backdrop-blur-xl border border-solid border-[rgba(255,255,255,0.15)]
                p-9 rounded-3xl shadow-lg max-w-96 w-9/10 flex flex-col gap-6 min-h-[512px]",
                h2 { class: "text-center select-none text-3xl font-bold", "登录" }
                hr { class: "m-0 border border-[rgba(255,255,255,0.15)]" }
                form {
                    class: "contents",
                    onsubmit: move |event| {
                        event.prevent_default();
                    },
                    label { r#for: "username", class: "font-bold", "用户名" }
                    input {
                        class: "{input_class}",
                        id: "username",
                        autocomplete: "username",
                        placeholder: "请输入用户名",
                        value: "{username}",
                        oninput: move |event| {
                            let value = event.value();
                            is_username_empty.set(value.trim().is_empty());
                            username.set(value);
                        },
                    }
                    label { r#for: "password", class: "font-bold", "密码" }
                    div { class: "relative flex items-center",
                        input {
                            class: "{input_class} pr-12.5",
                            id: "password",
                            placeholder: "请输入密码",
                            r#type: if is_password_visible() { "text" } else { "password" },
                            autocomplete: "current-password",
                            value: "{password}",
                            oninput: move |event| {
                                let value = event.data.value();
                                is_password_empty.set(value.trim().is_empty());
                                password.set(value);
                            },
                        }
                        button {
                            r#type: "button",
                            class: "absolute right-0 top-0 h-full w-12.5 bg-transparent
                            border-0 cursor-pointer flex justify-center items-center p-0 outline-0",
                            aria_label: "show_password",
                            onclick: move |_| is_password_visible.toggle(),
                            InlineSvg {
                                content: EYE_SHOW_ICON,
                                class: if is_password_visible() { "{eye_class} opacity-100" } else { "{eye_class} opacity-0" },
                            }
                            InlineSvg {
                                content: EYE_OFF_ICON,
                                class: if is_password_visible() { "{eye_class} opacity-0" } else { "{eye_class} opacity-100" },
                            }
                        }
                    }
                    button {
                        r#type: "submit",
                        class: "w-full py-3.5 px-5 rounded-lg text-[16px] font-semibold cursor-pointer mt-6
                        text-center self-center relative overflow-hidden bg-[#3498db] hover:brightness-110
                        active:scale-[0.98] active:brightness-90 text-white transition-all duration-200 ease-in-out",
                        disabled: is_verifying(),
                        onclick: move |_| async move {
                            if is_username_empty() {
                                error_message.set("用户名不能为空!".to_string());
                                is_open.set(true);
                            } else if is_password_empty() {
                                error_message.set("密码不能为空!".to_string());
                                is_open.set(true);
                            } else {
                                let login_info = LoginInfo {
                                    username: username.to_string(),
                                    password: password.to_string(),
                                };
                                login_coroutine.send(login_info);
                            }
                        },
                        "登录"
                    }
                }
            }
        }
    }
}
