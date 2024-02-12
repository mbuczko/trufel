<script>
import Toggle from './Toggle.svelte';
import IconsComposer from './IconComposer.svelte';
import Autocomplete from './Autocomplete.svelte';

/** @type {HTMLDialogElement} */
let ref;

/** @type {HTMLElement} */
let defaultInput;

/** @type {boolean} */
let iconComposer = false;

/** @type {string} - SVG icon definition */
let svgIcon = '';

/** @type {string} */
let url;

/** @type {string} */
let title;

/** @type {string} */
let description;

/** @type {boolean} */
let isShared;

/** @type {boolean} */
let isSearchable;

let categories = [
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

    },
    {
        id: 'bb',
        label: 'Cindarella',
        icon: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><title>bookmark-outline</title><path d="M17,18L12,15.82L7,18V5H17M17,3H7A2,2 0 0,0 5,5V21L12,18L19,21V5C19,3.89 18.1,3 17,3Z" /></svg>'

    }

]

let types = [
    {
        id: 'app',
        label: 'Application',
        icon: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><title>bookmark-box-outline</title><path d="M5 3H19C20.1 3 21 3.89 21 5V19C21 19.53 20.79 20.04 20.41 20.41C20.04 20.79 19.53 21 19 21H5C4.47 21 3.96 20.79 3.59 20.41C3.21 20.04 3 19.53 3 19V5C3 3.89 3.89 3 5 3M19 19V5H5V19H19M17 7H12V15L14.5 13.5L17 15V7Z" /></svg>'
    },
    {
        id: 'bookmark',
        label: 'Bookmark',
        icon: '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><title>bookmark-outline</title><path d="M17,18L12,15.82L7,18V5H17M17,3H7A2,2 0 0,0 5,5V21L12,18L19,21V5C19,3.89 18.1,3 17,3Z" /></svg>'

    }
]

export const openDialog = () => {
    
    // reset values to default ones each time
    // dialog is re-opened.

    url = '';
    title = '';
    description = '';
    isShared = false;
    isSearchable = false;
    svgIcon = '';
    
    ref.showModal();
    defaultInput.focus()
}
</script>

<dialog id="dialog"
        class="create-dialog w-full min-w-[540px] max-w-screen-md max-h-[270px] bg-slate-500 shadow-md border-1 rounded-lg"
        bind:this={ref}>
    <div class="flex">
        <div class="sidebar p-2 grid grid-cols-[64px_1fr] justify-items-start">
            <button
                class="self-start icon w-14 h-14 mb-4 border-2 border-slate-700 opacity-80 rounded-lg shadow-md active:shadow-none hover:opacity-100"
                on:click={() => iconComposer = true}>
                {@html svgIcon}
            </button>
            <div class="self-start pt-2 min-w-40">
                <h3 class="title text-gray-800 font-medium uppercase max-w-[160px] truncate text-ellipsis"> {title || '---'} </h3>
                <div class="description text-xs text-gray-700 max-w-[160px] truncate text-ellipsis"> {description || 'No description'} </div>
            </div>
            <div class="mt-14 col-span-2">
                <Toggle id="shared" label="Shared with others" hint="Other users can see this too" enabled={isShared} />
                <Toggle id="searchable" label="Intrinsic search" hint="Enables search within the app" enabled={isSearchable} />
            </div>
        </div>
        <div class="details relative p-4 flex-1 bg-white">
            {#if iconComposer}
                <IconsComposer
                    svg={svgIcon}
                    on:apply={({detail}) => { svgIcon = detail.svg; iconComposer = false; }}
                    on:close={() => iconComposer = false}>
                </IconsComposer>
            {:else}
                <form class="grid grid-cols-[1fr_1fr] gap-2">
                    <div class="col-span-2">
                        <label for="url"> URL </label>
                        <input bind:value={url} bind:this={defaultInput} name="url" id="url" type="text" class="text-sm" />
                    </div>
                    <div>
                        <label for="type"> Type </label>
                        <Autocomplete placeholder="Choose type..." items={types} />
                    </div>
                    <div>
                        <label for="category"> Category </label>
                        <Autocomplete placeholder="Choose category..." items={categories} allowCreate={true} />
                    </div>
                    <div>
                        <label for="title"> Title </label>
                        <input bind:value={title} name="title" id="title" class="text-sm" type="text" maxlength="64"/>
                    </div>
                    <div>
                        <label for="description"> Description </label>
                        <input bind:value={description} name="description" id="description" class="text-sm" type="text" maxlength="64" />
                    </div>

                    <div class="form-buttons col-span-2">
                        <button> Cancel </button>
                        <input name="submit" type="submit" value="Save" class="submit-button" />
                    </div>
                </form>
            {/if}
        </div>
    </div>
</dialog>
