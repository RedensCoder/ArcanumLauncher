<template>
        <div class="lib">
            <AsideVue />
            <div class="lib__lib">
                <p class="label">Библиотека</p>
                <div class="game">
                    <div class="product" v-for="p in purchase">
                        <lib_game :game="p" />
                    </div>
                </div>
            </div>
        </div>
</template>

<script setup>
    import AsideVue from './Aside.vue';
    import lib_game from './lib_game.vue';
    import { onMounted, reactive } from 'vue';
    import jwtDecode from 'jwt-decode';
    import axios from 'axios';

    let purchase = reactive([])

    onMounted(async () => {
        if (localStorage.getItem("token") === null) {
            router.push("/");
        }

        let res = await axios.get(`http://127.0.0.1:8080/api/v1/getPurchase/${jwtDecode(localStorage.getItem("token")).user.username}`,{
            headers: {
                Authorization: "Bearer " + localStorage.getItem("token")
            }
        })

        purchase.push(...res.data)
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

    .game {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        text-align: center;
    }

    .product {
        margin: 37px 20px 0px 20px;
    }
</style>