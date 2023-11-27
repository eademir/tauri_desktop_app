<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    let data = 0;
    let answer = "";
    let number = "123";
    invoke("get_num_cpu").then((res: number) => {
        data = res;
    });

    const findIndexesOfTheNumber = async () => {
        answer = "";
        answer = await invoke("find_in_pi", { number });
    };

    const findIndexesOfTheNumberWithThreads = async () => {
        answer = "";
        answer = await invoke("find_with_threads", { number });
    };
</script>

<main>
    <h1>
        {data}
    </h1>
    <h2>
        {answer}
    </h2>
    <input bind:value={number} />
    <button on:click={() => findIndexesOfTheNumber()}>Search</button>
    <button on:click={() => findIndexesOfTheNumberWithThreads()}
        >Search with threads</button
    >
</main>

<style>
</style>
