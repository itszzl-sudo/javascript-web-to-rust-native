<script setup lang="ts">
import { ref, computed } from 'vue'

const title = ref('Vue Compile Demo')
const count = ref(0)
const inputValue = ref('')
const isActive = ref(true)

const doubledCount = computed(() => count.value * 2)

function increment() {
  count.value++
}

function updateTitle(newTitle: string) {
  title.value = newTitle
}

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement
  inputValue.value = target.value
}
</script>

<template>
  <div id="app" :class="{ active: isActive }">
    <h1>{{ title }}</h1>
    <p>Count: {{ count }}, Doubled: {{ doubledCount }}</p>

    <button
      class="btn btn-primary"
      :class="{ active: isActive }"
      @click="increment"
    >
      Click me ({{ count }})
    </button>

    <div class="input-wrapper">
      <input
        v-model="inputValue"
        type="text"
        :placeholder="title"
        @input="handleInput"
      />
      <p>Input value: {{ inputValue }}</p>
    </div>

    <div class="card" v-if="count > 0">
      <h2>Card #{{ count }}</h2>
      <p>This card appears when count > 0</p>
    </div>
  </div>
</template>

<style scoped>
.active {
  color: blue;
}

.btn {
  padding: 8px 16px;
  border-radius: 4px;
}

.btn-primary {
  background: #42b983;
  color: white;
}

.input-wrapper {
  margin: 16px 0;
}

.card {
  border: 1px solid #ddd;
  padding: 16px;
  margin-top: 16px;
  border-radius: 8px;
}
</style>