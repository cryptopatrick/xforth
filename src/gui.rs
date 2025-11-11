#[cfg(feature = "gui")]
use dioxus::prelude::*;

#[cfg(feature = "gui")]
mod init;
#[cfg(feature = "gui")]
mod fund;
#[cfg(feature = "gui")]
mod test;
#[cfg(feature = "gui")]
mod utils;

#[cfg(not(feature = "gui"))]
fn main() {
    eprintln!("Error: xforth-gui must be compiled with the 'gui' feature enabled.");
    eprintln!("Run: cargo build --bin xforth-gui --features gui");
    std::process::exit(1);
}

#[cfg(feature = "gui")]

#[derive(Clone, PartialEq)]
struct AppState {
    project_name: String,
    rpc_url: String,
    use_local: bool,
    output_logs: Vec<String>,
    is_running: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            project_name: "my-x402-agent".to_string(),
            rpc_url: "https://api.devnet.solana.com".to_string(),
            use_local: false,
            output_logs: Vec::new(),
            is_running: false,
        }
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut state = use_signal(AppState::default);

    rsx! {
        style { {include_str!("../assets/style.css")} }
        div {
            class: "container",
            h1 { class: "title", "xforth GUI" }
            p { class: "subtitle", "Bootstrap x402 Solana projects with a graphical interface" }

            ConfigPanel {
                state: state,
            }

            CommandPanel {
                state: state,
            }

            OutputPanel {
                state: state,
            }
        }
    }
}

#[component]
fn ConfigPanel(state: Signal<AppState>) -> Element {
    rsx! {
        div {
            class: "panel config-panel",
            h2 { "Configuration" }

            div {
                class: "form-group",
                label { r#for: "project-name", "Project Name:" }
                input {
                    id: "project-name",
                    class: "input",
                    r#type: "text",
                    value: "{state().project_name}",
                    oninput: move |evt| {
                        state.write().project_name = evt.value().clone();
                    },
                    disabled: state().is_running,
                }
            }

            div {
                class: "form-group",
                label { r#for: "rpc-url", "RPC URL:" }
                input {
                    id: "rpc-url",
                    class: "input",
                    r#type: "text",
                    value: "{state().rpc_url}",
                    oninput: move |evt| {
                        state.write().rpc_url = evt.value().clone();
                    },
                    disabled: state().is_running || state().use_local,
                }
            }

            div {
                class: "form-group checkbox-group",
                input {
                    id: "use-local",
                    r#type: "checkbox",
                    checked: state().use_local,
                    oninput: move |evt| {
                        let checked = evt.value() == "true";
                        let mut s = state.write();
                        s.use_local = checked;
                        if checked {
                            s.rpc_url = "http://127.0.0.1:8899".to_string();
                        } else {
                            s.rpc_url = "https://api.devnet.solana.com".to_string();
                        }
                    },
                    disabled: state().is_running,
                }
                label { r#for: "use-local", "Use Local Validator" }
            }
        }
    }
}

#[component]
fn CommandPanel(mut state: Signal<AppState>) -> Element {
    let run_init = move |_| {
        let state = state.clone();
        spawn(async move {
            state.write().is_running = true;
            state.write().output_logs.clear();
            state.write().output_logs.push("Running init...".to_string());

            let project_name = state().project_name.clone();
            let rpc_url = state().rpc_url.clone();

            match init::run(&project_name, &rpc_url, false).await {
                Ok(_) => {
                    state.write().output_logs.push("✓ Init completed successfully".to_string());
                }
                Err(e) => {
                    state.write().output_logs.push(format!("✗ Init failed: {}", e));
                }
            }

            state.write().is_running = false;
        });
    };

    let run_fund = move |_| {
        let state = state.clone();
        spawn(async move {
            state.write().is_running = true;
            state.write().output_logs.clear();
            state.write().output_logs.push("Running fund...".to_string());

            let rpc_url = state().rpc_url.clone();

            match fund::run(&rpc_url, false).await {
                Ok(_) => {
                    state.write().output_logs.push("✓ Fund completed successfully".to_string());
                }
                Err(e) => {
                    state.write().output_logs.push(format!("✗ Fund failed: {}", e));
                }
            }

            state.write().is_running = false;
        });
    };

    let run_test = move |_| {
        let state = state.clone();
        spawn(async move {
            state.write().is_running = true;
            state.write().output_logs.clear();
            state.write().output_logs.push("Running test...".to_string());

            let rpc_url = state().rpc_url.clone();

            match test::run(&rpc_url, false).await {
                Ok(_) => {
                    state.write().output_logs.push("✓ Test completed successfully".to_string());
                }
                Err(e) => {
                    state.write().output_logs.push(format!("✗ Test failed: {}", e));
                }
            }

            state.write().is_running = false;
        });
    };

    rsx! {
        div {
            class: "panel command-panel",
            h2 { "Commands" }

            div {
                class: "button-group",
                button {
                    class: "button button-primary",
                    onclick: run_init,
                    disabled: state().is_running,
                    "Init Project"
                }

                button {
                    class: "button button-secondary",
                    onclick: run_fund,
                    disabled: state().is_running,
                    "Fund Wallets"
                }

                button {
                    class: "button button-tertiary",
                    onclick: run_test,
                    disabled: state().is_running,
                    "Run Test"
                }
            }
        }
    }
}

#[component]
fn OutputPanel(state: Signal<AppState>) -> Element {
    rsx! {
        div {
            class: "panel output-panel",
            h2 { "Output" }

            div {
                class: "output-box",
                if state().output_logs.is_empty() {
                    p { class: "placeholder", "Command output will appear here..." }
                } else {
                    for log in state().output_logs.iter() {
                        div {
                            class: "log-line",
                            "{log}"
                        }
                    }
                }
            }

            if state().is_running {
                div {
                    class: "loading-spinner",
                    "⟳ Running..."
                }
            }
        }
    }
}
