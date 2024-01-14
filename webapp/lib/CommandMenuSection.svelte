<script>
import { setContext } from "svelte";
import { writable } from "svelte/store";

/** @type String */
export let title;

/** @type {import("svelte/store").Writable<Object.<String, boolean>>} */
const itemsStatuses = writable({});

/** @type {Number} - Number of hidden items in a section */
let hiddens;

itemsStatuses.subscribe((value) => {
    hiddens = Object.values(value).filter(s => s).length;
})

setContext('command-menu-section', {
    registerStatus: (
        /** @type {string} */ uuid,
        /** @type {boolean} */ status) => {
            itemsStatuses.update((u) => { u[uuid] = status; return u; });
        }
})
</script>

<div class="pb-1 space-y-1 {hiddens === Object.keys(itemsStatuses).length ? 'hidden' : ''}">
    <div class="px-1 overflow-hidden text-left text-gray-700">
        <div class="px-2 py-1 my-1 text-xs font-medium text-neutral-500">{title}</div>
        <slot />
    </div>
</div>
