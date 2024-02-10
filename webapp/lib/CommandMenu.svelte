<script>
import { onMount, setContext } from 'svelte';

/** @type {Number} - max visible items in menu */
export let maxVisible = 6;

/**
 * @typedef CommandMenuItemFns
 * @property {function(boolean):void} toggleActive
 * @property {function(boolean):void} toggleHidden
 * @property {function(String):boolean} matchesTitle
 * @property {function():void} invokeAction
 * @property {function():boolean} isHidden
 *
 * @typedef CommandMenuItemData
 * @property {string} id - a unique item identifier
 * @property {CommandMenuItemFns} fns - a functitons to manipulate with item
 */

/** @type {CommandMenuItemData[]} */
const items = [];

/** @type {number} */
let selectedItemIdx = -1;

/** @type {number} */
let itemsRegistered = 0;

/** @type HTMLElement */
let searchElement;

/** @type HTMLElement */
let ref;

/** @type String */
let pattern = "";

$: onPatternChange(pattern);

/**
 * Finds index of next non-hidden item starting from given {startIndex}.
 *
 * @param {Number} startIndex - an index to start looking from.
 * @returns {Number} - an index of first found non-hidden command-menu,
 * or -1 otherwise.
 */
const findNext = (startIndex) => {
    for (let i=startIndex+1; i<items.length; i++) {
        if (!items[i].fns.isHidden()) return i;
    }
    // nothing found - start from beginning
    for (let i=0; i<=startIndex; i++) {
        if (i>=0 && !items[i].fns.isHidden()) return i;
    }
    return -1;
}

/**
 * Finds index of previous non-hidden item starting from given {startIndex}.
 *
 * @param {number} startIndex - an index to start looking from.
 * @returns {number} - an index of first found non-hidden command-menu,
 * or -1 otherwise.
 */
const findPrev = (startIndex) => {
    for (let i=startIndex-1; i>=0; i--) {
        if (!items[i].fns.isHidden()) return i;
    }
    // nothing found - start from the end
    for (let i=items.length-1; i>=startIndex; i--) {
        if (i>=0 && !items[i].fns.isHidden()) return i;
    }
    return -1;
}

/**
 * Reacts on pattern change by hiding or showing matching items.
 * @param {string} pattern - a new pattern that items should match.
 */
const onPatternChange = (pattern) => {
    let p = (pattern || "").toLowerCase();
    let c = 0;
    let f = -1;
    items.forEach((item, i) => {
        let matches = p.length === 0 || item.fns.matchesTitle(p);

        item.fns.toggleActive(false);
        item.fns.toggleHidden(!matches || (c++) >= maxVisible);

        // if there is any match, record first index and
        // make corresponding {CommandMenuItem} active.
        if (matches && (f < 0)) f = i;
    })
    if (f >= 0) {
        items[f].fns.toggleActive(true);
        selectedItemIdx = f;
    } else {
        selectedItemIdx = -1;
    }
}

/**
 * @param {KeyboardEvent} event - keydown event to react on up/down arrows.
 * @listens KeyboardEvent
 */
const onKeydown = (event) => {
    if (event.key === 'Enter') {
        if (selectedItemIdx >= 0) {
            items[selectedItemIdx].fns.invokeAction();
        }
    } else {
        if (selectedItemIdx >=0 && selectedItemIdx < items.length) {
            items[selectedItemIdx].fns.toggleActive(false)
        }
        if (event.key === 'ArrowDown') {
            selectedItemIdx = findNext(selectedItemIdx);
        } else if (event.key === 'ArrowUp') {
            selectedItemIdx = findPrev(selectedItemIdx);
        }
        if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
            event.preventDefault();
            if (selectedItemIdx >= 0) {
                items[selectedItemIdx].fns.toggleActive(true)
            }
        }
    }
}

/**
 * Selects CommandMenu item of given Id
 * @param {CustomEvent} event - The observable event
 * @listens itemselected
 */
const onItemSelected = (event) => {
    items.forEach((item, i) => {
        let selected = item.id === event.detail;
        item.fns.toggleActive(selected);
        if (selected) selectedItemIdx = i;
    })
    searchElement.focus()
}

const openMenu = () => {
    pattern = "";
    if (ref) {
        ref.classList.remove('hidden');
        searchElement.focus();
    }
}

const closeMenu = () => {
    ref && ref.classList.add('hidden');
}

onMount(() => {
    const bodyEl = document.querySelector('body');
    const commandMenuEl = document.querySelector('#command-menu');
    if (commandMenuEl) {
        // @ts-ignore
        commandMenuEl.addEventListener('itemselected', onItemSelected);
    }
    if (bodyEl) {
        bodyEl.addEventListener('keydown', (event) => {
            if (event.metaKey && event.key === 'k') {
                openMenu();
            } else if (event.key === 'Escape') {
                closeMenu();
            }
        })
    }
})

setContext('command-menu', {
    registerItem: (
        /** @type String */ id,
        /** @type CommandMenuItemFns */ fns
    ) => {
        items.push({id, fns});
        itemsRegistered++;
        if (itemsRegistered > maxVisible) {
            fns.toggleHidden(true);
        }
}}
);

</script>


<div id="command-menu"
     class="hidden flex flex-col w-full h-full overflow-hidden bg-white border rounded-lg shadow-md"
     bind:this={ref}>
    <div class="flex items-center px-3 border-b">
        <svg class="w-4 h-4 mr-0 text-neutral-400 shrink-0" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" data-darkreader-inline-stroke="" style="--darkreader-inline-stroke: currentColor;"><circle cx="11" cy="11" r="8"></circle><line x1="21" x2="16.65" y1="21" y2="16.65"></line></svg>
        <input
            bind:this={searchElement}
            bind:value={pattern}
            on:keydown={onKeydown}
            type="text" class="flex w-full px-2 py-3 text-sm bg-transparent border-0 rounded-md outline-none focus:outline-none focus:ring-0 focus:border-0 placeholder:text-neutral-400 h-11 disabled:cursor-not-allowed disabled:opacity-50" placeholder="Type a command or search..." autocomplete="off" autocorrect="off" spellcheck="false"/>
    </div>
    <div class="max-h-[320px] overflow-y-auto overflow-x-hidden">
        <slot />
    </div>
</div>
