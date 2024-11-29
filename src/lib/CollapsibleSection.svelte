<script>
    // based on suggestions from:
    // Inclusive Components by Heydon Pickering https://inclusive-components.design/collapsible-sections/
    // online available: https://svelte.dev/playground/a5f4d395b15a44d48a6b2239ef705fc4?version=5.2.10
    export let headerText;

    let expanded = false
</script>

<div class="collapsible">
    <h3>
        <button aria-expanded={expanded} on:click={() => expanded = !expanded}>
            <svg viewBox="0 0 20 20" fill="none" class="arrow">
                <path d="M5 7L10 12L15 7" stroke="black" stroke-width="2"/>
            </svg>
            {headerText}
        </button>
    </h3>
    
    <div class='contents' hidden={!expanded}>
        <slot></slot>
    </div>
</div>

<style>
    .collapsible {
        border-bottom: 1px solid var(--gray-light, #eee);
        margin: 1em; 
    }
	
	h3 {
		margin: 0;
	}
	
    button {
        background-color: var(--bg-color);
        color: var(--main-color);
        display: flex;
        justify-content: space-between;
        width: 100%;
        border: none;
        margin: 0;
        padding: 1em 0.5em;
    }

    button[aria-expanded="true"] {
        border-bottom: 1px solid var(--gray-light, #eee);
    }

    button[aria-expanded="true"] .vert {
        display: none;
    }

    button:focus svg{
        outline: 2px solid;
    }

    button [aria-expanded="true"] rect {
        fill: currentColor;
    }

    svg {
        height: 2em;
        width: 2em;
    }
</style>