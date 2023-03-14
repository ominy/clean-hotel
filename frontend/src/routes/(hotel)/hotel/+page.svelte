<script lang="ts">
    import "../../../app.css";

    let newCleaner = ``;

    function addCleaner() {
        hotel.cleaners = [...hotel.cleaners, newCleaner];
        newCleaner = ``;
    }

    let newFloor = ``;

    function addFloor() {
        hotel.floors.set(+newFloor, []);
        newFloor = ``;
    }

    let newRoom = ``;

    function addRoom() {
        hotel.floors.set(+newRoom[0], [
            ...hotel.floors.get(+newRoom[0]),
            [newRoom, true],
        ]);
        newRoom = ``;
    }

    let hotel = {
        name: "TestHotel",
        floors: new Map<number, Array<Map<string, boolean>>>([
            [
                1,
                [
                    ["101", true],
                    ["102", false],
                    ["103", false],
                    ["104", true],
                ],
            ],
            [
                2,
                [
                    ["201", true],
                    ["202", false],
                    ["203", false],
                    ["204", true],
                    ["205", false],
                    ["206", true],
                ],
            ],
        ]),
        cleaners: ["John", "Lisa", "Mattew"],
    };
</script>

<header>
    <nav class="flex justify-between p-0 m-0 items-baseline">
        <h1 class="title text-3xl p-0 m-0">Clean hotel</h1>
        <!-- <a href="/">Home</a> -->
    </nav>
</header>

<input type="text" placeholder="Add Cleaner" bind:value={newCleaner} />
<button on:click={addCleaner}>Add Cleaner</button>
<input type="text" placeholder="Add Floor" bind:value={newFloor} />
<button on:click={addFloor}>Add Floor</button>
<input type="text" placeholder="Add Room" bind:value={newRoom} />
<button on:click={addRoom}>Add Room</button>

<main>
    {#each [...hotel.floors] as [floor, _]}
        <h2>Floor {floor}</h2>
        <ul>
            {#each [hotel.floors.get(floor)] as rooms}
                {#each [...rooms] as [room, status]}
                    <li style:background-color={status ? "green" : "red"}>
                        {room}
                        <select name="room" id={room}>
                            {#each hotel.cleaners as cleaner}
                                <!-- content here -->
                                <option value={cleaner}>{cleaner}</option>
                            {/each}
                        </select>
                    </li>
                {/each}
            {/each}
        </ul>
    {/each}
</main>

<style lang="postcss">
    nav {
        column-gap: 2ch;
    }

    .title {
        flex-grow: 2;
    }

    ul {
        display: flex;
        justify-content: space-around;
        flex-grow: 2;
        flex-wrap: wrap;
        margin: 3em;
    }

    main {
        margin-top: 3em;
    }
</style>
