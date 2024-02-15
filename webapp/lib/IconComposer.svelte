<script>
import { createEventDispatcher } from "svelte";
import { Icons } from "$lib/icons.js";
import Banner from "./Banner.svelte";

const dispatch = createEventDispatcher();

/** @type {string} - SVG icon definition */
export let svg = '';

/** @type {number | undefined} */
let timer;

/** @type {boolean | undefined} */
let isValidSvg;

/**
 * Debounces textarea input events.
 *
 * @param {KeyboardEvent & { currentTarget: EventTarget & HTMLTextAreaElement }} event - a keyboard event to debounce
 */
const debounce = (event) => {
    let text = (event.target && event.currentTarget.value) || '';
    clearTimeout(timer);
    timer = setTimeout(() => { svg = text }, 500);
}

/**
 * Parses SVG to be sure it's a valid svg+xml image definition.
 *
 * @param {string | null} svg - SVG definition to parse.
 * @returns validated svg as a text or null if validation failed.
 */
const intoSvg = (svg) => {
    let parsed = svg && new DOMParser().parseFromString(svg, "image/svg+xml");
    if (parsed) {
        let child = parsed.firstChild;
        if (child && child.nodeName === 'svg') {
            isValidSvg = true;
            /** @ts-ignore */
            return child.outerHTML;
        }
    }
    isValidSvg = false;
}
</script>

<div class="icon-composer">
    <Banner style="rounded-tr-lg" closable={false}>
        <span slot="title">
            <svg class="w-4 h-4 mr-1 inline-block" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <g fill="none" stroke="none" data-darkreader-inline-stroke="" style="--darkreader-inline-stroke: none;">
                    <path d="M10.1893 8.12241C9.48048 8.50807 9.66948 9.5633 10.4691 9.68456L13.5119 10.0862C13.7557 10.1231 13.7595 10.6536 13.7968 10.8949L14.2545 13.5486C14.377 14.3395 15.4432 14.5267 15.8333 13.8259L17.1207 11.3647C17.309 11.0046 17.702 10.7956 18.1018 10.8845C18.8753 11.1023 19.6663 11.3643 20.3456 11.4084C21.0894 11.4567 21.529 10.5994 21.0501 10.0342C20.6005 9.50359 20.0352 8.75764 19.4669 8.06623C19.2213 7.76746 19.1292 7.3633 19.2863 7.00567L20.1779 4.92643C20.4794 4.23099 19.7551 3.52167 19.0523 3.82031L17.1037 4.83372C16.7404 4.99461 16.3154 4.92545 16.0217 4.65969C15.3919 4.08975 14.6059 3.39451 14.0737 2.95304C13.5028 2.47955 12.6367 2.91341 12.6845 3.64886C12.7276 4.31093 13.0055 5.20996 13.1773 5.98734C13.2677 6.3964 13.041 6.79542 12.658 6.97364L10.1893 8.12241Z" stroke="currentColor" stroke-width="1.5" data-darkreader-inline-stroke="" style="--darkreader-inline-stroke: currentColor;"></path>
                    <path d="M12.1575 9.90759L3.19359 18.8714C2.63313 19.3991 2.61799 20.2851 3.16011 20.8317C3.70733 21.3834 4.60355 21.3694 5.13325 20.8008L13.9787 11.9552" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" data-darkreader-inline-stroke="" style="--darkreader-inline-stroke: currentColor;"></path><path d="M5 6.25V3.75M3.75 5H6.25" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" data-darkreader-inline-stroke="" style="--darkreader-inline-stroke: currentColor;"></path><path d="M18 20.25V17.75M16.75 19H19.25" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" data-darkreader-inline-stroke="" style="--darkreader-inline-stroke: currentColor;"></path>
                </g>
            </svg>
            <strong class="font-semibold"> SVG icon </strong>
        </span>
        <span slot="description">
            Head over to <a href="https://pictogrammers.com/library/mdi" target="_blank">pictogrammers.com</a> for some inspiration!
        </span>
    </Banner>
    <div class="w-full flex gap-3 my-11">
        <div class="flex-1">
            <textarea value={svg}
                      class="h-full p-1"
                      placeholder="Your SVG icon as 'image/svg+xml' string..."
                      on:keyup={(e) => debounce(e)}/>
        </div>
        <div class="icon-preview w-full h-full">
            {@html (intoSvg(svg) || Icons.invalid)}
        </div>
    </div>
    <div class="form-buttons">
        <button on:click={() => dispatch('close')}>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><title>arrow-left</title><path d="M20,11V13H8L13.5,18.5L12.08,19.92L4.16,12L12.08,4.08L13.5,5.5L8,11H20Z" /></svg>
            Back to details
        </button>
        <input
            name="submit"
            type="submit"
            value={isValidSvg ? 'Apply icon' : 'No valid icon' }
            class="submit-button"
            disabled={!isValidSvg}
            on:click={() => dispatch('apply', {svg: svg})}/>
    </div>
</div>

<style>
 .icon-composer {
     --light-grey: 0deg 0% 94%;
 }
 .icon-composer textarea {
     border: var(--dialog-border);
     border-radius: var(--dialog-button-radius);
     min-width: 350px;
     max-width: 380px;
     resize: horizontal;
     font-size: 0.7em;
     color: gray;
 }
 .icon-composer .form-buttons {
     position: absolute;
     bottom: 8px;
     right: 0;
     margin: 8px 15px;
     width: 100%;
 }
 .icon-composer input[type=submit] {
     min-width: 8.5em;
 }
 .icon-preview {
     background-image: repeating-conic-gradient(hsl(var(--light-grey)) 0 25%,transparent 0 50%);
     background-size: 20px 20px;
     border-radius: var(--dialog-button-radius);
     outline: var(--dialog-border);
     min-width: 60px;
     position: relative;
 }
</style>
