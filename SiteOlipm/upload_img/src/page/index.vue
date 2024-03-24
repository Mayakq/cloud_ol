<template>
  <div class="main-container">
    <div class="frame">
      <button class="button"><span class="no-name">No Name</span></button>
      <div class="frame-1"><span class="exit">Выйти</span></div>
      <div class="frame-2">
        <div class="search-icon">
          <div class="vector"></div>
        </div>
        <span class="search-text">Поиск...</span>
      </div>
      <div class="frame-3">
        <div class="star"></div>
        <span class="star-stock">Star Stock</span>
      </div>
      <span class="albums">Альбомы</span>
    </div>
    <span class="create-album-4">Создать альбом</span>
    <div class="frame-5">
      <div class="frame-6">
        <input type="file" ref="file-input" @change="handleFileChange">
        <img id="image-preview" src="" alt="Preview">
        <div class="upload-tabler">
          <div class="vector-7"></div>
        </div>
        <span class="file-upload">Выберите файл или перетащите его сюда</span>
      </div>
      <div class="frame-8"><span class="title">Название</span></div>
      <div class="frame-9"><span class="tags">Тэги</span></div>
      <div class="frame-a"></div>
      <span class="auto-tag">Автоматическое добавление тегов</span>
      <div class="frame-b">
        <span class="privacy">Приватность</span>
        <div class="vector-c"></div>
      </div>
      <div class="frame-d">
        <span class="add-collaborators">Добавить соавторов</span>
        <div class="vector-e"></div>
      </div>
      <button class="button-frame" @click="uploadFile">Создать</button>
    </div>
    <span class="choose-media">Выберите фото или видео со своей галлереи</span>
    <div class="flex-row">
      <div class="frame-f">
        <div class="image"></div>
        <div class="frame-10">
          <div class="ellipse"></div>
          <span class="jimmy-loya">Jimmy Loya12</span>
        </div>
      </div>
      <div class="frame-11">
        <div class="image-12"></div>
        <div class="frame-13">
          <div class="ellipse-14"></div>
          <span class="jimmy-loya-15">Jimmy Loya12</span>
        </div>
      </div>
      <div class="frame-16">
        <div class="image-17"></div>
        <div class="frame-18">
          <div class="ellipse-19"></div>
          <span class="jimmy-loya-1a">Jimmy Loya12</span>
        </div>
      </div>
      <div class="frame-1b">
        <div class="rectangle"></div>
        <div class="frame-1c">
          <div class="ellipse-1d"></div>
          <span class="jimmy-loya-1e">Jimmy Loya12</span>
        </div>
      </div>
    </div>
    <div class="flex-row-ac">
      <div class="frame-1f">
        <div class="image-20"></div>
        <div class="frame-21">
          <div class="ellipse-22"></div>
          <span class="jimmy-loya-23">Jimmy Loya12</span>
        </div>
      </div>
      <div class="frame-24">
        <div class="image-25"></div>
        <div class="frame-26">
          <div class="ellipse-27"></div>
          <span class="jimmy-loya-28">Jimmy Loya12</span>
        </div>
      </div>
      <div class="frame-29">
        <div class="frame-2a">
          <div class="ellipse-2b"></div>
          <span class="jimmy-loya-2c">Jimmy Loya12</span>
        </div>
        <div class="image-2d"></div>
        <div class="ellipse-2e"></div>
        <div class="polygon"></div>
      </div>
      <div class="frame-2f">
        <div class="image-30"></div>
        <div class="frame-31">
          <div class="ellipse-32"></div>
          <span class="jimmy-loya-33">Jimmy Loya12</span>
        </div>
      </div>
    </div>
    <div class="flex-row-fb">
      <div class="frame-34">
        <div class="image-35"></div>
        <div class="frame-36">
          <div class="ellipse-37"></div>
          <span class="jimmy-loya-38">Jimmy Loya12</span>
        </div>
      </div>
      <div class="frame-39">
        <div class="image-3a"></div>
        <div class="frame-3b">
          <div class="ellipse-3c"></div>
          <span class="jimmy-loya-3d">Jimmy Loya12</span>
        </div>
      </div>
      <div class="frame-3e">
        <div class="image-3f"></div>
        <div class="frame-40">
          <div class="ellipse-41"></div>
          <span class="jimmy-loya-42">Jimmy Loya12</span>
        </div>
      </div>
      <div class="frame-43">
        <div class="image-44"></div>
        <div class="frame-45">
          <div class="ellipse-46"></div>
          <span class="jimmy-loya-47">Jimmy Loya12</span>
        </div>
      </div>
    </div>
    <div class="flex-row-48">
      <div class="frame-49">
        <div class="image-4a"></div>
        <div class="frame-4b">
          <div class="ellipse-4c"></div>
          <span class="jimmy-loya-4d">Jimmy Loya12</span>
        </div>
      </div>
      <div class="frame-4e">
        <div class="image-4f"></div>
        <div class="frame-50">
          <div class="ellipse-51"></div>
          <span class="jimmy-loya-52">Jimmy Loya12</span>
        </div>
      </div>
      <div class="frame-53">
        <div class="image-54"></div>
        <div class="frame-55">
          <div class="ellipse-56"></div>
          <span class="jimmy-loya-57">Jimmy Loya12</span>
        </div>
      </div>
      <div class="frame-58">
        <div class="image-59"></div>
        <div class="frame-5a">
          <div class="ellipse-5b"></div>
          <span class="jimmy-loya-5c">Jimmy Loya12</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="javascript">
import axios from 'axios';

export default {
  data() {
    return {
      file: null,
      fileExtension: '',
      tags: '123',
      bytes: Uint8Array,
      name: "FILENAME"
    };
  },
  methods: {
    handleFileChange(event) {
      this.file = event.target.files[0];
      this.fileExtension = this.file.name.split('.').pop();
    },
    uploadFile() {
      if (this.file) {
        var reader = new FileReader();
        var fileByteArray = [];
        reader.readAsArrayBuffer(this.file);
        reader.onloadend = function (evt) {
          if (evt.target.readyState === FileReader.DONE) {
            var arrayBuffer = evt.target.result,
                array = new Uint8Array(arrayBuffer);
            for (var i = 0; i < array.length; i++) {
              fileByteArray.push(array[i]);
            }
            console.log(array)
          }
          const formData = new FormData();
          formData.append('name', "this.name");
          formData.append('bytes', fileByteArray);
          formData.append('ext', "this.fileExtension");
          formData.append('tags', "this.tags");

          axios.post("http://localhost:8080/api/lphoto", formData, {
                headers:
                    {
                      "Authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyMjAwMGQ4Zi00NjYwLTQxNjMtYjQ1My02M2JkNzI1MTVhMTYiLCJpYXQiOjE3MTEyMDE0NjEsImV4cCI6MTcxMzg3OTg2MX0.UXRdpI0c_jwIJnARTLuUdq3gqRkuZQDYVdtanLxwTl0",
                      'Content-Type': 'application/json',
                    }
              }
          )
              .then(response => {
                console.log('Файл успешно загружен и отправлен на сервер');
                // Дополнительная логика после успешного завершения загрузки
              })
              .catch(error => {
                console.error('Ошибка при загрузке файла', error);
              });
        }
      }
    }
  }
};
</script>
<style src="./index.css"></style>
