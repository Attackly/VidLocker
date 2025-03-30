<script lang="ts">
    import "../app.css";
    import { link } from "$lib/stores/linkstore";
    let validation = "";
    let url = "";

    function url_verify() {
        if (url.length == 11) {
            validation = "input-success";
            link.set(url);
            console.log("Viewkey:", $link);
        } // full link with HTTPS or without HTTPS
        else if (url.length == 43 || url.length == 35) {
            validation = "input-success";
            link.set(url);
            console.log("Viewkey:", $link);
        }
        // Shortend Youtube link
        else if (url.length == 28 || url.length == 20) {
            validation = "input-success";
            link.set(url);
            console.log("Viewkey:", $link);
        }
        // Between Viewkey and link
        else if (url.length < 23 && url.length > 11) {
            validation = "input-error";
            return;
        }
        // Not even a viewkey
        else if (url.length < 11) {
            validation = "input-error";
            return;
        }
    }
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
            on:input={url_verify}
        />
    </label>
</div>
