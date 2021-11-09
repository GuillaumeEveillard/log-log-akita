mod engine;

use std::{error::Error, io::{self, Stdout}};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use engine::Engine;
use tui::{Terminal, backend::Backend};
use tui::backend::CrosstermBackend;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};
use tui::layout::{Alignment, Constraint, Direction, Layout};

fn main() -> Result<(), io::Error> {
    let arg_matches = clap::App::new("Log Log Akita")
        .arg(clap::Arg::new("include")
            .about("include pattern")
            .takes_value(true)
            .short('i')
            .long("include")
            .multiple_occurrences(true))
        .arg(clap::Arg::new("exclude")
            .about("exclude pattern")
            .takes_value(true)
            .short('e')
            .long("exclude")
            .multiple_occurrences(true))
        .arg(clap::Arg::new("file")
            .about("fiel or folder")
            .takes_value(true)
            .short('f')
            .long("file")
            .multiple_occurrences(true))
        .get_matches();

    let mut filters : Vec<Box<dyn engine::Filter>> = Vec::new();

    filters.extend(match arg_matches.values_of("include") {
        None => Vec::new(),
        Some(includes) => includes.map(|v| Box::new(engine::PatternFilter::new(engine::FilterMode::Includes, v)) as Box<dyn engine::Filter>).collect()
    });

    filters.extend(match arg_matches.values_of("exclude") {
        None => Vec::new(),
        Some(excludes) => excludes.map(|v| Box::new(engine::PatternFilter::new(engine::FilterMode::Excludes, v)) as Box<dyn engine::Filter>).collect()
    });

    let files = match arg_matches.values_of("file") {
        None => Vec::new(),
        Some(ff) => ff.map(|v| std::path::PathBuf::from(v)).collect()
    };

    println!("{:?}", filters);

    let mut engine = engine::Engine::new(files, filters);
    engine.compute();
   
    let mut app = TerminalApp::new(engine)?;
    app.run();
    Ok(())

}

struct TerminalApp {
    engine: Engine,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    first_line: usize
}

impl TerminalApp {
    fn new(engine: Engine) -> Result<TerminalApp, io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(TerminalApp{engine, terminal, first_line: 0})
    }

    fn run(&mut self) -> Result<(), io::Error> {
        loop {
    
            self.terminal.draw(|f| {
                let size = f.size();
    
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Max(10000),
                            Constraint::Length(10)
                        ].as_ref()
                    )
                    .split(f.size());

                let text: Vec<Spans> = self.engine.lines(self.first_line, 100).iter()
              //  .take(10)
                .map(|l| Spans::from(vec![Span::raw(l.clone())]))
                .collect();

             /*    let txt = if self.x { "faux "} else {"vrai"};
                let text = vec![
                    Spans::from(vec![
                        Span::raw(txt),
                        Span::styled("line",Style::default().add_modifier(Modifier::ITALIC)),
                        Span::raw("."),
                    ]),
                    Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),];*/
                let data_para = Paragraph::new(text)
                    .block(Block::default().title("Data").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });
                f.render_widget(data_para, chunks[0]);
    
                let last_line = &self.first_line + chunks[0].height as usize;
                let info_para  = TerminalApp::build_info_paragraph(self.first_line, last_line, &self.engine);
                f.render_widget(info_para, chunks[1]);
            
            })?;
    
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Down => {
                        self.first_line += 1;
                    }
                    KeyCode::Up => {
                        if self.first_line > 0 {
                            self.first_line -= 1;
                        }
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn build_info_paragraph(first_line: usize, last_line: usize, engine: &Engine) -> Paragraph<'static> {
        let line_info = format!("Displaying from {} to {} (total {})", first_line, last_line, engine.number_of_lines());
        let spans: Vec<Spans> = vec![
            Spans::from(vec![Span::raw(line_info)])
        ];

        Paragraph::new(spans)
            .block(Block::default().title("Info").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
    }
}

impl Drop for TerminalApp {
    fn drop(&mut self) {
        // restore terminal
        disable_raw_mode().unwrap();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ).unwrap();
        self.terminal.show_cursor().unwrap();
    }
}

