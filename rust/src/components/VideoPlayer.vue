<template>
    <div class="trailer">
        <video ref="video" width="900" @timeupdate="updateProgress" @ended="play = true" src="https://media.kg-portal.ru/games/d/deadbydaylight/trailers/deadbydaylight_addontrailerendtransmission_1920.mp4"></video>
        <div class="controls">
            <svg @click="pPlay" v-if="play" class="play" xmlns="http://www.w3.org/2000/svg" height="25" viewBox="0 0 384 512"><path fill="#FFFFFF" d="M73 39c-14.8-9.1-33.4-9.4-48.5-.9S0 62.6 0 80V432c0 17.4 9.4 33.4 24.5 41.9s33.7 8.1 48.5-.9L361 297c14.3-8.7 23-24.2 23-41s-8.7-32.2-23-41L73 39z"/></svg>
            <svg @click="pPause" v-else class="play" xmlns="http://www.w3.org/2000/svg" height="25" viewBox="0 0 320 512"><path fill="#FFFFFF" d="M48 64C21.5 64 0 85.5 0 112V400c0 26.5 21.5 48 48 48H80c26.5 0 48-21.5 48-48V112c0-26.5-21.5-48-48-48H48zm192 0c-26.5 0-48 21.5-48 48V400c0 26.5 21.5 48 48 48h32c26.5 0 48-21.5 48-48V112c0-26.5-21.5-48-48-48H240z"/></svg>
            <div @click="setProgress" ref="progress" class="progress">
                <div ref="bar" class="progress__bar"></div>
            </div>
        </div>
    </div>
</template>

<script setup>
    import { ref } from 'vue';

    const play = ref(true);

    const video = ref(null);
    const progress = ref(null);
    const bar = ref(null);

    const pPlay = () => {
        play.value = false;
        video.value.play();
    }

    const pPause = () => {
        play.value = true;
        video.value.pause();
    }

    const updateProgress = (e) => {
        const {duration, currentTime} = e.target;
        bar.value.style.width = (currentTime / duration) * 100 + "%";
    }

    const setProgress = (e) => {
        const width = progress.value.clientWidth;
        const clickX = e.offsetX;
        const dur = video.value.duration;

        video.value.currentTime = (clickX / width) * dur;
    }
</script>

<style scoped>
    .trailer {
        max-width: 900px;
        width: 100%;
    }

    .controls {
        background: #393E46;
        display: flex;

    }

    .play {
        max-height: 25px;
        min-width: 25px;
        cursor: pointer;
        padding: 8px 12px;
    }

    .progress {
        width: 100%;
        max-height: 10px;
        background: #FFF;
        border-radius: 10px;
        margin: 16px 12px 16px 0;
        cursor: pointer;
    }

    .progress__bar {
        width: 0;
        max-height: 10px;
        height: 10px;
        background: #FF2E63;
        border-radius: 10px;
    }
</style>