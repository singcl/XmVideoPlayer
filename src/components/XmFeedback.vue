<script setup lang="ts">
import { ref, reactive, computed } from 'vue';
import API from '@/api';
import type { FormInstance } from '@arco-design/web-vue/es/form';
import { Notification } from '@arco-design/web-vue';
// import '@arco-design/web-vue/es/notification/style/css.js';

const loading = ref(false);
const visible = ref(false);
const feedModalVisible = computed(() => ({
  hide: !visible.value,
  show: visible.value,
}));

const formRef = ref<FormInstance>();
const form = reactive({
  message: '',
  email: '',
});

async function handleFeedback(e: Event) {
  e.stopPropagation();
  e.preventDefault();
  visible.value = !visible.value;
}

//
const handleClose = () => {
  visible.value = false;
  formRef.value?.resetFields();
};

const handleSubmit = async () => {
  if (!formRef.value) return;
  const err = await formRef.value?.validate();
  if (err) return;
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  try {
    loading.value = true;
    const response = await API.xmvideo.feedback.feedbackUpdate(form);
    // Message.success(`反馈成功:${response?.id}`);
    Notification.success({
      title: '反馈成功',
      content: `ID:${response?.id}`,
      position: 'bottomRight',
    });
    handleClose();
  } catch (e) {
    console.error('请求出错：', e);
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <div class="feedback">
    <a-tooltip :mini="true">
      <template #content>
        <span style="color: #eee6e6; font-size: 12px">意见与建议</span>
      </template>
      <a-button class="feedback-btn" shape="circle" size="large" status="success" @click="handleFeedback">
        <XmSvgIcon name="icon-feedback" style="padding-left: 3px; font-size: 20px" />
      </a-button>
    </a-tooltip>
  </div>

  <div class="feed-modal" :class="[feedModalVisible]">
    <div class="feed-modal__body">
      <a-form ref="formRef" :model="form" layout="vertical" @submit="handleSubmit">
        <div class="feedback-header">
          <div class="f-title">意见与建议</div>
          <span class="f-close" @click="handleClose">
            <icon-close />
          </span>
          <p class="f-desc">XmVideoPlayer的成长需要您的助力！感谢您提交真实的反馈意见~</p>
        </div>
        <div class="feedback-body">
          <a-form-item
            field="message"
            label="反馈内容"
            feedback
            :rules="[
              { required: true, message: '请输入要反馈的内容' },
              { match: /^\S+$/, message: '输入内容不能包含空字符哦' },
            ]"
          >
            <a-textarea
              v-model="form.message"
              :max-length="200"
              :auto-size="{
                minRows: 5,
                maxRows: 5,
              }"
              show-word-limit
              placeholder="留下你的建议或意见(*￣︶￣)"
            />
          </a-form-item>
          <a-form-item
            field="email"
            label="联系方式"
            feedback
            :rules="[
              { required: true, message: '请输入联系方式' },
              { match: /^\S+$/, message: '输入内容不能包含空字符哦' },
            ]"
          >
            <a-input v-model="form.email" placeholder="少年何不留下你的联系方式(*￣︶￣)" />
          </a-form-item>
          <a-button type="primary" html-type="submit" long :loading="loading">提交</a-button>
        </div>
      </a-form>
    </div>
  </div>
</template>

<style scoped>
.feedback-btn {
  box-shadow: 0 2px 12px #0000001a;
}

.feedback {
  position: absolute;
  top: 50%;
  right: 0;
  padding: 5px 20px;
  transform: translate(50%, -50%) scale(1);
  transition: all 0.2s;
}

.feedback:hover,
.feedback:focus {
  transform: translate(0%, -50%) scale(1.1);
}

.feed-modal {
  position: fixed;
  right: 65px;
  bottom: 36px;
  overflow: hidden;
  width: 300px;
  border-radius: 5px;
  background-color: #fff;
  box-shadow: 0 4px 16px 0 #00000026;
}

.feed-modal.hide {
  z-index: 0;
  opacity: 0;
  transform: scaleX(0.5) scaleY(0.5);
  transition: transform 0.3s ease-out;
  visibility: hidden;
}

.feed-modal.show {
  z-index: 100;
  opacity: 1;
  transform: scaleX(1) scaleY(1);
  transition: transform 0.3s ease-out;
  visibility: visible;
}

.feed-modal__body {
  width: 100%;
  height: 410px;
}

.feedback-header {
  position: relative;
  width: 100%;
  height: 100px;
  box-sizing: border-box;
  padding: 20px 20px 0;
  margin-bottom: 10px;
  background: linear-gradient(180deg, #dbedff, #f7fbff00);
  color: #333;
  font-size: 16px;
  font-weight: 500;
  line-height: 22px;
  text-align: left;
}

.feedback-header .f-title {
  color: #333;
  font-size: 18px;
  font-weight: 550;
  text-align: left;
}

.feedback-header .f-close {
  position: fixed;
  top: 16px;
  right: 16px;
  color: #9a9a9b;
  cursor: pointer;
}

.feedback-header .f-desc {
  margin: 10px 0 19px;
  color: #999;
  font-size: 12px;
  font-weight: 400;
}

.feedback-body {
  padding: 0 20px 20px;
}
</style>
