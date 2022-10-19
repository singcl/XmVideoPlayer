<template>
  <div class="theme-wrapper">
    <a-popover trigger="click" content-class="theme-popover">
      <icon-palette :size="18" />
      <template #title>
        <span>主题切换</span>
      </template>
      <template #content>
        <a-radio-group v-model="themeSelected" @change="handleThemeChange">
          <template v-for="theme in themes" :key="theme.value">
            <a-radio :value="theme.value">
              <template #radio="{ checked }">
                <div
                  class="theme-radio-card"
                  :class="[{ 'theme-radio-card-checked': checked }, `theme-${theme.value}`]"
                >
                  <div className="theme-radio-card-mask">
                    <div className="theme-radio-card-mask-dot" />
                  </div>
                  <div>
                    <a-typography-text type="secondary" class="theme-radio-card-txt">
                      {{ theme.label }}
                    </a-typography-text>
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

<script setup lang="ts">
import { ref, onMounted } from 'vue';
const themeSelected = ref('blue');
const themes = [
  {
    label: '深海蓝',
    value: 'blue',
  },
  {
    label: '喜庆红',
    value: 'red',
  },
  {
    label: '清新绿',
    value: 'green',
  },
  {
    label: '金秋黄',
    value: 'gold',
  },
];
onMounted(() => {
  const defaultTheme = localStorage.getItem('xm-theme');
  if (defaultTheme) {
    themeSelected.value = defaultTheme;
    document.body.setAttribute('xm-theme', defaultTheme);
  }
});
function handleThemeChange(v: string | number | boolean) {
  document.body.setAttribute('xm-theme', v as string);
  localStorage.setItem('xm-theme', v as string);
}
</script>

<style scoped>
.theme-wrapper {
  position: absolute;
  top: 5px;
  right: 8px;
  line-height: 1;
  /* color: rgb(255, 154, 46); */
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

.theme-popover .arco-radio-group {
  display: flex;
  flex-direction: column;
}

.theme-popover .arco-radio-group .arco-radio {
  margin-right: 0;
  padding-left: 0;
}

.theme-popover .arco-radio-group .arco-radio:not(:last-child) {
  margin-bottom: 5px;
}

.theme-popover .theme-radio-card {
  padding: 10px 16px;
  border: 1px solid var(--color-border-2);
  border-radius: 4px;
  width: 200px;
  box-sizing: border-box;
}

.theme-popover .theme-radio-card-mask {
  height: 14px;
  width: 14px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 100%;
  border: 1px solid var(--color-border-2);
  box-sizing: border-box;
}

.theme-popover .theme-radio-card-mask-dot {
  width: 8px;
  height: 8px;
  border-radius: 100%;
}

.theme-popover .theme-radio-card-txt {
  color: rgb(var(--xm-txt-10));
}

.theme-popover .theme-radio-card.theme-blue {
  background: linear-gradient(91deg, rgb(var(--blue-1)), rgb(var(--blue-5)) 70%, rgb(var(--blue-7)));
}
.theme-popover .theme-radio-card.theme-red {
  background: linear-gradient(91deg, rgb(var(--red-1)), rgb(var(--red-5)) 70%, rgb(var(--red-7)));
}
.theme-popover .theme-radio-card.theme-green {
  background: linear-gradient(91deg, rgb(var(--green-1)), rgb(var(--green-5)) 70%, rgb(var(--green-7)));
}
.theme-popover .theme-radio-card.theme-gold {
  background: linear-gradient(91deg, rgb(var(--gold-1)), rgb(var(--gold-5)) 70%, rgb(var(--gold-7)));
}

.theme-popover .theme-blue.theme-radio-card:hover,
.theme-popover .theme-blue.theme-radio-card-checked,
.theme-popover .theme-blue.theme-radio-card:hover .theme-radio-card-mask,
.theme-popover .theme-blue.theme-radio-card-checked .theme-radio-card-mask {
  border-color: rgb(var(--primary-6));
}

.theme-popover .theme-blue.theme-radio-card-checked {
  background-color: var(--color-primary-light-1);
}

.theme-popover .theme-blue.theme-radio-card-checked .theme-radio-card-mask-dot {
  background-color: rgb(var(--primary-6));
}

.theme-popover .theme-red.theme-radio-card:hover,
.theme-popover .theme-red.theme-radio-card-checked,
.theme-popover .theme-red.theme-radio-card:hover .theme-radio-card-mask,
.theme-popover .theme-red.theme-radio-card-checked .theme-radio-card-mask {
  border-color: rgb(var(--danger-6));
}

.theme-popover .theme-red.theme-radio-card-checked {
  background-color: var(--color-danger-light-1);
}

.theme-popover .theme-red.theme-radio-card-checked .theme-radio-card-mask-dot {
  background-color: rgb(var(--danger-6));
}

.theme-popover .theme-green.theme-radio-card:hover,
.theme-popover .theme-green.theme-radio-card-checked,
.theme-popover .theme-green.theme-radio-card:hover .theme-radio-card-mask,
.theme-popover .theme-green.theme-radio-card-checked .theme-radio-card-mask {
  border-color: rgb(var(--green-6));
}

.theme-popover .theme-green.theme-radio-card-checked {
  background-color: var(--color-green-light-1);
}

.theme-popover .theme-green.theme-radio-card-checked .theme-radio-card-mask-dot {
  background-color: rgb(var(--green-6));
}

.theme-popover .theme-gold.theme-radio-card:hover,
.theme-popover .theme-gold.theme-radio-card-checked,
.theme-popover .theme-gold.theme-radio-card:hover .theme-radio-card-mask,
.theme-popover .theme-gold.theme-radio-card-checked .theme-radio-card-mask {
  border-color: rgb(var(--gold-6));
}

.theme-popover .theme-gold.theme-radio-card-checked {
  background-color: var(--color-gold-light-1);
}

.theme-popover .theme-gold.theme-radio-card-checked .theme-radio-card-mask-dot {
  background-color: rgb(var(--gold-6));
}
</style>
