use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
};

use crate::app::{App, Filter, Mode};

pub fn draw(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_header(frame, app, layout[0]);
    render_todos(frame, app, layout[1]);
    render_progress(frame, app, layout[2]);
    render_help(frame, app, layout[3]);

    match app.mode {
        Mode::Editing => render_edit_modal(frame, app),
        Mode::Deleting => render_delete_modal(frame),
        Mode::Normal => {}
    }
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let filter = match app.filter {
        Filter::All => "Todas",
        Filter::Active => "Activas",
        Filter::Done => "Completadas",
    };

    let title = format!(
        " TODOs | Total: {} | Activas: {} | Completadas: {} | Filtro: {} ",
        app.total(),
        app.active(),
        app.done(),
        filter
    );

    let header = Paragraph::new(title)
        .block(Block::default().borders(Borders::ALL).title("Resumen"))
        .alignment(Alignment::Center);

    frame.render_widget(header, area);
}

fn render_todos(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .todos
        .iter()
        .filter(|todo| match app.filter {
            Filter::All => true,
            Filter::Active => !todo.done,
            Filter::Done => todo.done,
        })
        .enumerate()
        .map(|(idx, todo)| {
            let marker = if todo.done { "[x]" } else { "[ ]" };
            let line = format!("{} {}. {}", marker, idx + 1, todo.text);
            ListItem::new(line)
        })
        .collect();

    let empty_state = if items.is_empty() {
        vec![ListItem::new("No hay tareas para mostrar")]
    } else {
        items
    };

    let list = List::new(empty_state).block(Block::default().borders(Borders::ALL).title("Tareas"));

    frame.render_widget(list, area);
}

fn render_progress(frame: &mut Frame, app: &App, area: Rect) {
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progreso"))
        .ratio(app.progress())
        .label(format!("{:.0}%", app.progress() * 100.0));

    frame.render_widget(gauge, area);
}

fn render_help(frame: &mut Frame, app: &App, area: Rect) {
    let message = match app.mode {
        Mode::Normal => "a: añadir | e: editar | d: borrar | f: filtro | ↑/↓: mover | q: salir",
        Mode::Editing => "Modo edición: Enter confirmar | Esc cancelar",
        Mode::Deleting => "Confirmar borrado: Enter o y | Esc cancelar",
    };

    let help = Paragraph::new(message)
        .block(Block::default().borders(Borders::ALL).title("Atajos"))
        .wrap(Wrap { trim: true });

    frame.render_widget(help, area);
}

fn render_edit_modal(frame: &mut Frame, app: &App) {
    let popup = centered_rect(70, 30, frame.area());
    frame.render_widget(Clear, popup);

    let modal = Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Editar / Añadir tarea"),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(modal, popup);
}

fn render_delete_modal(frame: &mut Frame) {
    let popup = centered_rect(60, 20, frame.area());
    frame.render_widget(Clear, popup);

    let modal = Paragraph::new("¿Seguro que quieres eliminar la tarea seleccionada? (Enter/y para confirmar, Esc para cancelar)")
        .block(Block::default().borders(Borders::ALL).title("Confirmar eliminación"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(modal, popup);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}
