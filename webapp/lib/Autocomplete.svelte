<script>
export let placeholder = 'Type something...';
export let items = ['application', 'bookmark']

/** @type {HTMLElement} */
let ref;

/** @type {HTMLElement} */
let popup;

/** @type {String} */
let pattern = '';

const onFocus = () => {
    popup.style.width = ref.offsetWidth + 'px';
    popup.style.left = ref.offsetLeft + 'px';
    popup.style.top = ref.offsetTop + ref.offsetHeight - 2 + 'px';
    popup.style.display = 'block';
}
const onFocusOut = () => {
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
        on:focus={onFocus}
        on:focusout={onFocusOut}/>
    <ul class="absolute hidden w-full p-1 overflow-auto text-sm bg-white max-h-56 focus:outline-none"
        bind:this={popup}>
        {#each items as item}
            {#if pattern.length === 0 || item.toLowerCase().includes(pattern.toLowerCase())}
                <li class="border-1">
                    <a href="#" class="flex items-center min-h-[30px]">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><title>bookmark</title><path d="M17,3H7A2,2 0 0,0 5,5V21L12,18L19,21V5C19,3.89 18.1,3 17,3Z" /></svg>
                        <span>{item}</span>
                    </a>
                </li>
            {/if}
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
 }
</style>