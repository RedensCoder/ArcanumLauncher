<template>
    <div class="flex">
        <Aside />
        <div class="content">
            <p class="game_n">Dead By Daylight</p>
            <div class="imgs">
                <div class="img">
                    <img src="https://pic.rutubelist.ru/video/4f/d5/4fd5d725d4e53f564f0ff9021fab2040.jpg" alt="" class="main">
                        <div class="info">
                            <img src="https://pic.rutubelist.ru/video/4f/d5/4fd5d725d4e53f564f0ff9021fab2040.jpg" alt="" class="ava">
                            <p class="desc">Описание Описание Описание Описание Описание Описание Описание Описание Описание</p>
                            <p class="author">Автор: <span class="color_wh">Papa</span></p>
                            <p class="genre">Жанр: <span class="color_wh">Хоррор</span></p>
                        </div>
                    </div>
                        <div class="img_dop">
                            <img src="https://adonius.club/uploads/posts/2022-01/1643632815_66-adonius-club-p-fon-dbd-75.jpg" alt="" class="dop">
                            <img src="https://phonoteka.org/uploads/posts/2021-04/1618487336_44-p-dbd-fon-47.jpg" alt="" class="dop">
                            <img src="https://sun9-49.userapi.com/impg/jklZ0XItpXReAu3yH4CWLeVggpGIKyMMqoKvcg/LivFBi9rWQ8.jpg?size=1280x720&quality=95&sign=0142e7c00a8e07917568f99729e48c40&c_uniq_tag=CVarfLe4S2zzGO4_w5fDVoQERIf3CF1FXi8xvA6pmVc&type=album" alt="" class="dop">
                            <img src="https://i.ytimg.com/vi/5sZp-6E8Ruc/maxresdefault.jpg" alt="" class="dop">
                        </div>
                        <div class="buy_game">
                            <router-link class="buy" to="/">Купить</router-link>
                            <div class="prices">
                                <s class="price">540 Руб</s>
                                <p class="orig_price">249 Руб</p>
                            </div>
                        </div>
                  </div>
                  <footer class="footer">
                        <p class="info_game">Об игре</p>
                        <hr class="hr">
                        <p class="desc_foo">Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания Еще больше описания</p>
                  </footer>
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

    .footer{
        display: grid;
    }

    .hr{
        background: #ffffff;
        border: 5px;           
        height: 5px;
        width: 874px;
        margin-left: 58px;
        margin-top: 9px;
    }

    .color_wh{
        color: #FFFFFF;
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
    }

    .main{
        width: 900px;
        height: 400px;
        padding: 0px 0px 0px 58px;
    }

    .ava{
        width: 400px;
        height: 200px;
        padding: 0px 20px;
    }

    .desc{
        max-width: 500px;
        margin-left: 20px;
        margin-top: 20px;
        font-family: 'Russo One';
        font-size: 24px;
        line-height: 29px;
        color: #FFFFFF;
    }

    .author{
        margin-top: 14px; 
        padding: 10px 20px 5px 20px;
        font-family: 'Russo One';
        font-size: 24px;
        line-height: 29px;
        color: #17B978;
    }

    .genre{
        padding: 0 20px;
        font-family: 'Russo One';
        font-size: 24px;
        line-height: 29px;
        color: #17B978;
    }

    .img_dop {
        display: flex;
        margin-left: 58px;
        gap: 12px;
    }

    .dop{
        margin-top: 20px;
        width: 216px;
        height: 100px;
    }

    .buy_game{
        display: flex;
        margin-top: 30px;
    }

    .buy{
        margin-top: 10px;
        margin-left: 58px;
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

    .info_game{
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 48px;
        line-height: 58px;
        color: #FFFFFF;
        margin-left: 58px;
        margin-top: 36px;
    }

    .desc_foo{
        margin-top: 32px;
        max-width: 1400px;
        max-height: 400px;
        margin-left: 58px;
        font-family: 'Russo One';
        font-style: normal;
        font-weight: 400;
        font-size: 32px;
        line-height: 39px;
        color: #FFFFFF;
    }
</style>