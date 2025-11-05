use std::collections::HashMap;

use dioxus::{html::FileData, prelude::*};
use reqwest::Client;
use uuid::Uuid;
use web_sys::{wasm_bindgen::JsCast, window, HtmlInputElement};

use crate::{
    components::{AuthGuard, Dialog, ImageGallery, RouteButton},
    models::PresignRequest,
    server::Route,
    utils::get_browser_origin,
};

pub type FileMap = HashMap<Uuid, FileData>;

#[component]
pub fn Upload() -> Element {
    const INPUT_FILE_ID: &str = "file-picker";
    let mut uploaded_files = use_signal(|| FileMap::new());
    let mut is_presign_failed = use_signal(|| false);

    let on_file_change = move |e: Event<FormData>| {
        let file_list = e.files();
        if file_list.is_empty() {
            return;
        }

        let mut file_write = uploaded_files.write();
        let mut uuid_vec = Vec::new();

        // TODO: Need to optimize
        for file in file_list {
            let file_uuid = Uuid::new_v4();
            let extension = file
                .name()
                .rsplit_once(".")
                .map(|(_, ext)| ext)
                .unwrap_or("dat")
                .to_string();

            tracing::debug!(
                "File {} renamed to uuid {}.{}",
                &file.name(),
                file_uuid,
                extension
            );

            file_write.insert(file_uuid, file);
            uuid_vec.push(PresignRequest {
                uuid: file_uuid.to_string(),
                extension: extension,
            });
        }
        //

        spawn(async move {
            let client = Client::new();

            let base_url = get_browser_origin().unwrap_or_else(|| "".to_string());
            let url = format!("{}/api/presign", base_url);

            tracing::debug!("Uuid: {:?}", uuid_vec);

            let response = client.post(url).json(&uuid_vec).send().await;

            // TODO: Implement error handling for presign request
            match response {
                Ok(res) => {
                    tracing::debug!("{:#?}", res);

                    if res.status().is_success() {
                        is_presign_failed.set(false);
                    } else {
                        is_presign_failed.set(true);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to presign files: {}", e);

                    is_presign_failed.set(true);
                }
            }
        });
    };

    // TODO: Implement image show
    rsx! {
        AuthGuard {
            Dialog{
                open: is_presign_failed,
                title: "预签名失败",
                description: "预签名失败，请重试"
            }
            div { class: "flex flex-col justify-center items-center w-full min-h-dvh",
                {if uploaded_files.len() > 0 {
                    rsx! {
                        ImageGallery {
                            files: uploaded_files
                        }
                    }
                } else {
                    rsx! {
                        div {}
                    }
                }}
                div {
                    class: "flex flex-row justify-center items-center",
                    RouteButton {
                        name: "返回首页",
                        class: "px-6 py-3 bg-gray-600 text-white rounded-lg cursor-pointer hover:bg-gray-700 transition-colors",
                        goto: Route::Home {},
                    }
                    button {
                        class: "ml-4 px-6 py-3 bg-green-600 text-white rounded-lg cursor-pointer hover:bg-green-700 transition-colors",
                        onclick: move |_| {
                            if let Some(document) = window().and_then(|w| w.document()) {
                                if let Some(element) = document.get_element_by_id(INPUT_FILE_ID) {
                                    if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
                                        input.click();
                                        return;
                                    }
                                }
                            }
                        },
                        "选择文件上传"
                    }
                    input {
                        r#type: "file",
                        id: "{INPUT_FILE_ID}",
                        style: "display: none;",
                        multiple: true,
                        accept: "image/*",
                        onchange: on_file_change,
                    }
                }
            }
        }
    }
}
