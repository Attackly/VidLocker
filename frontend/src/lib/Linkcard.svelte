<script lang="ts">
    import "../app.css";
    import { link } from "$lib/stores/linkstore";
    
    let url = $state("");
    const VALID_URL_LENGTHS = [11, 20, 28, 35, 43];

    let validation = $derived(() => {
        if (url.length === 0) {
            return "";
        }
        if (VALID_URL_LENGTHS.includes(url.length) || url.length > VALID_URL_LENGTHS[VALID_URL_LENGTHS.length - 1]) {
            return "input-success";
        }
        return "input-error";
    });

    $effect(() => {
        if (validation() === "input-success") {
            link.set(url);
        }
    });

</script>

<div
    class="card sm:w-3/4 lg:w-1/2 mt-8 p-4 text-primary rounded-lg overflow-hidden shadow-lg card-bg"
>
    <label class="label flex items-center space-x-4 text-primary">
        <span>Url</span>
        <input
            class="{validation}  border-primary p-2 w-full rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
            type="text"
            placeholder="https://www.youtube.com/watch?v=dQw4w9WgXcQ"
            bind:value={url}
        />
    </label>
</div>
