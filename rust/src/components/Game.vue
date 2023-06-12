<template>
    <div class="flex">
        <Aside />
        <div class="content">
            <p class="game_n">{{ game.gamename }}</p>
            <div class="imgs">
                <div class="game__info">
                    <div class="img">
                        <Trailer :trailer="game.trailer" />
                        <div class="img_dop">
                            <img :src="game.screenshots" alt="" class="dop">
                        </div>
                    </div>
                    <div class="info">
                        <img :src="game.avatar" alt="" class="ava">
                        <p class="desc">{{ game.about }}</p>
                        <p class="author">Автор: <span class="color_wh">{{ game.author }}</span></p>
                        <p class="genre">Жанр: <span class="color_wh">{{ game.genre }}</span></p>
                    </div>
                </div>
                        <div class="buy_game">
                            <button v-if="haveGame" @click="buy" class="buy">Купить</button>
                            <router-link v-else to="/lib" class="buy">В библиотеку</router-link>
                            <div class="prices">
                                <s class="price">540 Руб</s>
                                <p class="orig_price">249 Руб</p>
                            </div>
                        </div>
                  </div>
                  <footer class="footer">
                        <p class="info_game">Об игре</p>
                        <hr class="hr">
                        <p class="desc_foo">{{ game.desc }}</p>
                  </footer>
            </div>        
    </div>
    
</template>

<script setup>
    import Aside from './Aside.vue';
    import Trailer from "./VideoPlayer.vue";
    import {ref, onMounted} from "vue";
    import axios from 'axios';
    import router from '../router';
    import jwtDecode from 'jwt-decode';

    let game = ref({})
    const haveGame = ref(true);

    const buy = async () => {
        await axios.post("http://127.0.0.1:8080/api/v1/addPurchase", {
            username: jwtDecode(localStorage.getItem("token")).user.username,
            password: jwtDecode(localStorage.getItem("token")).user.password,
            gamename: game.value.gamename,
        }, {
            headers: {
                Authorization: "Bearer " + localStorage.getItem("token")
            }
        })

        router.push("/lib")
    }

    onMounted(async () => {
        let res = await axios.get(`http://127.0.0.1:8080/api/v1/getGameByName/${router.currentRoute.value.params.game}`)

        game.value = res.data

        let resLib = await axios.get(`http://127.0.0.1:8080/api/v1/getPurchase/${jwtDecode(localStorage.getItem("token")).user.username}` ,{
            headers: {
                Authorization: "Bearer " + localStorage.getItem("token")
            }}
        )
        console.log(resLib.data);
        resLib.data.forEach(el => {
            
            if (el.game === game.value.gamename) {
                haveGame.value = false;
            }
        })
    })
</script>

<style scoped>
    .flex {
        display: flex;
    }

    .content {
        margin-left: 58px;
    }

    .footer{
        display: grid;
    }

    .hr{
        background: #ffffff;
        border: 5px;           
        height: 5px;
        width: 50%;
        margin-top: 9px;
    }

    .color_wh{
        color: #FFFFFF;
    }

    .game__info {
        display: flex;
    }

    .game_n {
        padding: 30px;
        text-align: center;
        font-family: 'Russo One';
        font-size: 64px;
        line-height: 77px;
        color: #FFFFFF;
    }

    .img{
        display: flex;
        flex-direction: column;
        max-width: 900px;
    }

    .ava{
        width: 95%;
    }
    .desc{
        margin-top: 20px;
        font-family: 'Russo One';
        font-size: 24px;
        line-height: 29px;
        color: #FFFFFF;
    }

    .author{
        margin-top: 14px; 
        padding: 10px 20px 5px 0;
        font-family: 'Russo One';
        font-size: 24px;
        line-height: 29px;
        color: #17B978;
    }

    .genre{
        font-family: 'Russo One';
        font-size: 24px;
        line-height: 29px;
        color: #17B978;
    }

    .img_dop {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr 1fr;
        width: 100%;
        max-width: 900px;
        gap: 12px;
    }

    .dop{
        margin-top: 20px;
        width: 100%;
    }

    .buy_game{
        display: flex;
        margin-top: 30px;
    }

    .buy{
        margin-top: 10px;
        text-decoration: none;
        background: #17B978;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 36px;
        line-height: 43px;
        color: #FFFFFF;
        padding: 8px 100px;
        cursor: pointer;
        border-radius: 10px;  
    }

    .prices {
        display: flex;
        gap: 18px;
    }

    .price {
        margin-left: 175px;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 40px;
        line-height: 48px;
        text-decoration-line: line-through;
        color: #919191;
        margin-top: 8px;
    }

    .orig_price {
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 48px;
        line-height: 58px;
        color: #A6E3E9;
    }

    .info {
        margin-left: 20px;
        max-width: 450px;
    }

    .info_game{
        font-family: 'Russo One';
        font-size: 48px;
        line-height: 58px;
        color: #FFFFFF;
        margin-top: 36px;
    }

    .desc_foo{
        margin-top: 32px;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 32px;
        line-height: 39px;
        color: #FFFFFF;
    }
</style>