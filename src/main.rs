use std::process::exit;

use caffeine_cli::caffeine::{end_protected_session, get_session, init_protected_session};
use icons::get_icon;
use tigris_rs::features::{
    actions::{ResultAction, RunExtensionAction},
    api::{
        get_extension_request, send_search_results,
        RequestType::{FormResults, GetResults, RunAction},
    },
    search_results::SearchResult,
    utils::send_notification,
};

pub mod icons;

fn main() {
    let request = get_extension_request();

    match request.request_type {
        GetResults => {
            let search_text = request.get_results_request.unwrap().search_text;
            let mut results = Vec::<SearchResult>::new();
            let session = get_session();

            if let Some(session) = session {
                results.push(
                    SearchResult::new("Disable Caffeine Session")
                        .set_icon_path(&get_icon("sleep"))
                        .set_icon_color("accent")
                        .set_action(&ResultAction::new_run_extension_action(
                            &RunExtensionAction::new("caffeine", "disable"),
                        )),
                );

                results.push(
                    SearchResult::new("Ellapsed Time")
                        .set_description(&session.get_elapsed_time())
                        .set_icon_path(&get_icon("clock"))
                        .set_icon_color("accent"),
                );

                if session.session_length.is_some() {
                    results.push(
                        SearchResult::new("Session Length")
                            .set_description(&session.get_session_length().unwrap())
                            .set_icon_path(&get_icon("clock"))
                            .set_icon_color("accent"),
                    );

                    results.push(
                        SearchResult::new("Remaining Time")
                            .set_description(&session.get_remaining_time().unwrap())
                            .set_icon_path(&get_icon("flag"))
                            .set_icon_color("accent"),
                    );
                }

                send_search_results(&results);
                exit(0);
            }

            if !search_text.trim().is_empty() && search_text.parse::<u64>().is_ok() {
                results.push(
                    SearchResult::new("Timed Caffeine Session")
                        .set_description(&format!(
                            "Start a caffeine session with {} minutes",
                            &search_text
                        ))
                        .set_icon_path(&get_icon("coffee"))
                        .set_icon_color("accent")
                        .set_action(&ResultAction::new_run_extension_action(
                            &RunExtensionAction::new("caffeine", "start-timed")
                                .add_arg(&search_text.trim()),
                        )),
                );
            } else {
                results.push(
                    SearchResult::new("Start Caffeine Session")
                        .set_icon_path(&get_icon("coffee"))
                        .set_icon_color("accent")
                        .set_action(&ResultAction::new_run_extension_action(
                            &RunExtensionAction::new("caffeine", "start"),
                        )),
                );
            }

            send_search_results(&results);
            exit(0);
        }
        RunAction => {
            let action_request = request.run_action_request.unwrap();
            let action = &action_request.action;

            match action.as_str() {
                "start" => {
                    if init_protected_session(None).is_ok() {
                        send_notification("Caffeine", "☕ Session started");
                    };
                }
                "start-timed" => {
                    let minutes = action_request.args.get(0).unwrap().parse::<u64>().unwrap();
                    let seconds = minutes * 60;

                    init_protected_session(Some(seconds)).unwrap();

                    send_notification("Caffeine", "☕ Session started");
                }
                "disable" => {
                    if end_protected_session().is_ok() {
                        send_notification("Caffeine", "😴 Session disabled");
                    }
                }
                _ => {}
            }
        }
        FormResults => {}
    }
}
