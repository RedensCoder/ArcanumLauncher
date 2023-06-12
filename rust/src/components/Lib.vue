<template>
        <div class="lib">
            <AsideVue />
            <div class="lib__lib">
                <p class="label">Библиотека</p>
                <div class="game">
                    <div class="product" v-for="g in games">
                        <img :src="g.avatar" alt="not img" class="img">
                            <div class="info">
                                <p class="name_game">{{ g.gamename }}</p>
                                <router-link :to="g.gamename" class="play">Играть</router-link>   
                            </div>
                    </div>
                </div>
            </div>
        </div>
</template>

<script setup>
    import AsideVue from './Aside.vue';
    import { onMounted, reactive } from 'vue';
    import axios from 'axios';

    let games = reactive([])

    onMounted(async () => {
        if (localStorage.getItem("token") === null) {
            router.push("/");
        }

        let res = await axios.get("http://127.0.0.1:8080/api/v1/getAllGames")

        games.push(...res.data)
    })
</script>

<style scoped>
    .lib {
        display: flex;
    }

    .label {
        margin-top: 32px;
        padding: 0px 494px;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 64px;
        line-height: 77px;      
        color: #FFFFFF;
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

    .game {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        text-align: center;
    }

    .product {
        margin: 37px 20px 0px 20px;
    }

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
</style>