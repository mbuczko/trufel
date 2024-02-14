<script>
import Toggle from './Toggle.svelte';
import IconsComposer from './IconComposer.svelte';
import Autocomplete from './Autocomplete.svelte';
import { getContext } from 'svelte';

const {getAuthClient} = getContext('auth');

const categorySvgIcon = '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><title>checkbox-intermediate-variant</title><path d="M19 3H5C3.9 3 3 3.9 3 5V19C3 20.1 3.9 21 5 21H19C20.1 21 21 20.1 21 19V5C21 3.9 20.1 3 19 3M19 19H5V5H19V19M7 17V7H17" /></svg>';

/** @type {HTMLDialogElement} */
let ref;

/** @type {HTMLElement} */
let defaultInput;

/** @type {boolean} */
let iconView = false;

/** @type {string} */
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

/** @type {AutocompleteItem[]} */
let categories = []

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
    // reset values to default ones each time dialog re-opens
    url = '';
    title = '';
    description = '';
    isShared = false;
    isSearchable = false;
    svgIcon = '';

    ref.showModal();
    defaultInput.focus()
}

/**
 * Called on new category being added.
 * @param {{detail: CreateCategoryEvent}} event
 */
const onNewCategory = ({detail: {text, resolve, reject}}) => {
    fetch('http://localhost:3030/categories', {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
            'Authorization': 'Bearer ' + getAuthClient().token
        },
        body: JSON.stringify({name: text})
    }).then((response) => {
        if (response.status !== 200) {
            throw "Request failed"
        }
        return response.json();
    }).then(({id, name}) => {
        let category = { id, label: name, icon: categorySvgIcon };

        categories.push(category);
        resolve(category);
    }).catch((e) => {
        console.error(e);
        reject(e);
    })
}
</script>

<dialog id="dialog"
        class="create-dialog w-full min-w-[540px] max-w-screen-md max-h-[270px] bg-slate-500 shadow-md border-1 rounded-lg"
        bind:this={ref}>
    <div class="flex">
        <div class="sidebar p-2 grid grid-cols-[64px_1fr] justify-items-start">
            <button
                class="self-start icon w-14 h-14 mb-4 border-2 border-slate-700 opacity-80 rounded-lg shadow-md active:shadow-none hover:opacity-100"
                on:click={() => iconView = true}>
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
            {#if iconView}
                <IconsComposer
                    svg={svgIcon}
                    on:apply={({detail}) => { svgIcon = detail.svg; iconView = false; }}
                    on:close={() => iconView = false}>
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
                        <Autocomplete placeholder="Choose category..."
                                      items={categories}
                                      allowCreate={true}
                                      on:create={onNewCategory} />
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
