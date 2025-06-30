<script>
 let {
        open = false,
        onConfirm = () => {},
        onCancel = () => {},
        message = "Are you sure you want to delete this item?"
    } = $props();

    const confirm = () => onConfirm();
    const cancel = () => onCancel();
</script>
{#if open}
    <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
        <div class="bg-white dark:bg-gray-800 rounded-2xl p-6 max-w-sm w-full shadow-xl relative">
            <p class="text-lg text-gray-800 dark:text-gray-200 mb-6">{message}</p>
            <div class="flex justify-end gap-4">
                <button
                        onclick={cancel}
                        class="bg-gray-300 dark:bg-gray-700 text-black dark:text-white px-4 py-2 rounded-xl hover:bg-gray-400 dark:hover:bg-gray-600"
                >
                    Cancel
                </button>
                <button
                        onclick={confirm}
                        class="bg-red-600 text-white px-4 py-2 rounded-xl hover:bg-red-700"
                >
                    Delete
                </button>
            </div>
        </div>
    </div>
{/if}