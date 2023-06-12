<template>
    <div class="flex">
        <Aside />
        <div class="store">
            <p class="label">Магазин</p>
            <div class="games">
                <div class="game" v-for="g in games">
                    <div class="discount__div">
                        <p class="discount">-15%</p>
                    </div>
                    <img :src="g.avatar" alt="not img" class="img">
                    <div class="info">
                        <router-link :to="g.gamename" class="nameGame">{{ g.gamename }}</router-link>
                        <div class="prices">
                            <s class="price">{{ Math.floor(g.price * 1.15) }} Руб</s>
                            <p class="orig_price">{{ g.price }} Руб</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
    import Aside from './Aside.vue';
    import {reactive, onMounted} from "vue";
    import axios from 'axios';

    const games = reactive([])

    onMounted(async () => {
        let res = await axios.get("http://127.0.0.1:8080/api/v1/getAllGames")

        games.push(...res.data)
    })
    
</script>

<style scoped>
    .flex {
        display: flex;
    }

    .store {
        margin-top: 32px;
        width: 100%;
    }

    .label {
        font-family: 'Russo One';
        font-size: 64px;
        line-height: 77px;
        color: #FFFFFF;
        text-align: center;
    }

    .games {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
    }

    .game {
        margin: 50px;
    }

    .discount__div {
        width: 100%;
        text-align: end;
    }

    .discount {
        display: inline-flex;
        background: #FFDE7D;
        border-radius: 10px;
        padding: 7px 21px;
        font-family: 'Russo One';
        font-size: 24px;
        line-height: 29px;
        position: relative;
        top: 25px;
        left: 40px;
    }

    .img {
        width: 100%;
    }

    .info {
        display: flex;
        justify-content: space-between;
        margin-top: 10px;
    }

    .nameGame {
        text-decoration: none;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 24px;
        line-height: 29px;
        color: #FFFFFF;
    }

    .prices {
        display: flex;
    }

    .price {
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 16px;
        line-height: 19px;
        text-decoration-line: line-through;
        color: #919191; 
        margin-top: 5px;
    }

    .orig_price {
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 24px;
        line-height: 28px;
        text-decoration-line: line-through;
        color: #A6E3E9;
        margin-left: 5px;
    }
</style>