<template>
    <header>
        <router-link to="/"><h1>ARCANUM LAUNCHER</h1></router-link>
    </header>

    <div class="content">
        <div class="form">
            <p class="label">Авторизация</p>
            <input v-model="login" type="text" placeholder="Имя пользователя" required />
            <input v-model="pass" type="password" placeholder="Пароль" required />
            <p class="text1">У тебя всё ещё нету аккаунта? <router-link to="/signup" class="auth">Регистрация</router-link></p>
            <button @click="auth">Войти</button>
            <p class="error">{{ error }}</p>
        </div>
    </div>

    <footer>
        <h1>ПРОЙДИ СВОЙ ПУТЬ ГЕЙМЕРА!</h1>
    </footer>
</template>

<script setup>
    import { ref } from 'vue';
    import axios from "axios";
    import router from '../router';
    import { onMounted } from 'vue';

    const error = ref("");

    const login = ref("");
    const pass = ref("");

    const auth = async () => {
        if (login.value !== "" || pass.value !== "") {
            const req = await axios.post("http://127.0.0.1:8080/api/v1/auth", {
                type: "auth",
                username: login.value,
                password: pass.value
            });

            if (req.data != "User not found!") {
                localStorage.setItem("token", req.data);
                router.push("/lib");
            } else {
                error.value = "Игрок не найден!";
            }
        } else {
            error.value = "Игрок, ты заполнил не все данные!";
        }
    }

    onMounted(() => {
        if (localStorage.getItem("token") !== null) {
            router.push("/lib");
        }
    })
</script>

<style scoped>
    header {
        background: #393E46;
        padding: 20px 0;
    }

    header h1 {
        font-family: 'Concert One';
        font-size: 48px;
        line-height: 37px;
        text-align: center;
        color: #FFFFFF;
        font-weight: normal;
    }

    .content {
       
        width: 100%;
        height: 100%;
        padding: 76px 0;
    }

    .form {
        width: 100%;
        display:flex;
        flex-direction: column;
        justify-content:center;
        align-items:center;
    }

    .form input {
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
        width: 30%;
    }
    
    .form button {
        margin-top: 32px;
        font-family: 'Russo One';
        font-size: 32px;
        line-height: 39px;
        text-align: center;
        color: #FFFFFF;
        background: #17B978;
        border-radius: 15px;
        padding: 7px 115px;
        cursor: pointer;
    }

    .label {
        padding-bottom: 49px;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 48px;
        line-height: 58px;
        color: #FFFFFF;
    }

    .text1 {
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 20px;
        line-height: 24px;
        color: #FFFFFF;
        margin-top: 20px;
        width: 30%;
        text-align: start;
    }

    .auth {
        text-decoration: none;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 20px;
        line-height: 24px;
        color: #FFDE7D;
        cursor: pointer;
    }

    footer {
        position: absolute;
        bottom: 20px;
        width: 100%;
    }

    footer h1 {
        font-family: 'Russo One';
        font-size: 64px;
        line-height: 77px;
        text-align: center;
        color: #FFFFFF;
    }

    .error {
        color: #FF2E63;
        font-size: 25px;
        font-family: "Russo One";
        margin-top: 15px;
    }
</style>