<script lang="ts">
    import type { Task } from '$lib/types';

    export let task: Task;
    export let setTaskComplete: (id: string, complete: boolean) => void;
    export let deleteTask: (id: string) => void;

    function handleSetTaskComplete(event: Event) {
        task.complete = (event.target as HTMLInputElement).checked;
        setTaskComplete(task.id, task.complete);
    }
</script>

<div class="flex items-center justify-between bg-white p-4 rounded-lg shadow-md mb-2">
    <div class="flex items-center">
        <input
            type="checkbox"
            checked={task.complete}
            class="mr-4"
            on:change={(event) => handleSetTaskComplete(event)}
        />
        <span class={task.complete ? 'line-through text-gray-400' : ''}>
            {task.title}
        </span>
    </div>

    <button on:click={() => deleteTask(task.id)} class="text-red-600 hover:text-red-800">
        Delete
    </button>
</div>
