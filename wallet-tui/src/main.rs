use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use qrcode::render::unicode;
use qrcode::QrCode;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use std::sync::Arc;
use tui_textarea::TextArea;
use wallet_core::{Bip39Adapter, Chain, CryptoAdapter, UrAdapter, Wallet, WalletService};

#[derive(Clone)]
enum AppState {
    Menu,
    InputMnemonic(TextArea<'static>),
    InputPassphrase {
        textarea: TextArea<'static>,
        mnemonic: Option<String>,
        word_count: u8,
    },
    WalletView(Wallet, Option<String>),
    QrView {
        content: String,
        title: String,
        wallet: Wallet,
        pass: Option<String>,
    },
}

struct App {
    state: AppState,
    menu_state: ListState,
    service: Arc<WalletService>,
}

impl App {
    fn new() -> Self {
        let bip39 = Arc::new(Bip39Adapter);
        let crypto = Arc::new(CryptoAdapter);
        let airgap = Arc::new(UrAdapter);
        let service = Arc::new(WalletService::new(bip39, crypto, airgap));
        let mut menu_state = ListState::default();
        menu_state.select(Some(0));

        Self {
            state: AppState::Menu,
            menu_state,
            service,
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match &mut app.state {
                AppState::Menu => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => {
                        let i = match app.menu_state.selected() {
                            Some(i) => {
                                if i >= 2 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        app.menu_state.select(Some(i));
                    }
                    KeyCode::Up => {
                        let i = match app.menu_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    2
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        app.menu_state.select(Some(i));
                    }
                    KeyCode::Enter => match app.menu_state.selected() {
                        Some(0) => {
                            app.state = AppState::InputPassphrase {
                                textarea: TextArea::default(),
                                mnemonic: None,
                                word_count: 12,
                            }
                        }
                        Some(1) => {
                            app.state = AppState::InputPassphrase {
                                textarea: TextArea::default(),
                                mnemonic: None,
                                word_count: 24,
                            }
                        }
                        Some(2) => app.state = AppState::InputMnemonic(TextArea::default()),
                        _ => {}
                    },
                    _ => {}
                },
                AppState::InputMnemonic(textarea) => match key.code {
                    KeyCode::Esc => app.state = AppState::Menu,
                    KeyCode::Enter => {
                        let mnemonic = textarea.lines()[0].trim().to_string();
                        if app.service.import_wallet(&mnemonic).is_ok() {
                            app.state = AppState::InputPassphrase {
                                textarea: TextArea::default(),
                                mnemonic: Some(mnemonic),
                                word_count: 0, // Not used for import
                            };
                        }
                    }
                    _ => {
                        textarea.input(key);
                    }
                },
                AppState::InputPassphrase {
                    textarea,
                    mnemonic,
                    word_count,
                } => match key.code {
                    KeyCode::Esc => app.state = AppState::Menu,
                    KeyCode::Enter => {
                        let passphrase = textarea.lines()[0].trim().to_string();
                        let pass_opt = if passphrase.is_empty() {
                            None
                        } else {
                            Some(passphrase)
                        };
                        let wallet = if let Some(m) = mnemonic {
                            app.service.import_wallet(m).unwrap()
                        } else {
                            app.service.create_random_wallet(*word_count).unwrap()
                        };
                        app.state = AppState::WalletView(wallet, pass_opt);
                    }
                    _ => {
                        textarea.input(key);
                    }
                },
                AppState::WalletView(wallet, pass) => match key.code {
                    KeyCode::Esc => app.state = AppState::Menu,
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('v') => {
                        let image = generate_qr(&wallet.mnemonic);
                        app.state = AppState::QrView {
                            content: image,
                            title: " Seed QR ".to_string(),
                            wallet: wallet.clone(),
                            pass: pass.clone(),
                        };
                    }
                    KeyCode::Char('e') => {
                        let addr = app
                            .service
                            .derive_address(wallet, Chain::Evm, pass.as_deref())
                            .unwrap();
                        let image = generate_qr(&addr);
                        app.state = AppState::QrView {
                            content: image,
                            title: " ETH QR ".to_string(),
                            wallet: wallet.clone(),
                            pass: pass.clone(),
                        };
                    }
                    KeyCode::Char('s') => {
                        let addr = app
                            .service
                            .derive_address(wallet, Chain::Solana, pass.as_deref())
                            .unwrap();
                        let image = generate_qr(&addr);
                        app.state = AppState::QrView {
                            content: image,
                            title: " SOL QR ".to_string(),
                            wallet: wallet.clone(),
                            pass: pass.clone(),
                        };
                    }
                    _ => {}
                },
                AppState::QrView { wallet, pass, .. } => match key.code {
                    KeyCode::Esc => app.state = AppState::WalletView(wallet.clone(), pass.clone()),
                    _ => {}
                },
            }
        }
    }
}

fn generate_qr(text: &str) -> String {
    let code = QrCode::new(text).unwrap();
    code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build()
}

fn ui(f: &mut Frame, app: &mut App) {
    let size = f.area();
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" VIBE Hardware Wallet ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));
    f.render_widget(block, size);

    let (h_h, f_h) = if size.height < 15 { (1, 1) } else { (3, 3) };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(if size.width < 40 { 0 } else { 1 })
        .constraints([
            Constraint::Length(h_h),
            Constraint::Min(0),
            Constraint::Length(f_h),
        ])
        .split(size);

    let header_text = if size.height < 15 {
        "VIBE Wallet"
    } else {
        "VIBE Airgapped Wallet Core"
    };
    let header = Paragraph::new(header_text)
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    match &mut app.state {
        AppState::Menu => {
            let items = vec![
                ListItem::new("1. Create New 12-word Wallet"),
                ListItem::new("2. Create New 24-word Wallet"),
                ListItem::new("3. Import Wallet"),
            ];
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" Menu "))
                .highlight_style(
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, center(chunks[1], 35, 7), &mut app.menu_state);
        }
        AppState::InputMnemonic(textarea) => {
            let area = center(chunks[1], 60, 5);
            f.render_widget(Clear, area);
            textarea.set_block(Block::default().borders(Borders::ALL).title(" Mnemonic "));
            f.render_widget(textarea.widget(), area);
        }
        AppState::InputPassphrase { textarea, .. } => {
            let area = center(chunks[1], 40, 5);
            f.render_widget(Clear, area);
            textarea.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Passphrase (Opt) "),
            );
            f.render_widget(textarea.widget(), area);
        }
        AppState::WalletView(wallet, pass) => {
            let eth_addr = app
                .service
                .derive_address(wallet, Chain::Evm, pass.as_deref())
                .unwrap_or_default();
            let sol_addr = app
                .service
                .derive_address(wallet, Chain::Solana, pass.as_deref())
                .unwrap_or_default();
            let w = chunks[1].width;

            let mut lines = vec![
                Line::from(vec![
                    Span::raw("Mnem: "),
                    Span::styled(
                        if w < 50 {
                            format!("{}...", &wallet.mnemonic[..10])
                        } else {
                            wallet.mnemonic.clone()
                        },
                        Style::default().fg(Color::Magenta),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("Eth:  "),
                    Span::styled(
                        truncate(&eth_addr, w),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("Sol:  "),
                    Span::styled(
                        truncate(&sol_addr, w),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    if w < 40 {
                        "[V]Seed [E]Eth [S]Sol"
                    } else {
                        "Press [V]Seed [E]Eth [S]Sol QR"
                    },
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::ITALIC),
                )]),
            ];
            if size.height >= 20 {
                lines.insert(1, Line::from(""));
                lines.insert(3, Line::from(""));
            }

            let paragraph = Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL).title(" Wallet "))
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, chunks[1]);
        }
        AppState::QrView { content, title, .. } => {
            let qr_w = std::cmp::min(chunks[1].width, 80);
            let qr_h = std::cmp::min(chunks[1].height, 40);
            let area = center(chunks[1], qr_w, qr_h);
            f.render_widget(Clear, area);
            let qr_display = if qr_w < 25 || qr_h < 15 {
                "Screen too small".to_string()
            } else {
                content.clone()
            };
            f.render_widget(
                Paragraph::new(qr_display)
                    .block(Block::default().borders(Borders::ALL).title(title.as_str()))
                    .alignment(Alignment::Center),
                area,
            );
        }
    }

    let footer_text = if size.width < 50 {
        "[Q]Quit [ESC]Back"
    } else {
        " [Q]Quit | [UP/DOWN]Nav | [ENT]Sel | [ESC]Back | [V/E/S]QR "
    };
    f.render_widget(
        Paragraph::new(footer_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center),
        chunks[2],
    );
}

fn truncate(addr: &str, width: u16) -> String {
    let limit = (width as usize).saturating_sub(10);
    if addr.len() > limit && limit > 15 {
        format!("{}...{}", &addr[..8], &addr[addr.len() - 8..])
    } else {
        addr.to_string()
    }
}

fn center(area: Rect, width: u16, height: u16) -> Rect {
    let w = std::cmp::min(width, area.width);
    let h = std::cmp::min(height, area.height);
    let p_v = area.height.saturating_sub(h) / 2;
    let p_h = area.width.saturating_sub(w) / 2;
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(p_v),
            Constraint::Length(h),
            Constraint::Min(0),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(p_h),
            Constraint::Length(w),
            Constraint::Min(0),
        ])
        .split(v[1])[1]
}
