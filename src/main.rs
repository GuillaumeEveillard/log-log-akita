#![recursion_limit = "512"]

mod engine;
mod cli;

use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;
use std::string::ToString;
use crate::Tab::RawLog;

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    active_tab: Tab,
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
pub enum Tab {Viewer, RawLog}

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
    UpdateFilterMode(usize, ChangeData),
    SelectTab(Tab)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            active_tab: RawLog,
            log_file: LogFile::new_empty(),
            log_filters: Vec::new()
        };
        App { link, state }
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
                        let mode = match selectElem.value().as_str() {
                            "Includes" => { FilterMode::Includes }
                            "Excludes" => { FilterMode::Excludes }
                            _ => panic!("Unknown mode ")
                        };

                        self.state.log_filters[id].mode = mode;
                    }
                    _ => panic!("What?!")
                }
            }
            Msg::SelectTab(tab) => {
                self.state.active_tab = tab;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let viewer_tab_style = match self.state.active_tab {
            Tab::Viewer => { " active" }
            Tab::RawLog => { "" }
        };
        let raw_log_tab_style = match self.state.active_tab {
            Tab::Viewer => { "" }
            Tab::RawLog => { " active" }
        };

        html! {
        <div>
            <h1>{"Log Log Akita"}</h1>
            <ul class="nav nav-tabs">
                <li class="nav-item">
                    <a class= {"nav-link".to_owned()+viewer_tab_style} href="#" onclick=self.link.callback(|_| Msg::SelectTab(Tab::Viewer)) >{"Viewer"}</a>
                </li>
                <li class="nav-item">
                    <a class={"nav-link".to_owned()+raw_log_tab_style} href="#" onclick=self.link.callback(|_| Msg::SelectTab(Tab::RawLog)) >{"Raw log"}</a>
                </li>
            </ul>
            <div class="tab-content">
                <div class={"tab-pane".to_owned()+viewer_tab_style} id="p1">

                <div>
                    {for self.state.log_filters.iter().map(|filter| self.view_one_filter(filter))}
                    <a
                        href="#"
                        onclick=self.link.callback(|_| Msg::NewFilter)>
                        {"New filter"}
                    </a>
                </div>
                <div>
                    {for self.state.log_file.lines.iter()
                        .filter(|l| line_matches(&self.state.log_filters, l))
                        .map(|l| self.view_line_of_log(l))}
                </div>

                </div>
                <div class={"tab-pane".to_owned()+raw_log_tab_style} id="p2">
                     <textarea
                     placeholder="past the log here"
                     cols="40" rows="5"
                     oninput=self.link.callback(|e: InputData| Msg::UpdateLogFile(e.value))>
                      </textarea>
                </div>
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
    fn view_one_filter(&self, filter: &LogFilter) -> Html {
        let id = filter.id;
        html! {
            <li>
                <select onchange=self.link.callback(move |e| Msg::UpdateFilterMode(id, e))>
                    <option>{"Includes"}</option>
                    <option>{"Excludes"}</option>
                </select>
                <input placeholder="insert a pattern" value={filter.pattern.clone()} oninput=self.link.callback(move |e: InputData| Msg::UpdateFilterPattern(id, e.value)) />
            </li>
        }
    }

    fn view_line_of_log(&self, line: &str) -> Html {
        html! {
            <p>{line}</p>
        }
    }
}


fn main() {
    yew::start_app::<App>();
}