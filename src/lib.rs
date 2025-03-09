use itertools::Itertools;
use std::iter::zip;

use config::{store_api_token_in_keyring, ConfiguredServices};
use crossterm::event::{self, Event};
use nerd_fonts::NerdFonts;
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, Cell, HighlightSpacing, Padding, Paragraph, Row, Table, Tabs, Widget},
    DefaultTerminal,
};
use rpassword;

#[macro_export]
macro_rules! catppuccin {
    () => {
        catppuccin::PALETTE.mocha.colors
    };
}

pub mod cli_args;
pub mod config;

pub fn authenticate() {
    let config = ConfiguredServices::from_file();
    if config.github.is_some() {
        let username = config.github.unwrap();
        let api_token = rpassword::prompt_password(format!("GitHub API token for {username}: "))
            .expect("API token should be readable by program.");
        store_api_token_in_keyring(config::ApiService::GitHub, &username, &api_token);
    }

    if config.jira.is_some() {
        let username = config.jira.unwrap();
        let api_token = rpassword::prompt_password(format!("Jira API token for {username}: "))
            .expect("API token should be readable by program.");
        store_api_token_in_keyring(config::ApiService::Jira, &username, &api_token);
    }
}

pub fn run_tui() {
    // let conf = Config::load();
    let terminal = ratatui::init();
    match run(terminal) {
        Ok(_) => println!("All went okay"),
        Err(e) => eprint!("Error: {e}"),
    };
    ratatui::restore();
}

fn run(mut terminal: DefaultTerminal) -> Result<(), String> {
    let app = App::new();
    loop {
        terminal
            .draw(|frame| frame.render_widget(&app, frame.area()))
            .map_err(|e| e.to_string())?;

        if matches!(event::read().map_err(|e| e.to_string())?, Event::Key(_)) {
            break Ok(());
        }
    }
}

struct Data {
    name: String,
    address: String,
    email: String,
}

impl Data {
    const fn ref_array(&self) -> [&String; 3] {
        [&self.name, &self.address, &self.email]
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn address(&self) -> &str {
        &self.address
    }

    fn email(&self) -> &str {
        &self.email
    }
}

struct App {
    items: Vec<Data>,
}

impl App {
    fn new() -> Self {
        Self {
            items: generate_fake_names(),
        }
    }

    fn render_table(&self, area: Rect, buf: &mut Buffer) {
        let header_style = Style::default()
            .fg(catppuccin!().text.into())
            .bg(catppuccin!().blue.into());
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(catppuccin!().mauve.into());
        let selected_col_style = Style::default().fg(catppuccin!().pink.into());
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(catppuccin!().pink.into());

        let header = ["Name", "Address", "Email"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => catppuccin!().base.into(),
                _ => catppuccin!().surface0.into(),
            };
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(catppuccin!().yellow.into()).bg(color))
                .height(4)
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(10 + 1),
                Constraint::Min(20 + 1),
                Constraint::Min(20),
            ],
        )
        .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        // .bg(catppuccin!().base.into())
        .highlight_spacing(HighlightSpacing::Always)
        .block(
            Block::bordered()
                .border_set(symbols::border::ROUNDED)
                .padding(Padding::horizontal(1))
                .border_style(Style::default().fg(catppuccin!().blue.into())),
        );
        t.render(area, buf);
    }

    fn render_details_asside(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Hello, World!")
            .block(
                Block::bordered()
                    .border_set(symbols::border::ROUNDED)
                    .padding(Padding::horizontal(1))
                    .border_style(Style::default().fg(catppuccin!().blue.into())),
            )
            .render(area, buf);
    }

    fn render_status_line(&self, area: Rect, buf: &mut Buffer) {
        let nf = NerdFonts {
            nf: NerdFonts::load(),
        };
        let nf_custom_c = nf.get("ple-right_half_circle_thick").unwrap();

        let bg_colours: Vec<Color> = vec![
            catppuccin!().rosewater.into(),
            catppuccin!().flamingo.into(),
            catppuccin!().mauve.into(),
            catppuccin!().sky.into(),
            catppuccin!().sapphire.into(),
        ];
        let bg_colours = zip(bg_colours.iter().cycle(), bg_colours.iter().cycle().skip(1));

        let base_text_segment_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(catppuccin::PALETTE.mocha.colors.crust.into());

        let segments = vec!["Hello there", "Oh yes", "Very nice"];

        let line = Line::from(
            zip(segments.iter(), bg_colours)
                .flat_map(|(content, (current_colour, next_colour))| {
                    vec![
                        Span::style(
                            format!(" {content} ").into(),
                            base_text_segment_style.clone().bg(current_colour.clone()),
                        ),
                        Span::style(
                            nf_custom_c.to_string().into(),
                            Style::default()
                                .fg(current_colour.clone())
                                .bg(next_colour.clone()),
                        ),
                    ]
                })
                .collect::<Vec<_>>(),
        );
        line.render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ]);
        let [header_area, content_area, status_line] = vertical.areas((area));

        let content_layout = Layout::horizontal(Constraint::from_percentages([70, 30]));
        let [table_area, details_area] = content_layout.areas(content_area);

        let tabs =
            Tabs::new(vec!["To Review", "To merge", "Failing CI", "Jira"]).highlight_style((
                Color::from(catppuccin::PALETTE.mocha.colors.base),
                Color::from(catppuccin::PALETTE.mocha.colors.pink),
                Modifier::ITALIC | Modifier::BOLD,
            ));
        tabs.render(header_area, buf);
        self.render_table(table_area, buf);
        self.render_details_asside(details_area, buf);
        self.render_status_line(status_line, buf);
    }
}

fn generate_fake_names() -> Vec<Data> {
    use fakeit::{address, contact, name};

    (0..20)
        .map(|_| {
            let name = name::full();
            let address = format!(
                "{}\n{}, {} {}",
                address::street(),
                address::city(),
                address::state(),
                address::zip()
            );
            let email = contact::email();

            Data {
                name,
                address,
                email,
            }
        })
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .collect()
}
