<template>
  <el-input
    type="text"
    :placeholder="placeholder"
    :model-value="modelValue"
    autocomplete="off"
    inputmode="text"
    class="fake-pwd"
    @input="$emit('update:modelValue', $event)"
  />
</template>

<script setup>
// eslint-disable-next-line no-unused-vars
const props = defineProps({
  placeholder: { type: String, default: '' },
  modelValue: { type: String, default: '' },
});
// eslint-disable-next-line no-unused-vars
const emit = defineEmits(['update:modelValue']);
</script>

<style scoped>
.fake-pwd :deep(input) {
  -webkit-text-security: disc;
  -moz-text-security: disc;
  text-security: disc;
}
</style>
