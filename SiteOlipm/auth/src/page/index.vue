<template>
  <div class="main-container">
    <div class="frame">
      <div class="frame-1">
        <div class="frame-2">
          <div class="star"></div>
          <span class="star-stock">Star Stock</span>
        </div>
        <span class="join-star-stock">Присоединяйся к Star Stock</span
        ><span class="share-users"
      >Более 10.000 пользователей уже делятся своими фотографиями и
          видео</span
      ><span class="designed-by-slicky">Designed by Slicky</span>
        <div class="image"></div>
      </div>
      <span class="authorization">Авторизация</span>
      <input type="text" placeholder="Имя пользователя" class="frame-3" v-bind:value="login" @input="updateLogin"><span
        class="user-name">  </span>
      <input type="text" placeholder="Пароль" class="frame-4" v-bind:value="password" @input="updatePassword"><span
        class="password"></span>
      <button class="button" @click="get_token"><span class="continue">Продолжить</span></button
      >
    </div>
  </div>
</template>

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
      await axios.post(HOST + "log/", JSON.stringify({
        name: this.login,
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

<style src="./index.css"></style>
