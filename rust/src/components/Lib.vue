<template>
        <div class="lib">
            <AsideVue />
            <div class="lib__lib">
                <p class="label">Библиотека</p>
                <div class="game">
                    <div class="product">
                        <img src="https://mykaleidoscope.ru/x/uploads/posts/2022-09/1663362480_16-mykaleidoscope-ru-p-gon-v-gneve-vkontakte-17.jpg" alt="not img" class="img">
                            <div class="info">
                                <p class="name_game">NameGame</p>
                                <router-link to="/" class="play">Играть</router-link>   
                            </div>
                    </div>
                </div>
            </div>
        </div>
</template>

<script setup>
    import AsideVue from './Aside.vue';
    import { onMounted } from 'vue';

    onMounted(() => {
        if (localStorage.getItem("token") === null) {
            router.push("/");
        }
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