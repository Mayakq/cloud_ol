<template>

  <div  id="app" class="main-container">
    <div class="frame">
      <div class="frame-1">
        <div class="frame-2">
          <div class="star"></div>
          <span class="star-stock">Star Stock</span>
        </div>
        <span class="join-star-stock">Присоединяйся к Star Stock</span
        ><span class="user-sharing"
          >Более 10.000 пользователей уже делятся своими фотографиями и
          видео</span
        ><span class="designed-by-slicky">Designed by Slicky</span>
        <div class="image"></div>
      </div>
      <div class="frame-3">
        <div class="frame-4">
          <div class="frame-5">
            <div class="star-6"></div>
            <span class="star-stock-7">Star Stock</span>
          </div>
          <span class="join-star-stock-8">Присоединяйся к Star Stock</span
          ><span class="share-users"
            >Более 10.000 пользователей уже делятся своими фотографиями и
            видео</span
          ><span class="designed-by-slicky-9">Designed by Slicky</span>
          <div class="image-a"></div>
        </div>
        <span class="registration">Регистрация</span>
        <input v-bind:value="login" @input="updateLogin" type="text" placeholder="Имя пользователя" class="frame-b">{{ login?.name }}
          <span class="username"></span>
        <input v-bind:value="password" @input="updatePassword" type="text" placeholder="Пароль" class="frame-c"><span class="password">{{ password?.name }} </span>
        <span class="password-min-length"
          >Пароль должен содержать не менее 8 символов</span
        >
        <input type="text" placeholder="Пвторите пароль" class="frame-d" v-on:keyup.enter="request">
          <span class="confirm-password"></span>
        <button @click="get_token" class="frame-e" value="Продолжить">Продолжить</button>
      </div>
      <span class="registration-12">Регистрация</span>
      <div class="frame-13">
        <span class="username-14">Имя пользователя</span>
      </div>
      <div class="frame-15"><span class="password-16">Пароль</span></div>
      <span class="password-length"
        >Пароль должен содержать не менее 8 символов</span
      >
      <div class="frame-17">
        <span class="confirm-password-18">Введите пароль еще раз</span>
      </div>
      <div class="line-1c"></div>
      <div class="line-1d"></div>
      <button class="frame-1e">
        <div class="google-1f"></div>
      </button>
    </div>
  </div>
</template>

<style>
:root {
  --default-font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
    Ubuntu, "Helvetica Neue", Helvetica, Arial, "PingFang SC",
    "Hiragino Sans GB", "Microsoft Yahei UI", "Microsoft Yahei",
    "Source Han Sans CN", sans-serif;
}

.main-container {
  overflow: hidden;
}

.main-container,
.main-container * {
  box-sizing: border-box;
}

input,
select,
textarea,
button {
  outline: 0;
}

.main-container {
  position: relative;
  width: 1920px;
  height: 1080px;
  margin: 0 auto;
  background: #0f0f0f;
  overflow: hidden;
}
.frame {
  position: relative;
  width: 1400px;
  height: 1080px;
  margin: 0 0 0 260px;
  background: #ffffff;
  overflow: hidden;
}
.frame-1 {
  position: absolute;
  width: 700px;
  height: 1080px;
  top: 0;
  left: 0;
  background: #000000;
  z-index: 1;
  overflow: hidden;
}
.frame-2 {
  position: relative;
  width: 169px;
  height: 36px;
  margin: 114px 0 0 102px;
  z-index: 5;
}
.star {
  position: absolute;
  width: 36px;
  height: 36px;
  top: 0;
  left: 0;
  background: url(./assets/images/7e522244-9f2e-4f7f-9bc1-cf7249bd45d5.png)
    no-repeat center;
  background-size: cover;
  z-index: 7;
}
.star-stock {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 3px;
  left: 51px;
  color: #ffffff;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  z-index: 6;
}
.join-star-stock {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: relative;
  width: 528px;
  height: 154px;
  margin: 90px 0 0 102px;
  color: #ffffff;
  font-family: Inter, var(--default-font-family);
  font-size: 64px;
  font-weight: 700;
  line-height: 77.455px;
  text-align: left;
  z-index: 3;
}
.user-sharing {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: relative;
  width: 528px;
  height: 192px;
  margin: 60px 0 0 102px;
  color: #ffffff;
  font-family: Inter, var(--default-font-family);
  font-size: 40px;
  font-weight: 500;
  line-height: 48.409px;
  text-align: left;
  z-index: 4;
}
.designed-by-slicky {
  display: block;
  position: relative;
  height: 29px;
  margin: 325px 0 0 102px;
  color: #ffffff;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  z-index: 8;
}
.image {
  position: absolute;
  width: 813px;
  height: 1080px;
  top: 0;
  left: -113px;
  background: url(./assets/images/13b2cde4d83f0631baebeb40540b4c2a73402b66.png)
    no-repeat center;
  background-size: cover;
  opacity: 0.6;
  z-index: 2;
}
.frame-3 {
  position: absolute;
  width: 1400px;
  height: 1080px;
  top: 0;
  left: 0;
  background: #ffffff;
  z-index: 28;
  overflow: hidden;
}
.frame-4 {
  position: absolute;
  width: 700px;
  height: 1080px;
  top: 0;
  left: 0;
  background: #000000;
  z-index: 29;
  overflow: hidden;
}
.frame-5 {
  position: relative;
  width: 169px;
  height: 36px;
  margin: 114px 0 0 102px;
  z-index: 33;
}
.star-6 {
  position: absolute;
  width: 36px;
  height: 36px;
  top: 0;
  left: 0;
  background: url()
    no-repeat center;
  background-size: cover;
  z-index: 35;
}
.star-stock-7 {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 3px;
  left: 51px;
  color: #ffffff;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  z-index: 34;
}
.join-star-stock-8 {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: relative;
  width: 528px;
  height: 154px;
  margin: 90px 0 0 102px;
  color: #ffffff;
  font-family: Inter, var(--default-font-family);
  font-size: 64px;
  font-weight: 700;
  line-height: 77.455px;
  text-align: left;
  z-index: 31;
}
.share-users {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: relative;
  width: 528px;
  height: 192px;
  margin: 60px 0 0 102px;
  color: #ffffff;
  font-family: Inter, var(--default-font-family);
  font-size: 40px;
  font-weight: 500;
  line-height: 48.409px;
  text-align: left;
  z-index: 32;
}
.designed-by-slicky-9 {
  display: block;
  position: relative;
  height: 29px;
  margin: 325px 0 0 102px;
  color: #ffffff;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  z-index: 36;
}
.image-a {
  position: absolute;
  width: 813px;
  height: 1080px;
  top: 0;
  left: -113px;
  background: url(./assets/images/13b2cde4d83f0631baebeb40540b4c2a73402b66.png)
    no-repeat center;
  background-size: cover;
  opacity: 0.6;
  z-index: 30;
}
.registration {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 48px;
  top: 117px;
  left: 780px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 40px;
  font-weight: 700;
  line-height: 48px;
  text-align: left;
  white-space: nowrap;
  z-index: 37;
}
.frame-b {
  position: absolute;
  width: 550px;
  height: 80px;
  top: 225px;
  left: 780px;
  background: #ffffff;
  border: 1px solid #000000;
  z-index: 38;
  overflow: hidden;
  color: black;
  font-size: 24px;
  opacity: 0.5;
  font-family: "Inter-regular", sans-serif;
  padding-left: 25px;
  border-radius: 10px;
}
.username {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 24px;
  left: 24px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  opacity: 0.5;
  z-index: 39;
}
.frame-c {
  position: absolute;
  width: 550px;
  height: 80px;
  top: 345px;
  left: 780px;
  background: #ffffff;
  border: 1px solid #000000;
  z-index: 40;
  overflow: hidden;
  border-radius: 10px;
  color: black;
  font-size: 24px;
  opacity: 0.5;
  font-family: "Inter-regular", sans-serif;
  padding-left: 25px;
}
.password {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 24px;
  left: 24px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  opacity: 0.5;
  z-index: 41;
}
.password-min-length {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 19px;
  top: 431px;
  left: 805px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 16px;
  font-weight: 400;
  line-height: 19px;
  text-align: left;
  white-space: nowrap;
  opacity: 0.8;
  z-index: 55;
}
.frame-d {
  position: absolute;
  width: 550px;
  height: 80px;
  top: 465px;
  left: 780px;
  background: #ffffff;
  border: 1px solid #000000;
  z-index: 42;
  overflow: hidden;
  border-radius: 10px;
  color: black;
  font-size: 24px;
  opacity: 0.5;
  font-family: "Inter-regular", sans-serif;
  padding-left: 25px;
}
.confirm-password {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 24px;
  left: 24px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  opacity: 0.5;
  z-index: 43;
}
.frame-e {
    font-size:  24px;
    font-family: "Inter-medium", sans-serif;
    color: #ffffff;
  position: absolute;
  width: 550px;
  height: 80px;
  top: 585px;
  left: 780px;
  background: linear-gradient(135deg, #3800b1, #ac00d7);
  z-index: 44;
  overflow: hidden;
  border-radius: 10px;
}
.continue {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 25px;
  left: 196px;
  color: #ffffff;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 500;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  z-index: 45;
}
.or {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 708px;
  left: 1033px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 300;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  z-index: 54;
}
.line {
  position: absolute;
  width: 220px;
  height: 1px;
  top: 724px;
  left: 780px;
  background: url(./assets/images/b7329a66-cce7-468c-93a6-ee0669940c71.png)
    no-repeat center;
  background-size: cover;
  z-index: 52;
}
.line-f {
  position: absolute;
  width: 220px;
  height: 1px;
  top: 724px;
  left: 1110px;
  background: url(./assets/images/5497cad9-3d41-4f30-9473-868e0a6880be.png)
    no-repeat center;
  background-size: cover;
  z-index: 53;
}
.frame-10 {
  position: absolute;
  width: 550px;
  height: 80px;
  top: 779px;
  left: 780px;
  background: #ffffff;
  border: 1px solid #000000;
  z-index: 46;
  overflow: hidden;
  border-radius: 10px;
}
.google {
  position: absolute;
  width: 30px;
  height: 30px;
  top: 24px;
  left: 29px;
  background: url(./assets/images/b5a447ac-70d8-4551-bc51-07e465fc9631.png)
    no-repeat center;
  background-size: cover;
  z-index: 47;
  overflow: hidden;
}
.continue-with-google {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 24px;
  left: 145px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  z-index: 48;
}
.frame-11 {
  position: absolute;
  width: 550px;
  height: 80px;
  top: 899px;
  left: 780px;
  background: #ffffff;
  border: 1px solid #000000;
  z-index: 49;
  overflow: hidden;
  border-radius: 10px;
}
.continue-with-apple {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 24px;
  left: 153px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  z-index: 50;
}
.apple-icon {
  position: absolute;
  width: 30px;
  height: 30px;
  top: 24px;
  left: 29px;
  background: url(./assets/images/4d53c32c-ba1f-46df-aa71-1f0ef934fd59.png)
    no-repeat center;
  background-size: cover;
  z-index: 51;
  overflow: hidden;
}
.registration-12 {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 48px;
  top: 117px;
  left: 780px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 40px;
  font-weight: 700;
  line-height: 48px;
  text-align: left;
  white-space: nowrap;
  z-index: 9;
}
.frame-13 {
  position: absolute;
  width: 550px;
  height: 80px;
  top: 225px;
  left: 780px;
  background: #ffffff;
  border: 1px solid #000000;
  z-index: 10;
  overflow: hidden;
  border-radius: 10px;
}
.username-14 {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 24px;
  left: 24px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  opacity: 0.5;
  z-index: 11;
}
.frame-15 {
  position: absolute;
  width: 550px;
  height: 80px;
  top: 345px;
  left: 780px;
  background: #ffffff;
  border: 1px solid #000000;
  z-index: 12;
  overflow: hidden;
  border-radius: 10px;
}
.password-16 {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 24px;
  left: 24px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  opacity: 0.5;
  z-index: 13;
}
.password-length {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 19px;
  top: 431px;
  left: 805px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 16px;
  font-weight: 400;
  line-height: 19px;
  text-align: left;
  white-space: nowrap;
  opacity: 0.8;
  z-index: 27;
}
.frame-17 {
  position: absolute;
  width: 550px;
  height: 80px;
  top: 465px;
  left: 780px;
  background: #ffffff;
  border: 1px solid #000000;
  z-index: 14;
  overflow: hidden;
  border-radius: 10px;
}
.confirm-password-18 {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 24px;
  left: 24px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  opacity: 0.5;
  z-index: 15;
}
.frame-19 {
  position: absolute;
  width: 550px;
  height: 80px;
  top: 585px;
  left: 780px;
  cursor: pointer;
  background: linear-gradient(135deg, #3800b1, #ac00d7);
  border: none;
  z-index: 16;
  overflow: hidden;
  border-radius: 10px;
}
.continue-1a {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 25px;
  left: 196px;
  color: #ffffff;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 500;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  z-index: 17;
}
.or-1b {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 708px;
  left: 1033px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 300;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  z-index: 26;
}
.line-1c {
  position: absolute;
  width: 220px;
  height: 1px;
  top: 724px;
  left: 780px;
  background: url(./assets/images/31e21ada-cfea-4d08-87d2-a7f834e8bc2d.png)
    no-repeat center;
  background-size: cover;
  z-index: 24;
}
.line-1d {
  position: absolute;
  width: 220px;
  height: 1px;
  top: 724px;
  left: 1110px;
  background: url(./assets/images/ea5be2ca-479d-4e5b-b7ee-5af5fc8ea2d5.png)
    no-repeat center;
  background-size: cover;
  z-index: 25;
}
.frame-1e {
  position: absolute;
  width: 550px;
  height: 80px;
  top: 779px;
  left: 780px;
  cursor: pointer;
  background: #ffffff;
  border: 1px solid #000000;
  z-index: 18;
  overflow: hidden;
  border-radius: 10px;
}
.google-1f {
  position: absolute;
  width: 30px;
  height: 30px;
  top: 24px;
  left: 29px;
  background: url(./assets/images/e0fc77b5-2754-4028-b496-a35d8ff25eff.png)
    no-repeat center;
  background-size: cover;
  z-index: 19;
  overflow: hidden;
}
.continue-with-google-20 {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 24px;
  left: 145px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  z-index: 20;
}
.frame-21 {
  position: absolute;
  width: 550px;
  height: 80px;
  top: 899px;
  left: 780px;
  cursor: pointer;
  background: #ffffff;
  border: 1px solid #000000;
  z-index: 21;
  overflow: hidden;
  border-radius: 10px;
}
.continue-with-apple-22 {
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
  position: absolute;
  height: 29px;
  top: 24px;
  left: 153px;
  color: #000000;
  font-family: Inter, var(--default-font-family);
  font-size: 24px;
  font-weight: 400;
  line-height: 29px;
  text-align: left;
  white-space: nowrap;
  z-index: 22;
}
.apple {
  position: absolute;
  width: 30px;
  height: 30px;
  top: 24px;
  left: 29px;
  background: url(./assets/images/d0f26d53-557f-4088-9123-40b3f79deee7.png)
    no-repeat center;
  background-size: cover;
  z-index: 23;
  overflow: hidden;
}
</style>

<script setup lang="javascript">
import axios from "axios";

const HOST = "http://localhost:8080/api/"
export default {
  data() {
    return {
      login: "",
      password: ""
    }
  },
  methods: {
    updateLogin(event) {
      this.login = event.target.value;
      console.log(this.login)
    },
    updatePassword(event) {
      this.password = event.target.value;
      console.log(this.password)
    },
    async get_token() {
      await axios.post(HOST + "reg/", JSON.stringify({
        username: this.login,
        password: this.password
      }), {headers: {'Content-Type': 'application/json'}}).then(function (response) {
        console.log('Authenticated');
      })
      // В РЕЗУЛЬТАТЕ ТЫ ПОЛУЧАЕШЬ ТОКЕН КОТОРЫЙ НУЖНО СОХРАНИТЬ В LOCAL STORAGE
      console.log(result)
    }
  }
}
</script>
