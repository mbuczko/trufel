<script>
import { onMount, setContext } from 'svelte';
import { writable } from 'svelte/store';

/**
 * @typedef CommandMenuItem
 * @property {function(String):boolean} onMatch - returns true if item contains given pattern
 * @property {function():void} onSelect - a function to invoke when item got selected
 */

/** @type {CommandMenuItem[]} */
const items = [];

/** @type HTMLElement */
let searchElement;

/** @type HTMLElement */
let ref;

/** @type import('svelte/store').Writable<number> */
const selectedItemIdx = writable(0);

/** @type import('svelte/store').Writable<String> */
const pattern = writable('');

/** Reacts on pattern change - selects first visible item */
$: selectItem(0, $pattern, n => n+1);

/**
 * Selects next or previous item, depending on transforming
 * function f which alters lookup index forward or backward respectively.
 *
 * @param {number} startIdx - initial lookup index
 * @param {string} pattern - pattern that item needs to match
 * @param {function(number):number=} f - a function which transforms lookup index
 */
function selectItem(startIdx, pattern, f=(n)=>n) {
    let i = startIdx, len = items.length;
    do {
        // if index points at valid item and item matches given pattern
        // then it becomes a candate for selection. otherwise, transform
        // the index and re-iterate.
        if (i >= 0 && i < len && items[i].onMatch(pattern)) {
            break
        }

        i = f(i);

        // ensure sane lookup boundaries and wrap the index if necessary.
        if (i >= len) {
            i = 0
        } else if (i < 0) {
            i = len - 1
        };
    } while (i !== startIdx);
    selectedItemIdx.set(i);
}

/**
 * @param {KeyboardEvent} event - keydown event to react on up/down/enter key downs.
 * @listens KeyboardEvent
 */
const onKeydown = (event) => {
    if (event.key === 'Enter') {
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
 * Selects CommandMenuItem
 * @param {CustomEvent} event - The observable event
 * @listens itemselected
 */
const onItemSelected = ({detail: {index}}) => {
    selectItem(index, $pattern);
    searchElement.focus()
}

/**
 * Invokes the action behind chosen CommandMenuItem
 * @param {CustomEvent} event - The observable event
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
    if (ref) {
        ref.classList.remove('hidden');
        searchElement.focus();
    }
}

const closeCommandMenu = () => {
    ref && ref.classList.add('hidden');
}

onMount(() => {
    const bodyEl = document.querySelector('body');

    if (bodyEl) {
        bodyEl.addEventListener('keydown', (event) => {
            if (event.metaKey && event.key === 'k') {
                openCommandMenu();
            } else if (event.key === 'Escape') {
                closeCommandMenu();
            }
        })
        // @ts-ignore
        ref.addEventListener('itemselected', onItemSelected);
        // @ts-ignore
        ref.addEventListener('iteminvoked', onItemInvoked);

    }
})

setContext('command-menu-state', {pattern, selectedItemIdx});
setContext('command-menu-register', (/** @type {CommandMenuItem} */ item) => {
    const len = items.length;
    items.push(item);

    return {
        index: len
    };
});
</script>


<div id="command-menu"
     class="hidden flex flex-col w-full h-full overflow-hidden bg-white border rounded-lg shadow-md"
     bind:this={ref}>
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
    <div class="max-h-[320px] overflow-y-auto overflow-x-hidden">
        <slot />
    </div>
</div>
