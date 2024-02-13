<script>
import scrollIntoView from 'scroll-into-view-if-needed';
import { createEventDispatcher } from 'svelte';
import '$lib/types.js';

/** @type {AutocompleteItem[]} */
export let items = [];

export let placeholder = 'Type something...';
export let allowCreate = false;
export let maxVisible = 3;

const dispatch = createEventDispatcher();
const createSvgIcon = '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><title>plus</title><path d="M19,13H13V19H11V13H5V11H11V5H13V11H19V13Z" /></svg>';

/** @type {HTMLInputElement} */
let input;

/** @type {HTMLElement} */
let popup;

/** @type {String} */
let pattern = '';

/** @type {AutocompleteItem | undefined} */
let selectedItem;

/** Reacts on pattern change initializing highlighted item index */
$: highlightedItemIdx = (pattern.length) ? 0 : -1;

/** Reacts on pattern change by narrowing down list of items to ones matching new pattern */
$: filteredItems = filter(items, pattern);

/**
 * Narrows down a list of items to those with labels matching new pattern.
 * Empty pattern does not impact the list - all the items are returned.
 *
 * If pattern is a unique string across all the items labels, a special item with
 * id="_create_" is prepended to keep the popup behavior consistent (navigation,
 * selecting, etc.)
 *
 * @param {AutocompleteItem[]} items - items to filter
 * @param {string} pattern - pattern to apply on each item
 */
const filter = (items, pattern) => {
    const lowered = pattern.toLowerCase();
    const isEmpty = pattern.length === 0;
    const filtered = items.filter((item) => isEmpty || item.label.toLowerCase().includes(lowered));
    const isUnique = allowCreate && !isEmpty && !Boolean(filtered.find((item) => item.label.toLowerCase() === lowered));
    
    if (isUnique) {
        filtered.unshift({id: '_create_', label: pattern, icon: createSvgIcon})
    }
    return filtered;
}

/**
 * Opens a popup with list of items.
 * Dynamically deduces height of list based on maxVisible number.
 */
const showPopup = () => {
    input.readOnly = false;

    popup.style.width = input.offsetWidth + 'px';
    popup.style.left = input.offsetLeft + 'px';
    popup.style.top = input.offsetTop + input.offsetHeight - 2 + 'px';
    popup.style.display = 'block';

    // compute popup height based on individual item height * max items visible
    let children = popup.children;
    let first = children && children.length && children[0];

    if (first) {
        popup.style.maxHeight = (first.clientHeight * maxVisible + 9) + 'px';
    }
}

/**
 * Hides a popup with items and updates input field with selected item.
 * @param {AutocompleteItem | undefined} item
 */
const closePopup = (item) => {
    if (!allowCreate) {
        input.readOnly = true;
    }
    selectedItem = item;
    pattern = (item && item.label) || '';
    popup.style.display = 'none';
}

const isPopupOpen = () => {
    return popup.style.display !== 'none';
}

/**
 * Scrolls list up or down to make item at given index visible.
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
 *
 * @param {Event} event
 * @param {AutocompleteItem} item
 */
const onSelect = (event, item) => {
    event.preventDefault();

    // react accordingly if "Create..." item has been selected
    if (item.id === '_create_') {
        dispatch('create', {
            text: item.label,
            resolve: (/** @type {AutocompleteItem} */ item) => {
                closePopup(item);
            }
        });
        // item = {label: item.label, id: '123', icon: ''};
        // items.push(item);
    } else {
        closePopup(item);
    }
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
 * Called on keydown event to react on up/down/enter/escape keys.
 *
 * @param {KeyboardEvent} event
 * @listens KeyboardEvent
 */
const onKeydown = (event) => {
    if (event.key === 'Enter') {
        const item = highlightedItemIdx >= 0 && filteredItems[highlightedItemIdx];

        // if there is a valid item selected, just accept it.
        // otherwise, it's unacceptable garbage - ignore the event.

        if (item) {
            onSelect(event, item);
        } else {
            event.preventDefault();
        }
    } else if (event.key === 'ArrowDown') {
        if (++highlightedItemIdx >= filteredItems.length) {
            highlightedItemIdx = 0;
        }
        event.preventDefault();
        scrollToItem(highlightedItemIdx);
    } else if (event.key === 'ArrowUp') {
        if (--highlightedItemIdx < 0) {
            highlightedItemIdx = filteredItems.length-1
        }
        event.preventDefault();
        scrollToItem(highlightedItemIdx);
    } else if (event.key === 'Escape') {
        // close the popup if it's opened, bubble up event otherwise
        if (isPopupOpen()) {
            event.preventDefault();
            event.stopPropagation();
            closePopup(selectedItem);
        }
    } else if (allowCreate && !event.ctrlKey && !event.metaKey && !event.shiftKey) {
        // for allowCreate mode popup should be always displayed
        // as it contains possible choice between new item creation
        // and selection of already existing item, if entry matches
        // any of the items labels.
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
        bind:this={input}
        on:mousedown={onFocus}
        on:keydown={onKeydown}
        on:focus={onFocus}
        on:focusout={onFocusOut}/>
    <ul class="absolute hidden w-full p-1 overflow-y-auto overflow-x-hidden text-sm bg-white max-h-px focus:outline-none z-40"
        role="menu"
        bind:this={popup}>
        
        {#each filteredItems as item, idx}
            {@const {id, label, icon} = item}
            <li class="flex gap-1 items-center min-h-[30px] border-1 {highlightedItemIdx === idx ? 'selected' : ''}"
                role="menuitem"
                data-item-index={idx}
                on:mousedown={(e) => onSelect(e, item)}
                on:mouseup={(e) => onSelect(e, item)}>
                {@html icon}
                <span class="truncate text-ellipsis">
                    {#if id === '_create_'}
                        Create <strong>{pattern}</strong>
                    {:else}
                        {label}
                    {/if}
                </span>
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
 .autocomplete li.selected,
 .autocomplete li:hover {
     background-color: var(--menu-item-highlighted);
     border-radius: 2px;
     cursor: pointer;
 }
</style>
