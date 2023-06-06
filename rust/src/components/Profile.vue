<template>
    <div class="flex">
        <Aside />
       <div class="content">
            <div class="profile">
                <img src="https://i.ytimg.com/vi/CxLaYXINBQY/maxresdefault.jpg" alt="No Avatar" class="ava">
                <p class="nickname">Ded</p>
                <div class="lvl">
                    <div class="prog_lvl"></div>
                </div>
                <p class="description">Описание Описание Описание Описание Описание Описание Описание Описание Описание Описание Описание Описание Описание Описание Описание Описание Описание Описание Описание Описание </p>
            </div>
                <router-link class="change" to="/">Изменить</router-link>
                <div class="game">
                <div class="product">
                    <img src="https://mykaleidoscope.ru/x/uploads/posts/2022-09/1663362480_16-mykaleidoscope-ru-p-gon-v-gneve-vkontakte-17.jpg" alt="not img" class="img">
                        <div class="info">
                            <p class="name_game">NameGame</p>
                            <router-link to="/" class="time_game">1234 ч</router-link>   
                        </div>
                </div>
            </div>
       </div>
    </div>
</template>

<script setup>
    import Aside from './Aside.vue';
</script>

<style scoped>
    .flex {
        display: flex;
    }

    .profile {
        display: flex;
        align-items: center;
        margin-top: 59px;
        margin-left: 74px;
        justify-content:center;
    }

    .ava {
        width: 200px;
        height: 200px;
        border-radius: 100%;
    }

    .nickname {
        position: relative;
        left: 30px;
        color: #FFFFFF;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 64px;
        line-height: 77px;
        margin-bottom: 130px;
    }

    .lvl {
        right: 80px;
        position: absolute;
        margin-bottom: 130px;
        padding: 22px 600px;
        width: auto;  
        background: #FFFFFF;
        border-radius: 25px;
    }

    .prog_lvl {
        position: absolute;
        padding: 11px 300px;
        background: #FF2E63;
        border-radius: 25px;
    }

    .description {
        position: relative;
        right: 92px;
        padding: 50px 0px 0px 0px;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 24px;
        line-height: 28px;
        color: #FFFFFF;
    }

    .change{
        margin-left: 80px;
        text-decoration: none;
        background: #17B978;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 32px;
        line-height: 100px;
        color: #FFFFFF;
        padding: 6px 80px;
        cursor: pointer;
        border-radius: 15px;  
    }

    .game {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        text-align: center;
    }

    .product {
        margin: 37px 0px 0px 20px;
    }

    .img {
        width: 433px;
    }

    .info {
        padding: 0 60px;
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

    .time_game {
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