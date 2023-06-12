<template>
        <aside class="aside"> 
            <div class="flex">
                <div @click="router.push('/profile')" class="profile">
                        <img :src="avatar" alt="no avatar" class="avatar">
                        <p class="nickname">{{ username }}</p>
                        <p class="lvl">{{ lvl }}</p>
                </div>
                <div class="navbar">
                        <div class="nb_lib">
                            <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="44" viewBox="0 0 640 512"><path fill="#FFFFFF" d="M192 64C86 64 0 150 0 256S86 448 192 448H448c106 0 192-86 192-192s-86-192-192-192H192zM496 168a40 40 0 1 1 0 80 40 40 0 1 1 0-80zM392 304a40 40 0 1 1 80 0 40 40 0 1 1 -80 0zM168 200c0-13.3 10.7-24 24-24s24 10.7 24 24v32h32c13.3 0 24 10.7 24 24s-10.7 24-24 24H216v32c0 13.3-10.7 24-24 24s-24-10.7-24-24V280H136c-13.3 0-24-10.7-24-24s10.7-24 24-24h32V200z"/></svg>
                            <routerLink to="/lib" class="nb_to">Библиотека</routerLink>
                        </div>
                        <div class="nb_lib">
                            <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="43" viewBox="0 0 576 512"><path fill="#FFFFFF" d="M547.6 103.8L490.3 13.1C485.2 5 476.1 0 466.4 0H109.6C99.9 0 90.8 5 85.7 13.1L28.3 103.8c-29.6 46.8-3.4 111.9 51.9 119.4c4 .5 8.1 .8 12.1 .8c26.1 0 49.3-11.4 65.2-29c15.9 17.6 39.1 29 65.2 29c26.1 0 49.3-11.4 65.2-29c15.9 17.6 39.1 29 65.2 29c26.2 0 49.3-11.4 65.2-29c16 17.6 39.1 29 65.2 29c4.1 0 8.1-.3 12.1-.8c55.5-7.4 81.8-72.5 52.1-119.4zM499.7 254.9l-.1 0c-5.3 .7-10.7 1.1-16.2 1.1c-12.4 0-24.3-1.9-35.4-5.3V384H128V250.6c-11.2 3.5-23.2 5.4-35.6 5.4c-5.5 0-11-.4-16.3-1.1l-.1 0c-4.1-.6-8.1-1.3-12-2.3V384v64c0 35.3 28.7 64 64 64H448c35.3 0 64-28.7 64-64V384 252.6c-4 1-8 1.8-12.3 2.3z"/></svg>
                            <routerLink to="/store" class="nb_to">Магазин</routerLink>
                        </div>
                        <div class="nb_lib">
                            <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="46" viewBox="0 0 640 512"><path fill="#FFFFFF" d="M96 128a128 128 0 1 1 256 0A128 128 0 1 1 96 128zM0 482.3C0 383.8 79.8 304 178.3 304h91.4C368.2 304 448 383.8 448 482.3c0 16.4-13.3 29.7-29.7 29.7H29.7C13.3 512 0 498.7 0 482.3zM609.3 512H471.4c5.4-9.4 8.6-20.3 8.6-32v-8c0-60.7-27.1-115.2-69.8-151.8c2.4-.1 4.7-.2 7.1-.2h61.4C567.8 320 640 392.2 640 481.3c0 17-13.8 30.7-30.7 30.7zM432 256c-31 0-59-12.6-79.3-32.9C372.4 196.5 384 163.6 384 128c0-26.8-6.6-52.1-18.3-74.3C384.3 40.1 407.2 32 432 32c61.9 0 112 50.1 112 112s-50.1 112-112 112z"/></svg>
                            <routerLink to="/friends" class="nb_to">Друзья</routerLink>
                        </div>
                        <div class="nb_lib">
                            <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="40" viewBox="0 0 512 512"><path fill="#FFFFFF" d="M512 240c0 114.9-114.6 208-256 208c-37.1 0-72.3-6.4-104.1-17.9c-11.9 8.7-31.3 20.6-54.3 30.6C73.6 471.1 44.7 480 16 480c-6.5 0-12.3-3.9-14.8-9.9c-2.5-6-1.1-12.8 3.4-17.4l0 0 0 0 0 0 0 0 .3-.3c.3-.3 .7-.7 1.3-1.4c1.1-1.2 2.8-3.1 4.9-5.7c4.1-5 9.6-12.4 15.2-21.6c10-16.6 19.5-38.4 21.4-62.9C17.7 326.8 0 285.1 0 240C0 125.1 114.6 32 256 32s256 93.1 256 208z"/></svg>
                            <routerLink to="/chat" class="nb_to">Чат</routerLink>
                        </div>                
                </div>
            </div>
            <router-link to="/" class="nb_text">ARCANUM<br>LAUNCHER</router-link>
        </aside>
</template>

<script setup>
    import { ref, onMounted } from 'vue';
    import axios from 'axios';
    import router from '../router';
    import jwtDecode from 'jwt-decode';

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

        username.value = res.data.nickname;
        lvl.value = res.data.lvl;
        about.value = res.data.about;
        avatar.value = res.data.avatar;
    });
</script>

<style scoped>
    .aside {
        position: sticky;
        top: 0;
        padding: 0 25px;
        background: #393E46;
        margin-left: 35px;
        display: flex;
        flex-direction: column;
        min-height: 100vh;
        max-height: 100vh;
        min-width: 350px;
    }

    .profile {
        display: flex;
        align-items: center;
        margin-top: 25px;
        cursor: pointer;
    }

    .avatar {
        max-width: 100px;
        max-height: 100px;
        border-radius: 100px;
     
    }

    .nickname {
        font-family: 'Rowdies';
        font-size: 48px;
        line-height: 60px;
        color: #FFFFFF;
        margin: 0 15px;
    }

    .lvl {
        font-family: 'Rowdies';
        font-size: 20px;
        line-height: 25px;
        text-align: center;
        color: #FFFFFF;

        background: #FF2E63;
        border-radius: 25px;
        padding: 5px 21px;
    }

    .nb_text {
        font-family: 'Concert One';
        font-size: 50px;
        line-height: 39px;
        color: #FFFFFF;
        font-style: normal;
        font-weight: 400;
        text-align: center;
        margin-top: auto;
        margin-bottom: 56px;
    }

    .navbar {
        margin-top: 65px;
    }

    .nb_lib {
        padding: 12px 0px;
    }

    .nb_to {
        font-family: 'Russo One';
        font-size: 36px;
        line-height: 43px;
        text-align: center;
        color: #FFFFFF;
        cursor: pointer;
    }

    .icon {
        padding-right: 5px;
    }

    .router-link-active {
        color: #FFDE7D;
    }
</style>