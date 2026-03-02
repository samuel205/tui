use ratatui::widgets::{ ListState }
 
#[derive(Debug)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    pub done: bool,
}

#[derive(Debug)]
pub enum Filter {
    All,
    Active,
    Done,
}

pub enum Mode {
    Normal,
    Editing,
    Deleting,
}

pub struct App {
    pub todos: Vec<Todo>,
    pub list_state: ListState,
    pub filter: Filter,
    pub mode: Mode,
    pub input: String,
    pub index: usize
}

pub impl Todo {
    pub fn next (id: usize, text: String) -> Self {
        Self { id, text, done: false }
    }
    pub fn toggle(&mut self) {
        self.done = !self.done;
    }
    pub fn prev (&self) -> Self {
        Self { id: self.id, text: self.text.clone(), done: self.done }
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            todos: Vec::new(),
            filter: Filter::All,
            mode: Mode::Normal,
            input: String::new(),
        };
        app
    }

    pub fn total(&self) -> usize {
        self.todos.len()
    }

    pub fn done(&self) -> usize {
        self.todos.iter().filter(|t| t.done).count()
    }

    pub fn active(&self) -> usize {
        self.total() - self.done()
    }

    pub fn progress(&self) -> f64 {
        if self.total() == 0 {
            0.0
        } else {
            (self.done() as f64 / self.total() as f64).clamp(0.0, 1.0)
        }
    }

    pub fn select_next(&mut self) {
        let len = self.todos.len();
        if len == 0 { return; }
        let next_index = self.list_state.selected().map(|i| (i + 1).min(len - 1)).unwrap_or(0);
        self.index = next_index;
    }

    pub fn select_previous(&mut self) {
        let len = self.todos.len();
        if len == 0 { return; }
        let prev_index = self.list_state.selected().map(|i| (i - 1).max(0)).unwrap_or(0);
        self.index = prev_index;
    }

    pub fn toggle_selected(&mut self) {
        if let Some(sel) = self.list_state.selected() {
            let todo = &self.todos[sel];
            todo.toggle()
        }
    }

    pub fn delete_selected(&mut self) {
        if let Some(sel) = self.list_state.selected() {
            self.todos.remove(sel);
            self.index = 0;
        }
        self.mode = Mode::Normal;
    }

    

}