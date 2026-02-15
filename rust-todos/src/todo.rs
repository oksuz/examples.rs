#[derive(Debug)]
pub struct ToDo {
    pub id: u32,
    pub label: String,
    pub done: bool,
}

pub struct ToDoList(Vec<ToDo>);

impl ToDoList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn create_new_todo(&mut self, label: String) -> u32 {
        let id = self.0.len() as u32 + 1;
        self.0.push(ToDo {
            id,
            label,
            done: false,
        });
        id
    }

    pub fn get_by_id(&mut self, id: u32) -> Option<&mut ToDo> {
        self.0.iter_mut().find(|todo| todo.id == id)
    }

    pub fn mark_as_done(&mut self, id: u32) -> Result<(), String> {
        self.get_by_id(id)
            .map(|todo| todo.done = true)
            .ok_or_else(|| "Todo not found".to_string())
    }

    pub fn get_all(&self) -> &[ToDo] {
        &self.0
    }
}

impl Default for ToDoList {
    fn default() -> Self {
        Self::new()
    }
}
