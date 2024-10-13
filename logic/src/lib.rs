use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub complete: bool,
}

#[derive(Clone, Default)]
pub struct TaskManager {
    tasks: Arc<Mutex<Vec<Task>>>,
}

impl TaskManager {
    pub async fn get_tasks(&self) -> Vec<Task> {
        self.tasks.lock().await.clone()
    }

    pub async fn create_task(&self, title: String) -> Task {
        let mut tasks = self.tasks.lock().await;
        let id = Uuid::new_v4().to_string();
        let task = Task {
            id,
            title,
            complete: false,
        };
        tasks.push(task.clone());
        task
    }

    pub async fn delete_task(&self, id: String) -> Option<Task> {
        let mut tasks = self.tasks.lock().await;
        tasks.iter().position(|task| task.id == id).map(|pos| tasks.remove(pos))
    }

    pub async fn set_task_complete(&self, id: String, complete: bool) -> Option<Task> {
        let mut tasks = self.tasks.lock().await;
        if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
            task.complete = complete;
            Some(task.clone())
        } else {
            None
        }
    }
}
