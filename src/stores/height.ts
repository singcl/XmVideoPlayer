// https://pinia.esm.dev/introduction.html
import { defineStore } from 'pinia';
// import { nextTick } from 'vue';

export const useHeightStore = defineStore('height', {
  state: () => {
    return { height: '100vh' };
  },
  // could also be defined as
  // state: () => ({ count: 0 })
  actions: {
    change() {
      this.height = 'calc(100vh - 1px)';
      setTimeout(() => {
        this.height = '100vh';
        console.log('--恢复高度:', this.height);
      }, 0);
    },
  },
});

// You can even use a function (similar to a component setup()) to define a Store for more advanced use cases:
// export const useHeightStore = defineStore('counter', () => {
//   const count = ref(0)
//
//   function increment() {
//     count.value++
//   }
//
//   return {count, increment}
// })
