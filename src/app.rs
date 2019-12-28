use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::events::IKeyboardEvent;
use yew::format::Json;
use yew::events::ChangeData;
use yew::services::storage::{Area, StorageService};
use yew::{html, Component, ComponentLink, Href, Html, Renderable, ShouldRender};
use std::string::ToString;

pub struct App {
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    log_file: LogFile,
    log_filters: Vec<LogFilter>
}

#[derive(Serialize, Deserialize)]
struct LogFile {
    lines: Vec<String>
}

impl LogFile {
    fn new_empty() -> LogFile {
        LogFile{lines: Vec::new()}
    }
    fn new_from_string(s: &str) -> LogFile {
        let lines = s.split("\n").map(str::to_string).collect();
        LogFile{ lines }
    }
}

#[derive(Serialize, Deserialize)]
enum FilterMode {Includes, Excludes}

#[derive(Serialize, Deserialize)]
struct LogFilter {
    id: usize,
    mode: FilterMode,
    pattern: String
}

impl LogFilter {
    fn new(id: usize) -> LogFilter {
        LogFilter{id: id, mode: FilterMode::Includes, pattern: "".to_string()}
    }
}

pub enum Msg {
    UpdateLogFile(String),
    NewFilter,
    UpdateFilterPattern(usize, String),
    UpdateFilterMode(usize, ChangeData)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let state = State {
            log_file: LogFile::new_empty(),
            log_filters: Vec::new()
        };
        App { state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateLogFile(log) => {
                self.state.log_file = LogFile::new_from_string(log.as_str());
            }
            Msg::NewFilter => {
                self.state.log_filters.push(LogFilter::new(self.state.log_filters.len()))
            }
            Msg::UpdateFilterPattern(id, pattern) => {
                self.state.log_filters[id].pattern = pattern
            }
            Msg::UpdateFilterMode(id, changeData) => {
                match changeData {
                    ChangeData::Select(selectElem) => {
                        match selectElem.value() {
                            Some(s) => {
                                let mode = match s.as_str() {
                                    "Includes" => { FilterMode::Includes }
                                    "Excludes" => { FilterMode::Excludes }
                                    _ => panic!("Unknown mode ")
                                };

                                self.state.log_filters[id].mode = mode;
                            }
                            None => {}
                        }

                    }
                    _ => panic!("What?!")
                }
            }
        }
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        info!("rendered!");
        html! {
        <div>
            <h1>{"Log Log Akita"}</h1>
            <div>
                {for self.state.log_filters.iter().map(|filter| self.view_one_filter(filter))}
                <a
                    href="#"
                    onclick=|_| Msg::NewFilter>
                    {"New filter"}
                </a>
            </div>
            <div>
                {for self.state.log_file.lines.iter()
                    .filter(|l| line_matches(&self.state.log_filters, l))
                    .map(|l| self.view_line_of_log(l))}
            </div>
            <div>
                 <textarea
                 placeholder="past the log here"
                 cols="40" rows="5"
                 oninput=|e| Msg::UpdateLogFile(e.value)>
                  </textarea>
            </div>
        </div>

        }
    }
}

fn line_matches(filters: &Vec<LogFilter>, s: &str) -> bool {

    let iter = filters.iter()
        .filter(|f|  f.pattern.len() != 0);

    for f in iter {
        match f.mode {
            FilterMode::Includes => {
                if !s.contains(&f.pattern) { return false };
            },
            FilterMode::Excludes => {
                if s.contains(&f.pattern) { return false };
            },
        };
    }

    return true;
}

impl App {
    fn view_one_filter(&self, filter: &LogFilter) -> Html<App> {
        let id = filter.id;
        html! {
            <li>
                <select onchange=|e| Msg::UpdateFilterMode(id, e)>
                    <option>{"Includes"}</option>
                    <option>{"Excludes"}</option>
                </select>
                <input placeholder="insert a pattern" value={filter.pattern.clone()} oninput=|e| Msg::UpdateFilterPattern(id, e.value) />
            </li>
        }
    }

    fn view_line_of_log(&self, line: &str) -> Html<App> {
        html! {
            <p>{line}</p>
        }
    }
}
