<script>
import { getContext } from "svelte";

/** @type String - title of the item */
export let title;
/** @type String - keyboard shortcut */
export let shortcut = "";
/** @type boolean - selection state */
export let selected = false;
/** @type boolean - visibility state */
let hidden = false;

/**
 * A random ID to recognize the item by a parent section
 */
const uuid = crypto.randomUUID();
const registerStatus = getContext('command-menu-section').registerStatus;

/** @type HTMLElement */
let ref;

/**
 * Dispatches a custom event to notify {CommandMenu} component about selection change
 * @param {Event} event - The observable event
 */
function dispatchEvent(event) {
    event.preventDefault();
    ref.dispatchEvent(new CustomEvent('itemselected', { detail: uuid, bubbles: true }));
}

getContext('command-menu').registerItem(uuid, {
    toggleActive: (/** @type boolean */ isActive) => selected = isActive,
    toggleHidden: (/** @type boolean */ isHidden) => {
        registerStatus(uuid, isHidden)
        hidden = isHidden;
    },
    matchesTitle: (/** @type String */  pattern) => title.toLowerCase().includes(pattern),
    isHidden: () => hidden
})
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div bind:this={ref} on:mousedown={dispatchEvent} class="px-1 {hidden ? 'hidden' : ''}">
    <div class="relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 text-gray-900 {selected ? 'bg-neutral-100' : ''}" id="calendar-command-1">
        <span>
            <slot />
        </span>
        <span>{title}</span>
        {#if shortcut.length}
            <span class="ml-auto text-xs tracking-widest text-muted-foreground">{shortcut}</span>
        {/if}
    </div>
</div>
