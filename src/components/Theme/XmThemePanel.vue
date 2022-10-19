<template>
  <div class="theme-wrapper">
    <a-popover trigger="click" content-class="theme-popover">
      <icon-palette :size="16" />
      <template #title>
        <span>主题切换</span>
      </template>
      <template #content>
        <a-radio-group>
          <template v-for="item in 2" :key="item">
            <a-radio :value="item">
              <template #radio="{ checked }">
                <div class="custom-radio-card" :class="{ 'custom-radio-card-checked': checked }">
                  <div className="custom-radio-card-mask">
                    <div className="custom-radio-card-mask-dot" />
                  </div>
                  <div>
                    <a-typography-text type="secondary">金秋黄色</a-typography-text>
                  </div>
                </div>
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
  font-size: 12px;
  margin-bottom: 3px;
}

.theme-popover .arco-radio-group .arco-radio {
  margin-right: 0;
}

.theme-popover .arco-radio-group .arco-radio:first-child {
  padding-left: 0;
}

.theme-popover .custom-radio-card {
  padding: 10px 16px;
  border: 1px solid var(--color-border-2);
  border-radius: 4px;
  width: 200px;
  box-sizing: border-box;
}

.theme-popover .custom-radio-card-mask {
  height: 14px;
  width: 14px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 100%;
  border: 1px solid var(--color-border-2);
  box-sizing: border-box;
}

.theme-popover .custom-radio-card-mask-dot {
  width: 8px;
  height: 8px;
  border-radius: 100%;
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

.custom-radio-card-checked .custom-radio-card-mask-dot {
  background-color: rgb(var(--primary-6));
}
</style>
