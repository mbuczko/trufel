<script>
import { getContext } from "svelte";

/** @type String - title of the item */
export let title;

/** @type function - action invoked on selection */
export let action;

/** @type String - keyboard shortcut */
export let shortcut = "";

/** @type boolean - selection state */
let selected = false;

/** @type boolean - visibility state */
let hidden = false;

/** @type HTMLElement */
let ref;

/**
 * A random ID to recognize the item by a parent section
 */
const uuid = crypto.randomUUID();

/**
 * registerStatus function is called each time a {CommandMenu} input changes.
 * This is required to force section to hide entirely when there was not even
 * a single item matching provided pattern.
 */
const registerStatus = (getContext('command-menu-section') || {}).registerStatus;

/**
 * Dispatches a custom event to notify {CommandMenu} component about selection change
 * @param {Event} event - The observable event
 */
function onMousedown(event) {
    event.preventDefault();
    ref.dispatchEvent(new CustomEvent('itemselected', { detail: uuid, bubbles: true }));
    
    // call item's action if provided
    if (action) action();
}

getContext('command-menu').registerItem(uuid, {
    toggleActive: (/** @type boolean */ isActive) => selected = isActive,
    toggleHidden: (/** @type boolean */ isHidden) => {
        // registerStatus function might not be provided if item
        // is not wrapped into a section.
        if (registerStatus) {
            registerStatus(uuid, isHidden)
        }
        hidden = isHidden;
    },
    matchesTitle: (/** @type String */  pattern) => title.toLowerCase().includes(pattern),
    invokeAction: () => { if (action) action() },
    isHidden: () => hidden
})
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div bind:this={ref}
     on:mousedown={onMousedown}
     class="px-1 {hidden ? 'hidden' : ''}">
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
