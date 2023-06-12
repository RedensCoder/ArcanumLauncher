<template>
    <div class="flex">
        <Aside />
        <div class="content">
            <div class="profile">
                <div class="change_ava">
                    <img v-if="img_ava === ''" :src="avatar" alt="no ava" class="ava">
                    <img v-else :src="img_ava" alt="no ava" class="ava">
                    <input @change="avaChange" type="file" accept="image/png, image/jpg, image/jpeg" id="button" class="input_hidden">
                    <label class="change" for="button">Загрузить</label>
                </div>
                <div class="change_main">
                    <p class="nickname">{{ username }}</p>
                    <input v-model="fUsername" type="text" placeholder="Имя пользователя" required />
                    <input v-model="pass" type="password" placeholder="Пароль" required />
                    <input v-model="about" type="text" placeholder="О себе" required />
                    <button @click="change" class="change">Изменить</button>
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
import router from '../router';

    const username = ref("");
    const avatar = ref("");

    const fUsername = ref("");
    const about = ref("");
    const pass = ref("");

    const img = ref("")
    const img_ava = ref("");

    const change = async () => {
        if (img.value !== "") {
            const file = new FormData();
            file.append("avatar", img.value)
            
            await axios.post("http://127.0.0.1:8080/api/v1/upload", file, {
                headers: {
                    "Authorization": localStorage.getItem("token"),
                    "Content-Type": "multipart/form-data"
                }
            })
        }

        if (fUsername === "") {
            await axios.post("http://127.0.0.1:8080/api/v1/updateAtribut", {
                username: jwtDecode(localStorage.getItem("token")).user.username,
                password: pass.value,
                atribut: {
                    about: about.value
                }
            }, {
                headers: {
                    "Authorization": localStorage.getItem("token")
                }
            })
        } else {
            await axios.post("http://127.0.0.1:8080/api/v1/updateAtribut", {
                username: jwtDecode(localStorage.getItem("token")).user.username,
                password: pass.value,
                atribut: {
                    about: about.value
                }
            }, {
                headers: {
                    "Authorization": localStorage.getItem("token")
                }
            })

            await axios.post("http://127.0.0.1:8080/api/v1/updateAtribut", {
                username: jwtDecode(localStorage.getItem("token")).user.username,
                password: pass.value,
                atribut: {
                    nickname: fUsername.value
                }
            }, {
                headers: {
                    "Authorization": localStorage.getItem("token")
                }
            })
        }

        router.push("/profile")
    }

    const avaChange = (e) => {
        img.value = e.target.files[0];
        img_ava.value = URL.createObjectURL(img.value);
    }

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
        avatar.value = res.data.avatar;
    });
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
        margin: 30px 0;
        padding: 7px 132px;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 32px;
        line-height: 39px;
        text-align: center;
        color: #FFFFFF;
        background: #17B978;
        border-radius: 15px;
        cursor: pointer;
    }

</style>