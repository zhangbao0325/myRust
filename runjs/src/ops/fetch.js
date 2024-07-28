((window) => {
    async function fetch(args) {
        return await Deno.core.opAsync("op_fetch", args);
    }

    window.fetch = fetch
})(this);