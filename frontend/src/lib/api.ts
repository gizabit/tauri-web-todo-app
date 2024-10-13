import type { Task } from './types';
import { invoke } from '@tauri-apps/api/core';

export interface Api {
    getTasks(): Promise<Task[]>;
    addTask(title: string): Promise<Task>;
    setTaskComplete(taskId: string, complete: boolean): Promise<void>;
    deleteTask(taskId: string): Promise<void>;
}

// Implementation of the Web version of Api (using fetch)
export class WebApi implements Api {
    static URL: string;

    static {
        if (import.meta.env.VITE_APP_BACKEND_URL) {
            WebApi.URL = import.meta.env.VITE_APP_BACKEND_URL;
        } else {
            WebApi.URL = window.APP_BACKEND_URL;
        }
    }

    async getTasks(): Promise<Task[]> {
        const response = await fetch(WebApi.URL + '/api/tasks');
        if (!response.ok) {
            throw new Error('Failed to fetch tasks');
        }
        return response.json();
    }

    async addTask(title: string): Promise<Task> {
        const response = await fetch(WebApi.URL + '/api/tasks', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ title })
        });
        if (!response.ok) {
            throw new Error('Failed to add task');
        }
        return response.json();
    }

    async setTaskComplete(taskId: string, complete: boolean): Promise<void> {
        const response = await fetch(WebApi.URL + `/api/tasks/${taskId}`, {
            method: 'PATCH',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ complete: complete })
        });
        if (!response.ok) {
            throw new Error('Failed to set task completion');
        }
    }

    async deleteTask(taskId: string): Promise<void> {
        const response = await fetch(WebApi.URL + `/api/tasks/${taskId}`, { method: 'DELETE' });
        if (!response.ok) {
            throw new Error('Failed to delete task');
        }
    }
}

// Implementation of the Desktop version of Api (using Tauri commands)
export class TauriApi implements Api {
    async getTasks(): Promise<Task[]> {
        return invoke<Task[]>('get_tasks');
    }

    async addTask(title: string): Promise<Task> {
        return invoke<Task>('add_task', { title });
    }

    async setTaskComplete(taskId: string, complete: boolean): Promise<void> {
        if (!(await invoke<boolean>('set_task_complete', { id: taskId, complete })))
            throw new Error('Failed to set task completion');

        this.getTasks();
    }

    async deleteTask(taskId: string): Promise<void> {
        if (!(await invoke<boolean>('delete_task', { id: taskId })))
            throw new Error('Failed to delete task');

        this.getTasks();
    }
}

// Factory function to get the correct implementation based on environment
export function createApi(): Api {
    if (import.meta.env.VITE_APP_ENV === 'tauri') {
        return new TauriApi();
    }
    return new WebApi();
}
