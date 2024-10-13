use logic::{Task, TaskManager};
use tauri::{command, Builder, State};

#[derive(Clone)]
struct AppState {
    task_manager: TaskManager,
}

#[command]
async fn get_tasks(state: State<'_, AppState>) -> Result<Vec<Task>, ()> {
    Ok(state.task_manager.get_tasks().await)
}

#[command]
async fn add_task(state: State<'_, AppState>, title: String) -> Result<Task, ()> {
    Ok(state.task_manager.create_task(title).await)
}

#[command]
async fn delete_task(state: State<'_, AppState>, id: String) -> Result<bool, ()> {
    Ok(state.task_manager.delete_task(id).await.is_some())
}

#[command]
async fn set_task_complete(
    state: State<'_, AppState>,
    id: String,
    complete: bool,
) -> Result<bool, ()> {
    Ok(state
        .task_manager
        .set_task_complete(id, complete)
        .await
        .is_some())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_tasks,
            add_task,
            delete_task,
            set_task_complete
        ])
        .manage(AppState {
            task_manager: TaskManager::default(),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
