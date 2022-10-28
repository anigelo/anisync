<script setup lang="ts">
import Config from './app-settings/Config.vue'
import SyncFolders from './app-settings/SyncFolders.vue'
import Filters from './app-settings/Filters.vue'
import type {ApiConfig,ApiSyncDir,ApiFilters} from './app-settings/ApiModels';
import {ref} from "vue";

const apiConfig: ApiConfig = await fetch("/api/config").then(r => r.json());
const apiFilters: ApiFilters = await fetch("/api/filters").then(r => r.json());
const apiSyncs: ApiSyncDir[] = await fetch("/api/syncs").then(r => r.json());

let newConfig = apiConfig;
let newSyncs = apiSyncs;
let newFilters = apiFilters;

let loading = ref(false);
const onSubmit = async () => {
  loading.value = true;

  const requestOptions = {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(newConfig)
  };
  await fetch("/api/config", requestOptions)
    .then(response => console.log(response));

  requestOptions.body = JSON.stringify(newSyncs);
  await fetch("/api/syncs", requestOptions)
    .then(response => console.log(response));

  requestOptions.body = JSON.stringify(newFilters);
  await fetch("/api/filters", requestOptions)
    .then(response => console.log(response));

  loading.value = false;
}

const onConfigChange = (config: ApiConfig) => {
  newConfig = config;
}
const onSyncsChange = (syncs: ApiSyncDir[]) => {
  newSyncs = syncs;
}
const onFiltersChange = (filters: ApiFilters) => {
  newFilters = filters;
}
</script>

<template>
  <i-form @submit="onSubmit">
    <Config :config="apiConfig" @change="onConfigChange" />
    <SyncFolders :syncs="apiSyncs" @change="onSyncsChange" />
    <Filters :filters="apiFilters" @change="onFiltersChange" />
    <i-container>
      <i-form-group>
        <i-button type="submit" :loading="loading">Submit</i-button>
      </i-form-group>
    </i-container>
  </i-form>
</template>