<script>
import scrollIntoView from 'scroll-into-view-if-needed';

export let placeholder = 'Type something...';
export let allowCreate = false;
export let maxVisible = 3;

/**
 * @typedef Item
 * @property {string} id
 * @property {string} label
 * @property {string} icon
 */

/** @type {Item[]} */
export let items = [];

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

/** Reacts on pattern change by verifying its uniqueness across all the items */
$: isUniqueItem = isUnique(items, pattern);

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

    if (pattern.length) {
        highlightedItemIdx = 0;
    }
    return items.filter((item) => isEmpty || item.label.toLowerCase().includes(lowered));
}

/**
 * Returns true if given text is unique across all the items labels.
 * For performance reasons, verification happens only when allowCreate is on.
 * @param {Item[]} items - items to go through
 * @param {string} text - text to verify
 */
const isUnique = (items, text) => {
    const lowered = text.toLowerCase();
    if (allowCreate && text.length) {
        return !Boolean(items.find((item) => item.label.toLowerCase() === lowered));
    }
    return false;
}

const showPopup = () => {
    ref.readOnly = false;

    popup.style.width = ref.offsetWidth + 'px';
    popup.style.left = ref.offsetLeft + 'px';
    popup.style.top = ref.offsetTop + ref.offsetHeight - 2 + 'px';
    popup.style.display = 'block';

    // compute popup height based on individual item height * max items visible
    let children = popup.children;
    let first = children && children.length && children[0];

    if (first) {
        popup.style.maxHeight = (first.clientHeight * maxVisible + 9) + 'px';
    }
}

/**
 * Hides a popup with items and sets provided item as selected.
 * @param {Item | undefined} item
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
 * Scrolls list to make item at given index visible
 * @param {number} itemIndex
 */
const scrollToItem = (itemIndex) => {
    let el = popup.querySelector(`li[data-item-index="${itemIndex}"]`);
    if (el) {
        scrollIntoView(el, { scrollMode: 'if-needed',  block: 'center' });
    }
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
 * Called on item creation.
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
 * Called on keydown event to react on up/down arrows.
 * @param {KeyboardEvent} event
 * @listens KeyboardEvent
 */
const onKeydown = (event) => {
    if (event.key === 'Enter') {
        if (highlightedItemIdx >= 0) {
            let item = filteredItems[highlightedItemIdx];

            // if there is a valid item selected, just accept it.
            // otherwise, if allowed, create a brand new one.
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
        if (++highlightedItemIdx >= filteredItems.length) {
            highlightedItemIdx = 0;
        }
        scrollToItem(highlightedItemIdx);
    } else if (event.key === 'ArrowUp') {
        event.preventDefault();
        if (--highlightedItemIdx < 0) {
            highlightedItemIdx = filteredItems.length-1
        }
        scrollToItem(highlightedItemIdx);
    } else if (allowCreate && !event.ctrlKey && !event.metaKey && !event.shiftKey) {
        showPopup();
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
    <ul class="absolute hidden w-full p-1 overflow-y-auto overflow-x-hidden text-sm bg-white max-h-px focus:outline-none z-40"
        bind:this={popup}>
        {#if allowCreate && isUniqueItem}
            <li class="{filteredItems.length === 0 ? 'selected' : ''} unique flex gap-1 items-center min-h-[30px] border-1"
                on:mousedown={(e) => onCreate(e, pattern)}>
                <svg xmlns="http://www.w3.org/2000/svg"  width="24" height="24" viewBox="0 0 24 24"><title>plus</title><path d="M19,13H13V19H11V13H5V11H11V5H13V11H19V13Z" /></svg>
                <span class="truncate text-ellipsis"> Create <strong>{pattern}</strong> </span>
            </li>
        {/if}
        {#each filteredItems as {label, icon}, idx}
            <li class="flex gap-1 items-center min-h-[30px] border-1 {highlightedItemIdx === idx ? 'selected' : ''}"
                data-item-index={idx}
                on:mousedown={(e) => onSelect(e, items[idx])}
                on:mouseup={(e) => onSelect(e, items[idx])}>
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
     background-color: var(--menu-item-highlighted);
     border-radius: 2px;
     cursor: pointer;
 }
 .autocomplete li.selected {
     background-color: var(--menu-item-highlighted)
 }
</style>
