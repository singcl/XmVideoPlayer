<template>
  <div class="theme-wrapper">
    <a-popover trigger="click" content-class="theme-popover">
      <icon-palette :size="16" />
      <template #title>
        <span style="font-size: 10px">主题切换</span>
      </template>
      <template #content>
        <a-radio-group>
          <template v-for="item in 2" :key="item">
            <a-radio :value="item">
              <template #radio="{ checked }">
                <a-space align="start" class="custom-radio-card" :class="{ 'custom-radio-card-checked': checked }">
                  <div className="custom-radio-card-mask">
                    <div className="custom-radio-card-mask-dot" />
                  </div>
                  <div>
                    <div className="custom-radio-card-title">radio Card {{ item }}</div>
                    <a-typography-text type="secondary"> this is a text </a-typography-text>
                  </div>
                </a-space>
              </template>
            </a-radio>
          </template>
        </a-radio-group>
      </template>
    </a-popover>
  </div>
</template>

<script setup lang="ts"></script>

<style scoped>
.theme-wrapper {
  position: absolute;
  top: 3px;
  right: 5px;
  line-height: 1;
  /* color: rgb(255, 154, 46); */
}

.theme {
  display: flex;
  flex-direction: row;
}

.theme__item {
  width: 80px;
  height: 60px;
  background-color: red;
}

.theme__item:not(:last-child) {
  margin-right: 5px;
}
</style>

<style>
.theme-popover {
  padding: 5px;
}

.theme-popover .arco-popover-title {
  line-height: 1;
  font-size: 10px;
}

.custom-radio-card {
  padding: 10px 16px;
  border: 1px solid var(--color-border-2);
  border-radius: 4px;
  width: 250px;
  box-sizing: border-box;
}

.custom-radio-card-mask {
  height: 14px;
  width: 14px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 100%;
  border: 1px solid var(--color-border-2);
  box-sizing: border-box;
}

.custom-radio-card-mask-dot {
  width: 8px;
  height: 8px;
  border-radius: 100%;
}

.custom-radio-card-title {
  color: var(--color-text-1);
  font-size: 14px;
  font-weight: bold;
  margin-bottom: 8px;
}

.custom-radio-card:hover,
.custom-radio-card-checked,
.custom-radio-card:hover .custom-radio-card-mask,
.custom-radio-card-checked .custom-radio-card-mask {
  border-color: rgb(var(--primary-6));
}

.custom-radio-card-checked {
  background-color: var(--color-primary-light-1);
}

.custom-radio-card:hover .custom-radio-card-title,
.custom-radio-card-checked .custom-radio-card-title {
  color: rgb(var(--primary-6));
}

.custom-radio-card-checked .custom-radio-card-mask-dot {
  background-color: rgb(var(--primary-6));
}
</style>
