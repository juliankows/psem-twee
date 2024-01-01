<script setup>
import { onMounted, ref, watch } from 'vue'

import dashjs from 'dashjs'

let file = ref("");
let buttons = ref([]);
let showButtons = ref(false);
let history = ref([]);

let first_interaction = true;

let player = dashjs.MediaPlayer().create();
let config = await getConfig();

watch(file, (f) => {
	if (!f) return;
	setTimeout(() => {
		buttons.value = config[f].links;
	}, 510) // wait for the animation
})

async function getConfig() {
	let res = await fetch("/config.json");
	return await res.json();
}

function select(target) {
	history.value.push(target)
	file.value = target;
	player.initialize(document.getElementById("player"), `/files/${file.value}/manifest.mpd`, false);
	player.on("playbackTimeUpdated", (ev) => {
		let btn_dur = player.duration() > 5 ? 5 : player.duration() * .1
		showButtons.value = ev.timeToEnd < btn_dur
		//if (ev.timeToEnd <= 0.5) {
		//	setTimeout(() => {
		//		let rnd = [...buttons.value].sort(() => Math.random() > 0.5 ? -1 : 1)[0];
		//		select(rnd)
		//	}, 500)
		//}
	})
	if (!first_interaction) // don't play on first page load
		player.play();
	first_interaction = false
}

function back() {
	if (history.value.length < 2) return;
	history.value.pop()
	select(history.value.pop())
}


onMounted(() => { select("Intro") })
</script>

<template>
	<video id="player" controls>
	</video>
	<div class="buttons" :class="{ show: showButtons }">
		<button class="back" @click="back">Zurück</button>
		<button class="btn" v-for="b of buttons" @click="select(b)">{{ b }}</button>
	</div>
</template>

<style scoped>
#player {
	width: 100vw;
	height: 100vh;
}

.buttons {
	position: fixed;
	width: 100vw;
	bottom: 0rem;
	display: grid;
	grid-template-columns: repeat(auto-fit, minmax(25vw, 1fr));
	grid-auto-flow: column;
	opacity: 0;
	transition: all 0.5s ease;
	margin: 0;
	padding: 0;
}

.buttons:not(.show) {
	pointer-events: none;
}

.back {
	position: fixed;
	top: 1rem;
	left: 1rem;
	opacity: 0.15;
}

.back:hover {
	opacity: 1;
}

.buttons.show {
	opacity: 1;
	bottom: 4rem;
}

.btn {
	background-color: dodgerblue;
	padding: 1rem;
	border: none;
	margin: 1rem;
	border-radius: .5rem;
}
</style>
