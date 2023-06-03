<template>
    <header class="header">
        <router-link to="/" class="text1">ARCANUM LAUNCHER</router-link>
        <div class="buttons">
            <router-link to="/signup" class="reg"> Зарегистрироваться</router-link>
            <router-link to="/signin" class="sign">Войти</router-link>
        </div>
    </header>
    <div class="content">
        <div class="content__content">
            <div class="banner">
                <router-link to="/" class="play">Играть</router-link>
            </div>
            <div class="products">
                <div class="product">
                    <div class="discount__div">
                        <p class="discount">-15%</p>
                    </div>
                    <img src="https://mykaleidoscope.ru/x/uploads/posts/2022-09/1663362480_16-mykaleidoscope-ru-p-gon-v-gneve-vkontakte-17.jpg" alt="not img" class="img">
                    <div class="info">
                        <router-link to="/" class="nameGame">NameGame</router-link>
                        <div class="prices">
                            <s class="price">540 Руб</s>
                            <p class="orig_price">249 Руб</p>
                        </div>
                    </div>
                </div>
            </div>
            <footer>
                <h1>ПРОЙДИ СВОЙ ПУТЬ ГЕЙМЕРА!</h1>
            </footer>
        </div>
    </div>

</template>

<script setup>

</script>

<style scoped>
    /*  HEADER  */
    .header {
        background-color: #393E46;
        width: 100%;
        padding: 20px 0;
        display: flex;
        justify-content: space-between;
    }

    .text1 {
        text-decoration: none;  
        font-family: 'Concert One';
        font-style: normal;
        font-size: 48px;
        color: white;
        padding: 0 35px;
    }

    .buttons {
        padding: 0 35px;
        display: flex;
        align-items: center;
    }

    .reg {
        cursor: pointer;
        text-decoration: none;  
        font-family: 'Russo One';
        font-size: 20px;
        line-height: 24px;
        color: #FFFFFF;
        padding: 11px 17px;
        background: #FF2E63;
        border-radius: 15px;
    }

    .sign {
        text-decoration: none;  
        cursor: pointer;
        margin-left: 35px;
        font-family: 'Russo One';
        font-size: 20px;
        line-height: 24px;
        color: #FFFFFF;
    }

    /*  CONTENT  */
    .content {
        padding: 43px;
    }

    .content__content {
        width: 90%;
        margin: 0 auto;
    }

    .banner {
        display:flex;
        justify-content:end;
        align-items:end;       
        height: 309px;
        background: url("https://i.pinimg.com/originals/16/54/6d/16546d2379fa3b47e85934301bcc0340.jpg") 50%/cover no-repeat;
        border-radius: 15px;
        margin-bottom: 83px;
    }

    .play {
        text-decoration: none;
        background: #17B978;
        border-radius: 15px;
        font-family: 'Russo One';
        font-size: 40px;
        line-height: 48px;
        color: #FFFFFF;
        padding: 6px 72px;
        cursor: pointer;
        margin: 0 35px 20px 0;
    }

    /* продукты сцуки */

    .products {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
    }

    .product {
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

    footer h1 {
        font-family: 'Russo One';
        font-size: 64px;
        line-height: 77px;
        text-align: center;
        color: #FFFFFF;
    }
</style>