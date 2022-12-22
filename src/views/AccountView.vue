<script setup lang="ts">
    const floors = [{
        number: 1,
        rooms: [{ number: 101, state: 'clean' },{ number: 102, state: 'clean' },{ number: 103, state: 'dirty' },{ number: 104, state: 'dirty' }]
    }, {
        number: 2,
        rooms: [{ number: 201, state: 'mid' },{ number: 202, state: 'clean' },{ number: 203, state: 'mid' },{ number: 204, state: 'dirty' }]
    }
    ];

</script>

<template>
    <main>
        <h1>Big hotel</h1>
        <ul class="floors">
            <li class="floor" v-for="floor in floors">
                <h2>Floor {{ floor.number }}</h2>
                <ul class="rooms">
                    <li class="room" v-for="room in floor.rooms">
                        <div v-if="room.state == 'clean'" style="background-color: #0f0;" id="rm">{{ room.number }}</div>
                        <div v-else-if="room.state == 'dirty'" style="background-color: red;" id="rm">{{ room.number }}</div>
                        <div v-else="room.state == 'mid'" style="background-color: yellow;" id="rm">{{ room.number }}</div>
                    </li>
                </ul>
                <hr>
            </li>
        </ul>
    </main>
</template>

<style scoped>
    main > h1{
        text-align: center;
    }

    ul {
        list-style: none;
    }
    .floor {
        margin: 50px 0;
    }

    .room {
        display: inline;
        margin: 10px;
    }
    #rm {
        background-color: #0f0;
        color: #333;
        display: inline-block;
        min-width: 200px;
        margin: 10px;
        text-align: center;
    }
</style>

<script lang='ts'>
    const room = document.getElementById('rm');
</script>
