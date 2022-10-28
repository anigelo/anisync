<script setup lang="ts">
import { reactive, toRaw } from 'vue';
import type {ApiSyncDir} from "./ApiModels";

interface Props {
  syncs?: ApiSyncDir[]
}

const {syncs = []} = defineProps<Props>();
const emit = defineEmits<{(e: 'change', config: ApiSyncDir[]): void}>();

const syncFolders: ApiSyncDir[] = reactive([...syncs]);

const addRow = () => {
  syncFolders.push({remote: "", local: ""})
}
const deleteRow = (index: number) => {
  syncFolders.splice(index, 1);
  emit('change', toRaw(syncFolders));
}
</script>

<template>
  <i-container>
    <i-layout>
      <i-layout-header><h3>Sync Folders</h3></i-layout-header>
      <i-layout-content>
        <i-table>
          <thead>
            <tr>
              <th>Remote</th>
              <th>Local</th>
              <th><i-button size="sm" @click="addRow" type="button" block><i-icon name="ink-plus" /></i-button></th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(syncFolder, index) in syncFolders">
              <td><i-input v-model="syncFolder.remote" placeholder="/Primavera/Danmachi" @change="emit('change', toRaw(syncFolders))" /></td>
              <td><i-input v-model="syncFolder.local" placeholder="anime/Danmachi"  @change="emit('change', toRaw(syncFolders))" /></td>
              <td><i-button @click="deleteRow(index)" type="button" block><i-icon name="ink-minus" /></i-button></td>
            </tr>
          </tbody>
        </i-table>
      </i-layout-content>
    </i-layout>
  </i-container>
</template>