<template>
    <div class="flex">
        <Aside />
       <div class="content">
            <div class="profile">
                <img :src="avatar" alt="No Avatar" class="ava">
                <div class="profile__info">
                    <p class="nickname">{{ username }}</p>
                    <div class="lvl">
                        <div class="prog_lvl">
                            <p class="lvl_info">Уровень {{ lvl }}</p>
                        </div>
                    </div>
                    
                    <p class="description">{{ about }}</p>
                </div>
            </div>  
            <router-link class="change" to="/profile/edit">Изменить</router-link>
            <div class="games">
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
    import {ref, onMounted} from "vue";
    import Aside from './Aside.vue';
    import axios from 'axios';
    import jwtDecode from "jwt-decode";

    const username = ref("");
    const lvl = ref(0);
    const about = ref("");
    const avatar = ref("");

    onMounted(async () => {
        if (localStorage.getItem("token") == null) {
            router.push("/");
        }

        let res = await axios.get(`http://127.0.0.1:8080/api/v1/getUserByUsername/${jwtDecode(localStorage.getItem("token")).user.username}`, {
            headers: {
                Authorization: `Bearer ${localStorage.getItem("token")}`
            }
        });

        username.value = res.data.username;
        lvl.value = res.data.lvl;
        about.value = res.data.about;
        avatar.value = res.data.avatar;
    });
</script>

<style scoped>
    .content {
        /* margin-left: 74px; */
        width: 70%;
        margin: 0 auto;
    }

    .flex {
        display: flex;
    }

    .profile {
        display: flex;
        align-items: start;
        width: 100%;
        margin-top: 59px;
    }
    
    .profile__info {
        display: flex;
        width: 100%;
        flex-direction: column;
        margin-left: 35px;
    }

    .ava {
        max-width: 200px;
        max-height: 200px;
        border-radius: 100%;
    }

    .nickname {
        color: #FFFFFF;
        font-family: 'Russo One';
        font-size: 64px;
        line-height: 77px;
        padding: 6px 0;
    }

    .lvl {
        width: 100%;
        min-height: 41px;
        background: #FFFFFF;
        border-radius: 25px;
        margin: 10px auto;
    }

    .prog_lvl {
        width: 30%;
        min-height: 41px;
        background: #FF2E63;
        border-radius: 25px;
    }

    .lvl_info {
        font-family: 'Russo One';
        font-size: 24px;
        line-height: 29px;
        text-align: start;
        padding-top: 6px;
        margin-left: 20px;
        color: #FFF;
    }

    .description {
        margin-top: 11px;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 24px;
        line-height: 28px;
        color: #FFFFFF;
    }

    .change {
        margin-top: 38px;
        text-decoration: none;
        background: #17B978;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 32px;
        line-height: 100px;
        color: #FFFFFF;
        padding: 6px 140px;
        cursor: pointer;
        border-radius: 15px;  
    }

    .games {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        text-align: center;
    }

    .product {
        margin: 37px 50px 0px 0px;
        width: 100%;
    }

    .img {
        max-width: 100%;
    }

    .info {
        display: flex;
        justify-content: space-between;
        margin-top: 10px;
    }

    .name_game {
        text-decoration: none;
        font-family: 'Russo One';
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