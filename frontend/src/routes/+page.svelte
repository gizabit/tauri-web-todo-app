<script lang="ts">
    import { onMount } from 'svelte';
    import TaskItem from '../components/TaskItem.svelte';
    import type { Task } from '$lib/types';
    import { createApi } from '$lib/api';

    const api = createApi();

    let newTaskTitle = '';
    let tasks: Task[] = [];

    onMount(async () => {
        await loadTasks();
    });

    async function loadTasks() {
        tasks = await api.getTasks();
    }

    async function handleAddTask() {
        if (newTaskTitle.trim() === '') return;

        const newTask = await api.addTask(newTaskTitle);
        tasks = [...tasks, newTask];
        newTaskTitle = '';
    }

    async function handleSetTaskComplete(taskId: string, complete: boolean) {
        const task = tasks.find((t) => t.id === taskId);
        if (task) {
            await api.setTaskComplete(taskId, complete);
            task.complete = complete;
        }
    }

    async function handleDeleteTask(taskId: string) {
        await api.deleteTask(taskId);
        tasks = tasks.filter((t) => t.id !== taskId);
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            handleAddTask();
        }
    }
</script>

<div class="max-w-md mx-auto mt-10">
    <h1 class="text-3xl font-bold text-center mb-5">ToDo App</h1>

    <div class="mb-5">
        <input
            type="text"
            bind:value={newTaskTitle}
            placeholder="Enter a new task"
            class="border p-2 w-full rounded-md"
            on:keydown={handleKeydown}
        />
        <button
            on:click={handleAddTask}
            class="bg-blue-500 text-white py-2 px-4 rounded-md mt-2 w-full hover:bg-blue-600"
        >
            Add Task
        </button>
    </div>

    {#if tasks.length === 0}
        <p class="text-center text-gray-500">No tasks available</p>
    {:else}
        {#each tasks as task (task.id)}
            <TaskItem
                {task}
                setTaskComplete={handleSetTaskComplete}
                deleteTask={handleDeleteTask}
            />
        {/each}
    {/if}
</div>
