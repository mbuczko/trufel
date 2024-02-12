<script>
import scrollIntoView from 'scroll-into-view-if-needed';
import { onMount, setContext } from 'svelte';
import { scale } from 'svelte/transition';
import { backInOut } from 'svelte/easing';
import { writable } from 'svelte/store';

/** @type {boolean} */
let active = false;

/**
 * @typedef CommandMenuItem
 * @property {function(String):boolean} onMatch - returns true if item contains given pattern
 * @property {function():void} onSelect - a function to invoke when item got selected
 */

/** @type {CommandMenuItem[]} */
const items = [];

/** @type {HTMLElement} - items container element */
let itemsElement;

/** @type {HTMLElement} - filtering input element */
let searchElement;

/** @type import('svelte/store').Writable<number> */
const selectedItemIdx = writable(0);

/** @type import('svelte/store').Writable<String> */
const pattern = writable('');

/** Reacts on pattern change - selects first visible item */
$: selectItem(0, $pattern, n => n+1);

/**
 * Highlights next or previous item, depending on transforming
 * function f which alters lookup index forward or backward respectively.
 *
 * @param {number} initIdx - initial lookup index
 * @param {string} pattern - pattern that item needs to match
 * @param {function(number):number=} f - a function which transforms lookup index
 */
function selectItem(initIdx, pattern, f=(n)=>n) {
    const len = items.length;
    const startIdx = (initIdx < 0 ? len-1 : initIdx) % len;
    
    if (len) {
        let idx = startIdx;
        do {
            if (items[idx].onMatch(pattern)) {
                selectedItemIdx.set(idx);

                // scroll to selected item if needed
                let el = itemsElement.querySelector(`div[data-item-index="${idx}"]`);
                if (el) {
                    scrollIntoView(el, { behavior: 'smooth', scrollMode: 'if-needed' });
                }
                break;
            }
            idx = f(idx) % len;
            if (idx < 0) idx = len-1; 
        } while (idx != startIdx);
     }
}

/**
 * Reacts on up/down/enter key-downs to change or confirm selection.
 * @param {KeyboardEvent} event
 * @listens KeyboardEvent
 */
const onKeydown = (event) => {
    if (event.key === 'Enter') {
        event.preventDefault();
        onItemInvoked(new CustomEvent('iteminvoked', {
            detail: {index: $selectedItemIdx}
        }))
    } else if (event.key === 'ArrowDown') {
        event.preventDefault();
        selectItem($selectedItemIdx+1, $pattern, n => n+1);
    } else if (event.key === 'ArrowUp') {
        event.preventDefault();
        selectItem($selectedItemIdx-1, $pattern, n => n-1)
    }
}

/**
 * Highlights CommandMenuItem at given index.
 * @param {CustomEvent} event
 * @listens itemselected
 */
const onItemSelected = ({detail: {index}}) => {
    selectItem(index, $pattern);
    searchElement.focus()
}

/**
 * Invokes the action behind chosen CommandMenuItem.
 * @param {CustomEvent} event
 * @listens iteminvoked
 */
const onItemInvoked = ({detail: {index}}) => {
    if (index === $selectedItemIdx) {
        let action = items[index].onSelect;

        if (action !== undefined) {
            closeCommandMenu();
            action();
        } else {
            console.warn('No action defined');
        }
    }
}

const openCommandMenu = () => {
    items.length = 0;
    active = true;

    // reset state values
    $selectedItemIdx = 0;
    $pattern = '';
}

const closeCommandMenu = () => {
    active = false;
}

onMount(() => {
    const body = document.querySelector('body');
    if (body) {
        // @ts-ignore
        body.addEventListener('itemselected', onItemSelected);
        // @ts-ignore
        body.addEventListener('iteminvoked', onItemInvoked);

        body.addEventListener('keydown', (event) => {
            if (event.metaKey && event.key === 'k') {
                openCommandMenu();
            } else if (event.key === 'Escape') {
                closeCommandMenu();
            }
        })
    }
})

setContext('command-menu-state', {pattern, selectedItemIdx});
setContext('command-menu-register', (/** @type {CommandMenuItem} */ item) => {
    const len = items.length;
    items.push(item);

    // registered item index is sufficient for now
    return {
        index: len
    };
});
</script>

{#if active}
<div
    class="flex flex-col w-full h-full overflow-hidden bg-white border rounded-lg shadow-md z-50"
    transition:scale={{ duration: 150, start: 0.9, easing: backInOut }}
    on:introstart={() => searchElement.focus()}>
    <div class="flex items-center px-3 border-b">
        <svg class="w-4 h-4 mr-0 text-neutral-400 shrink-0" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" data-darkreader-inline-stroke="" style="--darkreader-inline-stroke: currentColor;"><circle cx="11" cy="11" r="8"></circle><line x1="21" x2="16.65" y1="21" y2="16.65"></line></svg>
        <input
            type="text"
            class="flex w-full px-2 py-3 text-sm bg-transparent border-0 rounded-md outline-none focus:outline-none focus:ring-0 focus:border-0 placeholder:text-neutral-400 h-11 disabled:cursor-not-allowed disabled:opacity-50"
            placeholder="Type a command or search..."
            autocomplete="off"
            autocorrect="off"
            spellcheck="false"
            bind:this={searchElement}
            bind:value={$pattern}
            on:keydown={onKeydown}/>
    </div>
    <div
        class="max-h-[332px] overflow-y-auto overflow-x-hidden py-2"
        bind:this={itemsElement}>
        <slot />
    </div>
</div>
{/if}
