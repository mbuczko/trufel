<script>
import Banner from "./Banner.svelte";

/** @type {String} - default "error" icon */
const error = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><title>alert-circle-outline</title><path d="M11,15H13V17H11V15M11,7H13V13H11V7M12,2C6.47,2 2,6.5 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2M12,20A8,8 0 0,1 4,12A8,8 0 0,1 12,4A8,8 0 0,1 20,12A8,8 0 0,1 12,20Z" /></svg>';

/** @type {String} - SVG icon definition */
let svg = '';

/** @type {number | undefined} */
let timer;

/** @type {boolean | undefined} */
let isCorrectSvg;

/**
 * Debounces textarea input events.
 * 
 * @param {KeyboardEvent & { currentTarget: EventTarget & HTMLTextAreaElement }} event - a keyboard event to debounce
 */
const debounce = (event) => {
    let text = (event.target && event.currentTarget.value) || '';
    clearTimeout(timer);
    timer = setTimeout(() => {
        svg = text;
    }, 500);
}

/**
 * Parses SVG to be sure it's a valid svg+xml image definition.
 * 
 * @param {String | null} svg - SVG definition to parse.
 * @returns validated svg as a text or null if validation failed.
 */
const intoSvg = (svg) => {
    let parsed = svg && new DOMParser().parseFromString(svg, "image/svg+xml");
    if (parsed) {
        let child = parsed.firstChild;
        if (!child || child.nodeName !== 'svg') {
            isCorrectSvg = false;
            return;
        } else {
            isCorrectSvg = true;
            /** @ts-ignore */
            return child.outerHTML;
        }
    }
}
</script>

<div class="icon-composer">
    <Banner style="rounded-tr-lg" closable={false} />
    <div class="w-full flex gap-4 mt-10">
        <div class="flex-1">
            <textarea on:keyup={(e) => debounce(e)}
                      class="h-full p-1"
                      placeholder="Your SVG icon as 'image/svg+xml' string..."/>
        </div>
        <div class="icon-preview w-full h-full">
            {@html (intoSvg(svg) || error)}
        </div>
    </div>
    <div class="form-buttons">
        <button>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><title>arrow-left</title><path d="M20,11V13H8L13.5,18.5L12.08,19.92L4.16,12L12.08,4.08L13.5,5.5L8,11H20Z" /></svg>
            Back to details
        </button>
        <input name="submit" type="submit" value={isCorrectSvg ? 'Use icon' : 'Invalid icon' } class="submit-button" disabled={!isCorrectSvg}/>
    </div>
</div>

<style>
 .icon-composer {
     --light-grey: 0deg 0% 94%;
 }
 .icon-composer textarea {
     border: var(--dialog-border);
     border-radius: var(--dialog-button-radius);
     min-width: 330px;
     max-width: 380px;
     resize: horizontal;
     font-size: 0.7em;
     color: gray;
 }
 .icon-composer .form-buttons {
     position: absolute;
     bottom: 12px;
     right: 0;
     margin: 8px 15px;
     width: 100%;
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
