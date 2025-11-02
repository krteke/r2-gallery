pub fn get_browser_origin() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window()?;
        let window = web_sys::window()?;

        let location = window.location();
        location.origin().ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}
