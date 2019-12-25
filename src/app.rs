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

const KEY: &'static str = "yew.todomvc.self";

pub struct App {
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    filter: Filter,
    value: String,
    edit_value: String,
    log_file: LogFile,
    log_filters: Vec<LogFilter>
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
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
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Nope,
    UpdateLogFile(String),
    NewFilter,
    UpdateFilterPattern(usize, String),
    UpdateFilterMode(usize, ChangeData)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local);
        let entries = {
            if let Json(Ok(restored_entries)) = storage.restore(KEY) {
                restored_entries
            } else {
                Vec::new()
            }
        };
        let state = State {
            entries,
            filter: Filter::All,
            value: "".into(),
            edit_value: "".into(),
            log_file: LogFile::new_empty(),
            log_filters: Vec::new()
        };
        App { storage, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let entry = Entry {
                    description: self.state.value.clone(),
                    completed: false,
                    editing: false,
                };
                self.state.entries.push(entry);
                self.state.value = "".to_string();
            }
            Msg::Edit(idx) => {
                let edit_value = self.state.edit_value.clone();
                self.state.complete_edit(idx, edit_value);
                self.state.edit_value = "".to_string();
            }
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::UpdateEdit(val) => {
                println!("Input: {}", val);
                self.state.edit_value = val;
            }
            Msg::Remove(idx) => {
                self.state.remove(idx);
            }
            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }
            Msg::ToggleEdit(idx) => {
                self.state.edit_value = self.state.entries[idx].description.clone();
                self.state.toggle_edit(idx);
            }
            Msg::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
            }
            Msg::Toggle(idx) => {
                self.state.toggle(idx);
            }
            Msg::ClearCompleted => {
                self.state.clear_completed();
            }
            Msg::Nope => {}
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
        self.storage.store(KEY, Json(&self.state.entries));
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        info!("rendered!");
        html! {
        <div>
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "Log log akita" }</h1>
                        { self.view_input() }
                    </header>
                    <section class="main">
                        <input class="toggle-all" type="checkbox" checked=self.state.is_all_completed() onclick=|_| Msg::ToggleAll />
                        <ul class="todo-list">
                            { for self.state.entries.iter().filter(|e| self.state.filter.fit(e)).enumerate().map(view_entry) }
                        </ul>
                    </section>
                    <footer class="footer">
                        <span class="todo-count">
                            <strong>{ self.state.total() }</strong>
                            { " item(s) left" }
                        </span>
                        <ul class="filters">
                            { for Filter::iter().map(|flt| self.view_filter(flt)) }
                        </ul>
                        <button class="clear-completed" onclick=|_| Msg::ClearCompleted>
                            { format!("Clear completed ({})", self.state.total_completed()) }
                        </button>
                    </footer>
                </section>
                <footer class="info">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by GGYE BORDEL DE MERDE" }<a href="https://github.com/DenisKolodin/" target="_blank">{ "Denis Kolodin" }</a></p>
                    <p>{ "Part of " }<a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a></p>
                </footer>
            </div>
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

    fn view_filter(&self, filter: Filter) -> Html<App> {
        let flt = filter.clone();
        html! {
            <li>
                <a class=if self.state.filter == flt { "selected" } else { "not-selected" }
                   href=&flt
                   onclick=|_| Msg::SetFilter(flt.clone())>
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self) -> Html<App> {
        html! {
            // You can use standard Rust comments. One line:
            // <li></li>
            <input class="new-todo"
                   placeholder="What needs to be done?"
                   value=&self.state.value
                   oninput=|e| Msg::Update(e.value)
                   onkeypress=|e| {
                       if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
                   } />
            /* Or multiline:
            <ul>
                <li></li>
            </ul>
            */
        }
    }
}

fn view_entry((idx, entry): (usize, &Entry)) -> Html<App> {
    let mut class = "todo".to_string();
    if entry.editing {
        class.push_str(" editing");
    }
    if entry.completed {
        class.push_str(" completed");
    }
    html! {
        <li class=class>
            <div class="view">
                <input class="toggle" type="checkbox" checked=entry.completed onclick=|_| Msg::Toggle(idx) />
                <label ondoubleclick=|_| Msg::ToggleEdit(idx)>{ &entry.description }</label>
                <button class="destroy" onclick=|_| Msg::Remove(idx) />
            </div>
            { view_entry_edit_input((idx, &entry)) }
        </li>
    }
}

fn view_entry_edit_input((idx, entry): (usize, &Entry)) -> Html<App> {
    if entry.editing {
        html! {
            <input class="edit"
                   type="text"
                   value=&entry.description
                   oninput=|e| Msg::UpdateEdit(e.value)
                   onblur=|_| Msg::Edit(idx)
                   onkeypress=|e| {
                      if e.key() == "Enter" { Msg::Edit(idx) } else { Msg::Nope }
                   } />
        }
    } else {
        html! { <input type="hidden" /> }
    }
}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl<'a> Into<Href> for &'a Filter {
    fn into(self) -> Href {
        match *self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Completed => "#/completed".into(),
        }
    }
}

impl Filter {
    fn fit(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }
}

impl State {
    fn total(&self) -> usize {
        self.entries.len()
    }

    fn total_completed(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| Filter::Completed.fit(e))
            .count()
    }

    fn is_all_completed(&self) -> bool {
        let mut filtered_iter = self
            .entries
            .iter()
            .filter(|e| self.filter.fit(e))
            .peekable();

        if filtered_iter.peek().is_none() {
            return false;
        }

        filtered_iter.all(|e| e.completed)
    }

    fn toggle_all(&mut self, value: bool) {
        for entry in self.entries.iter_mut() {
            if self.filter.fit(entry) {
                entry.completed = value;
            }
        }
    }

    fn clear_completed(&mut self) {
        let entries = self
            .entries
            .drain(..)
            .filter(|e| Filter::Active.fit(e))
            .collect();
        self.entries = entries;
    }

    fn toggle(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.completed = !entry.completed;
    }

    fn toggle_edit(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.editing = !entry.editing;
    }

    fn complete_edit(&mut self, idx: usize, val: String) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.description = val;
        entry.editing = !entry.editing;
    }

    fn remove(&mut self, idx: usize) {
        let idx = {
            let filter = self.filter.clone();
            let entries = self
                .entries
                .iter()
                .enumerate()
                .filter(|&(_, e)| filter.fit(e))
                .collect::<Vec<_>>();
            let &(idx, _) = entries.get(idx).unwrap();
            idx
        };
        self.entries.remove(idx);
    }
}
