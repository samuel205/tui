use ratatui::widgets::ListState;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    pub done: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Filter {
    All,
    Active,
    Done,
}

#[derive(Debug, Clone, Copy)]
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
    pub index: usize,
}

impl Todo {
    pub fn new(id: usize, text: String) -> Self {
        Self {
            id,
            text,
            done: false,
        }
    }

    pub fn toggle(&mut self) {
        self.done = !self.done;
    }
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(None);

        Self {
            todos: Vec::new(),
            list_state,
            filter: Filter::All,
            mode: Mode::Normal,
            input: String::new(),
            index: 0,
        }
    }

    pub fn total(&self) -> usize {
        self.todos.len()
    }

    pub fn done(&self) -> usize {
        self.todos.iter().filter(|t| t.done).count()
    }

    pub fn active(&self) -> usize {
        self.total().saturating_sub(self.done())
    }

    pub fn progress(&self) -> f64 {
        if self.total() == 0 {
            0.0
        } else {
            (self.done() as f64 / self.total() as f64).clamp(0.0, 1.0)
        }
    }

    fn sync_selection(&mut self) {
        if self.todos.is_empty() {
            self.index = 0;
            self.list_state.select(None);
            return;
        }

        if self.index >= self.todos.len() {
            self.index = self.todos.len() - 1;
        }

        self.list_state.select(Some(self.index));
    }

    pub fn up(&mut self) {
        if self.todos.is_empty() {
            return;
        }

        self.index = self.index.saturating_sub(1);
        self.sync_selection();
    }

    pub fn down(&mut self) {
        if self.todos.is_empty() {
            return;
        }

        self.index = (self.index + 1).min(self.todos.len() - 1);
        self.sync_selection();
    }

    pub fn add(&mut self) {
        self.mode = Mode::Editing;
        self.input.clear();
        self.index = self.todos.len();
    }

    pub fn edit(&mut self) {
        if let Some(todo) = self.todos.get(self.index) {
            self.mode = Mode::Editing;
            self.input = todo.text.clone();
        }
    }

    pub fn delete(&mut self) {
        if !self.todos.is_empty() {
            self.mode = Mode::Deleting;
        }
    }

    pub fn filter(&mut self) {
        self.filter = match self.filter {
            Filter::All => Filter::Active,
            Filter::Active => Filter::Done,
            Filter::Done => Filter::All,
        };
    }

    pub fn confirm(&mut self) {
        match self.mode {
            Mode::Editing => {
                let text = self.input.trim();
                if !text.is_empty() {
                    if self.todos.get(self.index).is_some() {
                        // Si veníamos de editar una tarea seleccionada, reemplazamos texto.
                        if let Some(todo) = self.todos.get_mut(self.index) {
                            todo.text = text.to_string();
                        }
                    } else {
                        let id = self.todos.len() + 1;
                        self.todos.push(Todo::new(id, text.to_string()));
                        self.index = self.todos.len() - 1;
                    }
                }
                self.input.clear();
                self.mode = Mode::Normal;
                self.sync_selection();
            }
            Mode::Deleting => {
                if !self.todos.is_empty() && self.index < self.todos.len() {
                    self.todos.remove(self.index);
                }
                self.mode = Mode::Normal;
                self.sync_selection();
            }
            Mode::Normal => {
                if let Some(todo) = self.todos.get_mut(self.index) {
                    todo.toggle();
                }
            }
        }
    }

    pub fn cancel(&mut self) {
        self.input.clear();
        self.mode = Mode::Normal;
        self.sync_selection();
    }
}
