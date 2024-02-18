<script>
import scrollIntoView from 'scroll-into-view-if-needed';
import { createEventDispatcher } from 'svelte';
import { Icons } from '$lib/icons.js';
import '$lib/types.js';

/** @type {AutocompleteItem[]} */
export let items = [];

export let placeholder = 'Type something...';
export let allowCreate = false;
export let maxVisible = 3;

const dispatch = createEventDispatcher();

/** @type {HTMLInputElement} */
let input;

/** @type {HTMLElement} */
let popup;

/** @type {String} */
let pattern = '';

/** @type {boolean} - new entry added, waiting for resolution */
let waiting = false;

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
    const isEmpty = pattern.length === 0;
    const lowered = pattern.toLowerCase();
    const filtered = items.filter((item) => isEmpty || item.label.toLowerCase().includes(lowered));
    const isUnique = allowCreate && !isEmpty && !Boolean(filtered.find((item) => item.label.toLowerCase() === lowered));

    if (isUnique) {
        filtered.unshift({id: '_create_', label: pattern, icon: Icons.create})
    }
    return filtered;
}

/**
 * Sets given item as selected and updates input field accordingly.
 *
 * @param {AutocompleteItem | undefined} item
 */
const setSelected = (item) => {
    selectedItem = item;
    pattern = (item && item.label) || '';
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

    // compute popup height based on individual item height * max items visible
    let children = popup.children;
    let first = children && children.length && children[0];

    popup.style.maxHeight = (first && (first.clientHeight * maxVisible + 9)) + 'px';
    popup.classList.remove('invisible');
}

const closePopup = () => {
    input.readOnly = !allowCreate;
    popup.classList.add('invisible');
}

const isPopupOpen = () => {
    return !popup.classList.contains('invisible');
}

/**
 * Scrolls list up or down to make item at given index visible.
 *
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
    event.stopPropagation();

    if (item.id !== '_create_') {
        setSelected(item);
        closePopup()
    } else {
        // store previous item to be able to restore it
        // in case when dispatched 'create' event fails and gets rejected.

        let previous = selectedItem;

        // note, waiting=true triggers onFocusOut event and closes a popup.
        //
        // optimistically assign selected item to provided one, and then in
        // an event resolution callback update it again with real response.

        selectedItem = item;
        waiting = true;

        dispatch('create', {
            text: item.label,
            resolve: (/** @type {AutocompleteItem} */ item) => {
                setTimeout(() => {
                    waiting = false;
                    setSelected(item);
                }, 300)

            },
            reject: () => {
                waiting = false;
                selectedItem = previous;

                setTimeout(() => {
                    input.focus();
                }, 100);
            }
        });
    };
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
    setSelected(selectedItem);
    closePopup();
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
        // otherwise assume it's an unacceptable garbage and ignore event.

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

            setSelected(selectedItem);
            closePopup();
        }
    } else if (allowCreate && !event.ctrlKey && !event.metaKey && !event.shiftKey) {

        // for allowCreate mode popup should be always displayed
        // as it contains possible choice between new item creation
        // and selection of listed suggestions, if entry matches
        // any of the items labels.

        showPopup();
    }
}
</script>

<div class="flex relative">
    <span class="autocomplete flex-1">
        <span class="loader absolute right-[5px] top-[5px] {waiting ? '' : 'invisible'}"></span>
        <input
            type="text"
            class="w-full text-sm bg-transparent rounded-md focus:outline-none focus:ring-0 focus:border-0 placeholder:text-neutral-400 disabled:cursor-not-allowed disabled:opacity-50"
            placeholder={placeholder}
            disabled={waiting}
            autocomplete="off"
            autocorrect="off"
            spellcheck="false"
            bind:value={pattern}
            bind:this={input}
            on:mousedown={onFocus}
            on:keydown={onKeydown}
            on:focus={onFocus}
            on:focusout={onFocusOut}/>
        <ul class="invisible absolute w-full p-1 overflow-y-auto overflow-x-hidden text-sm bg-white max-h-px focus:outline-none z-40"
            role="menu"
            bind:this={popup}>
            {#if !filteredItems.length}
                <li class="autocomplete-empty-item flex gap-1 items-center min-h-[30px] border-1 px-1">
                    {#if allowCreate}
                        {@html Icons.keys}  <span> No suggestions yet... </span>
                    {:else}
                        {@html Icons.empty} <span> No suggestions </span>
                    {/if}
                </li>
            {/if}
            {#each filteredItems as item, idx}
                {@const {id, label, icon} = item}
                <li class="autocomplete-item flex gap-1 items-center min-h-[30px] border-1 {highlightedItemIdx === idx ? 'selected' : ''}"
                    role="menuitem"
                    data-item-index={idx}
                    on:mousedown={(e) => onSelect(e, item)}
                    on:mouseup={(e) => onSelect(e, item)}
                    on:mouseenter={({target}) => {
                        // @ts-ignore
                        // this is because Safari doesn't add :hover when dragging
                        target.classList.add('selected');
                      }}
                    on:mouseleave={({target}) => {
                        // @ts-ignore
                        // this is because Safari doesn't add :hover when dragging
                        target.classList.remove('selected')
                      }}>
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
</div>

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
 .autocomplete-item.selected {
     background-color: var(--menu-item-highlighted);
     border-radius: 2px;
     cursor: pointer;
 }
 .autocomplete-empty-item {
     color: #aaa;
     fill: #aaa;
     font-size: 0.9em;
 }
</style>
