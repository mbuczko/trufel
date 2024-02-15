<script>
import Dialog from './Dialog.svelte';
import Toggle from './Toggle.svelte';
import IconsComposer from './IconComposer.svelte';
import Autocomplete from './Autocomplete.svelte';
import { getContext } from 'svelte';
import { Icons } from '$lib/icons.js';

const {getAuthClient} = getContext('auth');

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

/** @type {Dialog} */
let dialog;

export const open = () => {
    dialog.open();
    defaultInput.focus();
}

let types = [
    {
        id: 'app',
        label: 'Application',
        icon: Icons.application
    },
    {
        id: 'bookmark',
        label: 'Bookmark',
        icon: Icons.bookmark

    }
]

const init = () => {
    console.log('initializing');

    url = '';
    title = '';
    description = '';
    isShared = false;
    isSearchable = false;
    svgIcon = '';

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
        if (response.status === 200) {
            return response.json();
        }
        throw response.text();
    }).then(({id, name}) => {
        let category = { id, label: name, icon: Icons.category };
        categories.push(category);
        resolve(category);
    }).catch((e) => Promise
        .resolve(e)
        .then(err => {
            err ||= "Request failed";
            console.error(err);
            reject(err);
        }
    ))
}
</script>

<Dialog bind:this={dialog}
        on:init={init}>
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
</Dialog>
