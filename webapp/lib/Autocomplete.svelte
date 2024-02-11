<script>
export let placeholder = 'Type something...';

/**
 * @typedef Item
 * @property {string} id
 * @property {string} label
 * @property {string} icon
 */

/** @type {Item[]} */
export let items = [
    {
        id: 'app',
        label: 'Application',
        icon: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><title>bookmark-box-outline</title><path d="M5 3H19C20.1 3 21 3.89 21 5V19C21 19.53 20.79 20.04 20.41 20.41C20.04 20.79 19.53 21 19 21H5C4.47 21 3.96 20.79 3.59 20.41C3.21 20.04 3 19.53 3 19V5C3 3.89 3.89 3 5 3M19 19V5H5V19H19M17 7H12V15L14.5 13.5L17 15V7Z" /></svg>'
    },
    {
        id: 'book',
        label: 'Bookmark',
        icon: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><title>bookmark-outline</title><path d="M17,18L12,15.82L7,18V5H17M17,3H7A2,2 0 0,0 5,5V21L12,18L19,21V5C19,3.89 18.1,3 17,3Z" /></svg>'

    },
    {
        id: 'aa',
        label: 'Citadel',
        icon: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><title>bookmark-outline</title><path d="M17,18L12,15.82L7,18V5H17M17,3H7A2,2 0 0,0 5,5V21L12,18L19,21V5C19,3.89 18.1,3 17,3Z" /></svg>'

    }
]

/** @type {HTMLInputElement} */
let ref;

/** @type {HTMLElement} */
let popup;

/** @type {String} */
let pattern = '';

/** @type {number} */
let selectedItemIdx = -1;

$: filteredItems = filter(items, pattern);

/**
 * Filters input items (on label) agains given pattern.
 *
 * @param {Item[]} items - items to filter
 * @param {string} pattern - pattern to apply on each item
 */
const filter = (items, pattern) => {
    const lowered = pattern.toLowerCase();
    const isEmpty = pattern.length === 0;
    return items.filter((item) => isEmpty || item.label.toLowerCase().includes(lowered));
}

const onFocus = () => {
    popup.style.width = ref.offsetWidth + 'px';
    popup.style.left = ref.offsetLeft + 'px';
    popup.style.top = ref.offsetTop + ref.offsetHeight - 2 + 'px';
    popup.style.display = 'block';
    selectedItemIdx = -1;
}

const onFocusOut = () => {
    popup.style.display = 'none';
}

/**
 * @param {KeyboardEvent} event - keydown event to react on up/down arrows.
 * @listens KeyboardEvent
 */
const onKeydown = (event) => {
    if (event.key === 'Enter') {
        if (selectedItemIdx >= 0) {
            onSelect(filteredItems[selectedItemIdx])
        }
    } else if (event.key === 'ArrowDown') {
        event.preventDefault();
        if (++selectedItemIdx >= filteredItems.length) {
            selectedItemIdx = 0;
        }
    } else if (event.key === 'ArrowUp') {
        event.preventDefault();
        if (--selectedItemIdx < 0) {
            selectedItemIdx = filteredItems.length-1
        }
    }
}

/**
 * Called on item selection.
 * @param {Item} item
 */
const onSelect = (item) => {
    ref.value = item.label;
    popup.style.display = 'none';
}
</script>

<span class="autocomplete">
    <input
        type="text"
        class="w-full text-sm bg-transparent border-0 rounded-md outline-none focus:outline-none focus:ring-0 focus:border-0 placeholder:text-neutral-400 disabled:cursor-not-allowed disabled:opacity-50"
        placeholder={placeholder}
        autocomplete="off"
        autocorrect="off"
        spellcheck="false"
        bind:value={pattern}
        bind:this={ref}
        on:mousedown={onFocus}
        on:keydown={onKeydown}
        on:focus={onFocus}
        on:focusout={onFocusOut}/>
    <ul class="absolute hidden w-full p-1 overflow-auto text-sm bg-white max-h-56 focus:outline-none"
        bind:this={popup}>
        {#each filteredItems as {id, label, icon}, idx}
            <li class="flex gap-1 items-center min-h-[30px] border-1 {selectedItemIdx === idx ? 'selected' : ''}"
                data-item-id={id}
                on:mousedown={() => onSelect(items[idx])}
                on:mouseup={() => onSelect(items[idx])}>
                {@html icon}
                <span>{label}</span>
            </li>
        {/each}
    </ul>
</span>

<style>
 .autocomplete input:focus {
     border: var(--dialog-border-active);
     box-shadow: 0 0 2px var(--dialog-button-active-shadow-color);
 }
 .autocomplete ul {
     border: var(--dialog-border-active);
     border-top: none;
     border-radius: 0 0 3px 3px;
     box-shadow: 0 0 2px var(--dialog-button-active-shadow-color);
 }
 .autocomplete li:hover {
     background-color: #eef;
     border-radius: 2px;
     cursor: pointer;
 }
 .autocomplete li.selected {
     background-color: #eef;
 }
</style>
