<template>
    <div class="product">
        <img :src="'http://127.0.0.1:8080/api/v1/gameAvatar/' + props.game.game" alt="not img" class="img">
            <div class="info">
                <p class="name_game">{{ props.game.game }}</p>
                <button @click="play(props.game.game)" v-if="hasDonwload" class="play">Играть</button>   
                <button @click="download(props.game.game)" v-else class="play">Скачать</button>
            </div>
    </div>
</template>

<script setup>
    import {ref, defineProps, onMounted} from 'vue';
    import jwtDecode from 'jwt-decode';
    import { invoke } from '@tauri-apps/api';

    let hasDonwload = ref(false);

    let props = defineProps(["game"])

    let download = async () => {
        let data = `${jwtDecode(localStorage.getItem('token')).user.username}:${localStorage.getItem('token')}`

        await invoke("download", {name: props.game.game, data: data})
    }

    let play = async () => {
        invoke("play", {game: props.game.game})
    }

    onMounted(async () => {
        let res = await invoke("check_download", {game: props.game.game})

        res.forEach(el => {
            if (props.game.game === el.replace(".exe", "")) {
                hasDonwload.value = true
            } 
        })
        
    })
</script>

<style scoped>
 .img {
        width: 100%;
    }

    .info {
        display: flex;
        justify-content: space-between;
        margin-top: 10px;
    }

    .name_game {
        text-decoration: none;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 24px;
        line-height: 29px;
        color: #FFFFFF;
    }
    
    .lib__lib {
        text-align: center;
    }

    .play {
        text-decoration: none;
        background: #17B978;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 24px;
        line-height: 29px;
        color: #FFFFFF;
        padding: 2px 36px;
        cursor: pointer;
        border-radius: 10px;        
    }
</style>