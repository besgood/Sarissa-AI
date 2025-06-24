use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub context: String,
}

#[derive(Default)]
pub struct TaskBus {
    queue: VecDeque<Task>,
}

impl TaskBus {
    pub fn new() -> Self {
        TaskBus { queue: VecDeque::new() }
    }

    pub fn add_task(&mut self, task: Task) {
        self.queue.push_back(task);
    }

    pub fn get_next(&mut self) -> Option<Task> {
        self.queue.pop_front()
    }

    pub fn has_tasks(&self) -> bool {
        !self.queue.is_empty()
    }
}
