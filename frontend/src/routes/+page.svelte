<script lang="ts">
    import type {PageData} from "./$types";
    import {invalidateAll} from "$app/navigation";

    export let data: PageData;
    let todos = data.todos;

    async function deleteTodo(id: number) {
        await fetch(`http://localhost:8000/delete/${id}`);
        await invalidateAll();
    }

    async function updateTodo(todo) {
        await fetch(`http://localhost:8000/update?id=${todo.id}&description=${todo.description}&done=${todo.done}`);
    }
</script>

<div class="container mx-auto mt-16">
    <h1 class="h1 text-center">Todos</h1>
    <form action="http://localhost:8000/create" method="POST">
        <input type="text" class="input p-4 my-8" name="description" placeholder="What needs to be done?" autocomplete="off" />
    </form>

    <div class="space-y-4">
        {#each data.todos as todo }
            <div class="flex items-center justify-between p-4 bg-surface-800 rounded-lg gap-4">
                <input type="checkbox" class="checkbox" bind:checked={todo.done} on:change={() => updateTodo(todo)} />
                <input type="text" class="input" bind:value={todo.description} placeholder="Description" />
                <div class="flex gap-2">
                    <button class="btn variant-filled-primary" on:click={() => {updateTodo(todo)}}>Update</button>
                    <button class="btn variant-filled-primary" on:click={() => {deleteTodo(todo.id)}}>Delete</button>
                </div>
            </div>
        {/each}
    </div>
</div>