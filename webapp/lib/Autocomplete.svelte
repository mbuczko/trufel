<script>
export let placeholder = 'Type something...';
export let allowCreate = false;

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

/** @type {Item | undefined} - selected item */
let selectedItem;

/** @type {number} - currently highlighted item */
let highlightedItemIdx = -1;

/** Reacts on pattern change by narrowing items list down to those matching new pattern */
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

    // highlight first item on a list only when
    // no empty pattern was already provided.

    if (pattern && pattern.length) {
        highlightedItemIdx = 0;
    }
    return items.filter((item) => isEmpty || item.label.toLowerCase().includes(lowered));
}

const showPopup = () => {
    ref.readOnly = false;

    popup.style.width = ref.offsetWidth + 'px';
    popup.style.left = ref.offsetLeft + 'px';
    popup.style.top = ref.offsetTop + ref.offsetHeight - 2 + 'px';
    popup.style.display = 'block';
}

/**
 * Hides a popup with items and sets provided items as selected
 * @param {Item | undefined} item - an item to set as selected
 */
const closePopup = (item) => {
    if (!allowCreate && item) {
        ref.readOnly = true;
    }
    selectedItem = item;
    pattern = (item && item.label) || '';
    popup.style.display = 'none';
}

/**
 * Called on item selection.
 * @param {Event} event
 * @param {Item} item
 */
const onSelect = (event, item) => {
    event.preventDefault();
    closePopup(item);
}

/**
 * Called on item creation
 * @param {Event} _event
 * @param {string} text - text to create an item from
 */
const onCreate = (_event, text) => {
    items.push({label: text, id: '123', icon: ''});
    closePopup(items.at(-1));
}

const onFocus = () => {
    showPopup();

    // placeholder holds current value (if there is any)
    if (selectedItem) {
        placeholder = selectedItem.label;
    }

    // no item highlighted by default
    highlightedItemIdx = -1;

    // no list filtering by default
    pattern = '';
}

const onFocusOut = () => {
    closePopup(selectedItem);
}

/**
 * @param {KeyboardEvent} event - keydown event to react on up/down arrows.
 * @listens KeyboardEvent
 */
const onKeydown = (event) => {
    if (event.key === 'Enter') {
        if (highlightedItemIdx >= 0) {
            let item = filteredItems[highlightedItemIdx];
            
            // if there is a valid item selected, just accept it.
            // otherwise, if allowed, create a brand new item.
            //
            // in case of any unacceptable garbage, bail out - ignore the event.
            
            if (item) {
                onSelect(event, item);
            } else if (allowCreate) {
                onCreate(event, pattern);
            } else {
                event.preventDefault();
            }
        }
    } else if (event.key === 'ArrowDown') {
        event.preventDefault();

        // this is to enforce popup to be visible
        // in case when it has been already closed,
        // eg. by choosing other item with mouse-up
        // event.
        showPopup();

        if (++highlightedItemIdx >= filteredItems.length) {
            highlightedItemIdx = 0;
        }
    } else if (event.key === 'ArrowUp') {
        event.preventDefault();
        if (--highlightedItemIdx < 0) {
            highlightedItemIdx = filteredItems.length-1
        }
    }
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
    <ul class="absolute hidden w-full p-1 overflow-auto text-sm bg-white max-h-56 focus:outline-none z-50"
        bind:this={popup}>
        {#if filteredItems.length === 0}
            {#if allowCreate}
                <li class="selected flex gap-1 items-center min-h-[30px] border-1"
                    on:mousedown={(e) => onCreate(e, pattern)}>
                    <svg xmlns="http://www.w3.org/2000/svg"  width="24" height="24" viewBox="0 0 24 24"><title>plus</title><path d="M19,13H13V19H11V13H5V11H11V5H13V11H19V13Z" /></svg>
                    <span class="truncate text-ellipsis"> Create <strong>{pattern}</strong> </span>
                </li>
            {/if}
        {:else}
            {#each filteredItems as {id, label, icon}, idx}
                <li class="flex gap-1 items-center min-h-[30px] border-1 {highlightedItemIdx === idx ? 'selected' : ''}"
                    data-item-id={id}
                    on:mousedown={(e) => onSelect(e, items[idx])}
                    on:mouseup={(e) => onSelect(e, items[idx])}>
                    {@html icon}
                    <span>{label}</span>
                </li>
            {/each}
        {/if}
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
