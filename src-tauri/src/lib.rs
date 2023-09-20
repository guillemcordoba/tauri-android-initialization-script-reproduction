use tauri::{scope::ipc::RemoteDomainAccessScope, Manager, WindowBuilder, WindowUrl};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.ipc_scope().configure_remote_access(
                RemoteDomainAccessScope::new(String::from("https://tauri.app"))
                    .add_window(String::from("main")),
            );

            let window = WindowBuilder::new(
                app,
                String::from("main"),
                WindowUrl::External(
                    url::Url::parse("https://tauri.app").expect("Cannot parse url"),
                ),
            )
            .initialization_script("console.error('hello from the webview!');")
            .build()?;
            window.eval("console.error(window.__TAURI_IPC__)")?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
