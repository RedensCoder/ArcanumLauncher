<template>
    <div class="flex">
        <Aside />
        <div class="content">
            <div class="profile">
                <div class="change_ava">
                    <img :src="avatar" alt="no ava" class="ava">
                    <input type="file" id="button" class="input_hidden">
                    <button class="change" for="id" @click="chooseFiles()">Загрузить</button>
                </div>
                <div class="change_main">
                    <p class="nickname">{{ username }}</p>
                    <input type="text" placeholder="Имя пользователя" required />
                    <input type="text" placeholder="Пароль" required />
                    <input type="text" placeholder="О себе" required />
                    <router-link to="/signin" class="change">Изменить</router-link>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
    import { ref, onMounted } from 'vue';
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

    const chooseFiles = async () => {
        let fileInputElement = this.$refs.file;
        fileInputElement.click();        
    }
    
</script>

<style scoped>
    .flex {
        display: flex;
    }

    .input_hidden{
        display: none;
    }

    .content {
        width: 100%;
        height: 100%;
        padding: 76px 0;
    }

    .change_ava{
        width: 100%;
        display:flex;
        flex-direction: column;
        justify-content:center;
        align-items:center;
    }

    .ava {
        max-width: 200px;
        max-height: 200px;
        border-radius: 100%;
    }

    .change_main{
        margin-top: 150px;
        width: 100%;
        display:flex;
        flex-direction: column;
        justify-content:center;
        align-items:center;
    }

    .profile input {
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 24px;
        line-height: 29px;
        background: #FFFFFF;
        border-radius: 10px;
        padding: 6px 0px;
        padding-left: 9px;
        margin: 10px;
        width: 28%;
    }

    input::placeholder {
        color: #222831;
    }

    .nickname {
        /* padding-bottom: 49px; */
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 48px;
        line-height: 58px;
        color: #FFFFFF;
    }

    .change {
        margin-top: 320px;
        padding: 7px 132px;
        position: absolute;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 32px;
        line-height: 39px;
        text-align: center;
        color: #FFFFFF;
        background: #17B978;
        border-radius: 15px;
    }

</style>