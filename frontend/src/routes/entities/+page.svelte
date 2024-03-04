<script lang="ts">
    import type {PageData} from "./$types";
    import Time from "svelte-time";

    // Get entities from page load
    export let data: PageData;
    let entities = data.entities;
</script>

<div class="container h-full mx-auto flex justify-center items-center">
    <ul role="list" class="divide-y divide-gray-100">
        {#each entities as entity (`${entity.area}.${entity.entity_id}`)}
            <li class="px-8 py-4 flex items-center gap-x-6 overflow-hidden">
                <strong class="font-semibold flex-4">{entity.unique_id()}</strong>
                <span class="flex-3">{entity.state}</span>
                <Time class="flex-2" timestamp={entity.timestamp} format="HH:mm DD.MM.YYYY"/>
                <div class="flex flex-1 justify-end">
                    <a href="/entity/{entity.area}/{entity.entity_id}">></a>
                </div>
            </li>
        {/each}
    </ul>
</div>
