<script setup lang="ts">
import AppSettings from './AppSettings.vue'
</script>

<template>
  <i-layout>
    <i-layout-content>
      <Suspense>
        <AppSettings />
      </Suspense>
    </i-layout-content>
  </i-layout>
</template>
