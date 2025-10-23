<template>
  <div class="page">
    <router-view />
  </div>
</template>

<style>
html,
body,
#app {
  margin: 0;
  height: 100%;
}
.page {
  min-height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f5f7fa;
  padding: 24px;
}
</style>
