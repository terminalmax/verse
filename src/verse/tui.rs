use std::io;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph, Widget, Wrap},
};

use crate::verse::db::*;

pub struct App {
    should_close: bool,
    current_chapter: u8,
    current_scroll: u16,

    show_book_menu: bool,
    book_input_string: String,

    db: Database,
}

impl App {
    pub fn init() -> Self {
        let db =
            Database::connect_and_load(BibleBook::Genesis).expect("Couldn't connect to Database.");

        App {
            should_close: false,
            current_chapter: 1,
            db,
            current_scroll: 0,
            show_book_menu: false,
            book_input_string: String::new(),
        }
    }

    pub fn run(&mut self) {
        let mut terminal = ratatui::init();

        //Main Loop
        while !self.should_close {
            let _ = terminal.draw(|frame| self.draw(frame));
            let _ = self.handle_events();
        }

        ratatui::restore();
    }

    fn update_state(&mut self, c: char) {
        if self.show_book_menu {
            if self.book_input_string.len() > 5 {
                return;
            }

            self.book_input_string.push(c);

            let search: Option<BibleBook> = minimum_prefix_match(&self.book_input_string);

            match search {
                Some(book) => {
                    self.current_chapter = 1;
                    self.current_scroll = 0;
                    self.show_book_menu = false;
                    let _ = self.db.load_book(book);
                }
                None => {}
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    event::KeyCode::Left => self.prev_chapter(),
                    event::KeyCode::Right => self.next_chapter(),

                    event::KeyCode::Up => {
                        self.current_scroll = self.current_scroll.saturating_sub(1);
                    }
                    event::KeyCode::Down => {
                        self.current_scroll = self.current_scroll.wrapping_add(1);
                    }

                    event::KeyCode::Char('q') => self.should_close = true,
                    event::KeyCode::Char('B') => {
                        self.show_book_menu = !self.show_book_menu;
                        self.book_input_string.clear();
                    }
                    event::KeyCode::Char(c) => self.update_state(c),
                    _ => {}
                }
            }

            _ => {}
        }

        Ok(())
    }

    fn prev_chapter(&mut self) {
        if self.current_chapter == 1 {
            return;
        }
        self.current_chapter -= 1;
    }

    fn next_chapter(&mut self) {
        if self.current_chapter == self.db.book.max_chapter_count() {
            return;
        }
        self.current_chapter += 1;
    }
}

//Rendering
impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        //Render Verses
        let verse_raw_data = self
            .db
            .get_chapter(self.current_chapter)
            .expect("Invalid data given to get_chapter, check render");

        let text: Vec<Line> = verse_raw_data
            .iter()
            .enumerate()
            .map(|(i, s)| {
                Line::from(vec![
                    Span::styled(format!("{:>3} ", i + 1), Style::default().fg(Color::Yellow)), // Colored verse number
                    Span::raw(s),
                ])
            })
            .collect();

        Paragraph::new(text)
            .block(title_block(self.db.book.as_str(), self.current_chapter))
            .wrap(Wrap { trim: true })
            .scroll((self.current_scroll, 0))
            .render(area, buf);

        //Book Menu
        if self.show_book_menu {
            let menu_area = Rect {
                x: area.width / 4,
                y: area.height / 4,
                width: area.width / 2,
                height: area.height / 2,
            };
            Clear.render(menu_area, buf);

            let [display_area, input_area] =
                Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)])
                    .areas(menu_area);

            //Highlited Book List
            Paragraph::new(display_list())
                .block(
                    Block::bordered()
                        .title(Line::from("Books").centered())
                        .title_bottom(Line::from("Press B twice to reset buffer").centered())
                        .style(Style::default().bg(Color::LightBlue).fg(Color::Black)),
                )
                .wrap(Wrap { trim: true })
                .render(display_area, buf);

            //Input
            Paragraph::new(self.book_input_string.clone())
                .block(Block::new().style(Style::default().fg(Color::Black).bg(Color::Cyan)))
                .render(input_area, buf);
        }
    }
}

fn title_block(title: &str, chapter_no: u8) -> Block {
    let title = format!("{} {}", title, chapter_no);

    Block::bordered()
        .title(
            Line::from(title)
                .right_aligned()
                .style(Style::default().fg(Color::LightBlue)),
        )
        .title_bottom(
            Line::from("<q> Quit | <Up,Down> Scroll | <Left,Right> Chapter | <B> Book")
                .right_aligned()
                .style(Style::default().fg(Color::LightBlue)),
        )
        //.border_set(border::THICK)
        .style(Style::default().fg(Color::Magenta))
}

fn display_list() -> Line<'static> {
    Line::from(vec![
        Span::styled("ge", Style::default().fg(Color::Yellow)),
        Span::raw("nesis "),
        Span::styled("ex", Style::default().fg(Color::Yellow)),
        Span::raw("odus "),
        Span::styled("le", Style::default().fg(Color::Yellow)),
        Span::raw("viticus "),
        Span::styled("nu", Style::default().fg(Color::Yellow)),
        Span::raw("mbers "),
        Span::styled("de", Style::default().fg(Color::Yellow)),
        Span::raw("uteronomy "),
        Span::styled("jos", Style::default().fg(Color::Yellow)),
        Span::raw("hua "),
        Span::styled("judg", Style::default().fg(Color::Yellow)),
        Span::raw("es "),
        Span::styled("ru", Style::default().fg(Color::Yellow)),
        Span::raw("th "),
        Span::styled("1s", Style::default().fg(Color::Yellow)),
        Span::raw("amuel "),
        Span::styled("2s", Style::default().fg(Color::Yellow)),
        Span::raw("amuel "),
        Span::styled("1k", Style::default().fg(Color::Yellow)),
        Span::raw("ings "),
        Span::styled("2k", Style::default().fg(Color::Yellow)),
        Span::raw("ings "),
        Span::styled("1ch", Style::default().fg(Color::Yellow)),
        Span::raw("ronicles "),
        Span::styled("2ch", Style::default().fg(Color::Yellow)),
        Span::raw("ronicles "),
        Span::styled("ezr", Style::default().fg(Color::Yellow)),
        Span::raw("a "),
        Span::styled("ne", Style::default().fg(Color::Yellow)),
        Span::raw("hemiah "),
        Span::styled("es", Style::default().fg(Color::Yellow)),
        Span::raw("ther "),
        Span::styled("job", Style::default().fg(Color::Yellow)),
        Span::raw(" "),
        Span::styled("ps", Style::default().fg(Color::Yellow)),
        Span::raw("alms "),
        Span::styled("pr", Style::default().fg(Color::Yellow)),
        Span::raw("overbs "),
        Span::styled("ec", Style::default().fg(Color::Yellow)),
        Span::raw("clesiastes "),
        Span::styled("s", Style::default().fg(Color::Yellow)),
        Span::raw("ongOfSolomon "),
        Span::styled("i", Style::default().fg(Color::Yellow)),
        Span::raw("saiah "),
        Span::styled("je", Style::default().fg(Color::Yellow)),
        Span::raw("remiah "),
        Span::styled("la", Style::default().fg(Color::Yellow)),
        Span::raw("mentations "),
        Span::styled("eze", Style::default().fg(Color::Yellow)),
        Span::raw("kiel "),
        Span::styled("da", Style::default().fg(Color::Yellow)),
        Span::raw("niel "),
        Span::styled("ho", Style::default().fg(Color::Yellow)),
        Span::raw("sea "),
        Span::styled("joe", Style::default().fg(Color::Yellow)),
        Span::raw("l "),
        Span::styled("am", Style::default().fg(Color::Yellow)),
        Span::raw("os "),
        Span::styled("o", Style::default().fg(Color::Yellow)),
        Span::raw("badiah "),
        Span::styled("jon", Style::default().fg(Color::Yellow)),
        Span::raw("ah "),
        Span::styled("mi", Style::default().fg(Color::Yellow)),
        Span::raw("cah "),
        Span::styled("na", Style::default().fg(Color::Yellow)),
        Span::raw("hum "),
        Span::styled("hab", Style::default().fg(Color::Yellow)),
        Span::raw("akkuk "),
        Span::styled("zep", Style::default().fg(Color::Yellow)),
        Span::raw("haniah "),
        Span::styled("hag", Style::default().fg(Color::Yellow)),
        Span::raw("gai "),
        Span::styled("zec", Style::default().fg(Color::Yellow)),
        Span::raw("hariah "),
        Span::styled("mal", Style::default().fg(Color::Yellow)),
        Span::raw("achi "),
        Span::styled("mat", Style::default().fg(Color::Yellow)),
        Span::raw("thew "),
        Span::styled("mar", Style::default().fg(Color::Yellow)),
        Span::raw("k "),
        Span::styled("lu", Style::default().fg(Color::Yellow)),
        Span::raw("ke "),
        Span::styled("joh", Style::default().fg(Color::Yellow)),
        Span::raw("n "),
        Span::styled("ac", Style::default().fg(Color::Yellow)),
        Span::raw("ts "),
        Span::styled("ro", Style::default().fg(Color::Yellow)),
        Span::raw("mans "),
        Span::styled("1co", Style::default().fg(Color::Yellow)),
        Span::raw("rinthians "),
        Span::styled("2co", Style::default().fg(Color::Yellow)),
        Span::raw("rinthians "),
        Span::styled("ga", Style::default().fg(Color::Yellow)),
        Span::raw("latians "),
        Span::styled("ep", Style::default().fg(Color::Yellow)),
        Span::raw("hesians "),
        Span::styled("phili", Style::default().fg(Color::Yellow)),
        Span::raw("ppians "),
        Span::styled("c", Style::default().fg(Color::Yellow)),
        Span::raw("olossians "),
        Span::styled("1th", Style::default().fg(Color::Yellow)),
        Span::raw("essalonians "),
        Span::styled("2th", Style::default().fg(Color::Yellow)),
        Span::raw("essalonians "),
        Span::styled("1ti", Style::default().fg(Color::Yellow)),
        Span::raw("mothy "),
        Span::styled("2ti", Style::default().fg(Color::Yellow)),
        Span::raw("mothy "),
        Span::styled("t", Style::default().fg(Color::Yellow)),
        Span::raw("itus "),
        Span::styled("phile", Style::default().fg(Color::Yellow)),
        Span::raw("mon "),
        Span::styled("he", Style::default().fg(Color::Yellow)),
        Span::raw("brews "),
        Span::styled("ja", Style::default().fg(Color::Yellow)),
        Span::raw("mes "),
        Span::styled("1p", Style::default().fg(Color::Yellow)),
        Span::raw("eter "),
        Span::styled("2p", Style::default().fg(Color::Yellow)),
        Span::raw("eter "),
        Span::styled("1j", Style::default().fg(Color::Yellow)),
        Span::raw("ohn "),
        Span::styled("2j", Style::default().fg(Color::Yellow)),
        Span::raw("ohn "),
        Span::styled("3", Style::default().fg(Color::Yellow)),
        Span::raw("John "),
        Span::styled("jude", Style::default().fg(Color::Yellow)),
        Span::raw(" "),
        Span::styled("re", Style::default().fg(Color::Yellow)),
        Span::raw("velation "),
    ])
}

fn minimum_prefix_match(prefix: &str) -> Option<BibleBook> {
    match prefix {
        "ge" => Some(BibleBook::Genesis),
        "ex" => Some(BibleBook::Exodus),
        "le" => Some(BibleBook::Leviticus),
        "nu" => Some(BibleBook::Numbers),
        "de" => Some(BibleBook::Deuteronomy),
        "jos" => Some(BibleBook::Joshua),
        "judg" => Some(BibleBook::Judges),
        "ru" => Some(BibleBook::Ruth),
        "1s" => Some(BibleBook::FirstSamuel),
        "2s" => Some(BibleBook::SecondSamuel),
        "1k" => Some(BibleBook::FirstKings),
        "2k" => Some(BibleBook::SecondKings),
        "1ch" => Some(BibleBook::FirstChronicles),
        "2ch" => Some(BibleBook::SecondChronicles),
        "ezr" => Some(BibleBook::Ezra),
        "ne" => Some(BibleBook::Nehemiah),
        "es" => Some(BibleBook::Esther),
        "job" => Some(BibleBook::Job),
        "ps" => Some(BibleBook::Psalms),
        "pr" => Some(BibleBook::Proverbs),
        "ec" => Some(BibleBook::Ecclesiastes),
        "s" => Some(BibleBook::SongOfSolomon),
        "i" => Some(BibleBook::Isaiah),
        "je" => Some(BibleBook::Jeremiah),
        "la" => Some(BibleBook::Lamentations),
        "eze" => Some(BibleBook::Ezekiel),
        "da" => Some(BibleBook::Daniel),
        "ho" => Some(BibleBook::Hosea),
        "joe" => Some(BibleBook::Joel),
        "am" => Some(BibleBook::Amos),
        "o" => Some(BibleBook::Obadiah),
        "jon" => Some(BibleBook::Jonah),
        "mi" => Some(BibleBook::Micah),
        "na" => Some(BibleBook::Nahum),
        "hab" => Some(BibleBook::Habakkuk),
        "zep" => Some(BibleBook::Zephaniah),
        "hag" => Some(BibleBook::Haggai),
        "zec" => Some(BibleBook::Zechariah),
        "mal" => Some(BibleBook::Malachi),
        "mat" => Some(BibleBook::Matthew),
        "mar" => Some(BibleBook::Mark),
        "lu" => Some(BibleBook::Luke),
        "joh" => Some(BibleBook::John),
        "ac" => Some(BibleBook::Acts),
        "ro" => Some(BibleBook::Romans),
        "1co" => Some(BibleBook::FirstCorinthians),
        "2co" => Some(BibleBook::SecondCorinthians),
        "ga" => Some(BibleBook::Galatians),
        "ep" => Some(BibleBook::Ephesians),
        "phili" => Some(BibleBook::Philippians),
        "c" => Some(BibleBook::Colossians),
        "1th" => Some(BibleBook::FirstThessalonians),
        "2th" => Some(BibleBook::SecondThessalonians),
        "1ti" => Some(BibleBook::FirstTimothy),
        "2ti" => Some(BibleBook::SecondTimothy),
        "t" => Some(BibleBook::Titus),
        "phile" => Some(BibleBook::Philemon),
        "he" => Some(BibleBook::Hebrews),
        "ja" => Some(BibleBook::James),
        "1p" => Some(BibleBook::FirstPeter),
        "2p" => Some(BibleBook::SecondPeter),
        "1j" => Some(BibleBook::FirstJohn),
        "2j" => Some(BibleBook::SecondJohn),
        "3" => Some(BibleBook::ThirdJohn),
        "jude" => Some(BibleBook::Jude),
        "re" => Some(BibleBook::Revelation),
        _ => None,
    }
}
