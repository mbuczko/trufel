<script>
import { getContext } from "svelte";

/** @type String - title of the item */
export let title;

/** @type function - action to invoke on selection */
export let action;

/** @type String - keyboard shortcut */
export let shortcut = '';

/** @type HTMLElement */
let item;

/**
 * Returns true if title matches given pattern. Returns false otherwise.
 * @param {string} pattern - a pattern to match against
 */
const contains = (pattern) => (pattern === '') || title.toLowerCase().includes(pattern);

const register = getContext('command-menu-register');
const {pattern, selectedItemIdx} = getContext('command-menu-state');
const {index} = register({
    onMatch: contains,
    onSelect: action
});

/** Reacts on selection index change and marks itself as selected if necessary */
$: selected = $selectedItemIdx === index;

/**
 * Dispatches a custom event to notify {CommandMenu} component about selection change
 * @param {Event} event - The observable event
 */
const onItemSelected = (event) => {
    event.preventDefault();
    item.dispatchEvent(new CustomEvent('itemselected', { detail: {index}, bubbles: true }));
}

/**
 * Dispatches a custom event to notify {CommandMenu} component about item being invoked
 * @param {Event} event - The observable event
 */
const onItemInvoked = (event) => {
    event.preventDefault();
    item.dispatchEvent(new CustomEvent('iteminvoked', { detail: {index}, bubbles: true }));
}
</script>

{#if contains($pattern)}
<div class="command-item px-1"
     role="menuitem"
     tabindex="-1"
     bind:this={item}
     on:mousedown={onItemSelected}
     on:mouseup={onItemInvoked}
     data-item-index={index}>
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
{/if}
