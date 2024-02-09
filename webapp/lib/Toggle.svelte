<script>

/** @type {boolean} */
export let enabled = false;

/** @type {string} */
export let id;

/** @type {string} */
export let label;

/** @type {String | undefined} */
export let hint;

</script>
<div class="flex items-start space-x-2">
    <input id={id} type="checkbox" name="switch" class="hidden">
    <button type="button"
            class="relative inline-flex h-6 py-0.5 ml-4 focus:outline-slate-300 rounded-full w-10 {enabled ? 'bg-slate-700' : 'bg-neutral-200'}"
            on:click="{() => enabled = !enabled }">
        <span class="{enabled ? 'translate-x-[18px]' : 'translate-x-[0.5px]'} w-5 h-5 duration-200 ease-in-out bg-white rounded-full shadow-md translate-x-0.5"></span>
    </button>
    <label for={id}
           role="presentation"
           class="text-blue-600 text-sm select-none text-gray-400"
           on:click="{() => enabled = !enabled}">
        {label}
        <span class="hint text-slate-700"> {hint} </span>
    </label>
</div>

<style>
 .hint {
     display: block;
     font-size: 0.8em;
     padding-bottom: 6px;
 }
</style>
