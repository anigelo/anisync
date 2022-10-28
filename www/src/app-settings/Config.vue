<script setup lang="ts">
import type {ApiConfig} from "./ApiModels";

interface Props {
  config?: ApiConfig
}

const {config = {}} = defineProps<Props>();
const emit = defineEmits<{(e: 'change', config: ApiConfig): void}>();
</script>

<template>
  <i-container>
    <i-row>
      <i-column>
        <i-form-group>
          <i-form-label>MEGA User</i-form-label>
          <i-input v-model="config.MEGA_USER" placeholder="patata@email.com" @change="emit('change', config)" />
        </i-form-group>
      </i-column>
      <i-column>
        <i-form-group>
          <i-form-label>MEGA Password</i-form-label>
          <i-input v-model="config.MEGA_PWD" placeholder="p4ssWord" />
        </i-form-group>
      </i-column>
    </i-row>
    <i-row>
      <i-column>
        <i-form-group>
          <i-form-label>Download Folder</i-form-label>
          <i-input v-model="config.DOWNLOAD_FOLDER" placeholder="C:\user\Downloads" />
        </i-form-group>
      </i-column>
      <i-column>
        <i-form-group>
          <i-form-label>Local Media Root</i-form-label>
          <i-input v-model="config.LOCAL_MEDIA_ROOT" placeholder="/media" />
        </i-form-group>
      </i-column>
      <i-column>
        <i-form-group>
          <i-form-label>Remote Media Root</i-form-label>
          <i-input v-model="config.REMOTE_MEDIA_ROOT" placeholder="//from:user@mega.nz/Folder" />
        </i-form-group>
      </i-column>
    </i-row>
    <i-row>
      <i-column>
        <i-form-group>
          <i-form-label>Auto-scan interval (in seconds)</i-form-label>
          <i-input v-model="config.SCAN_INTERVAL" placeholder="86400" />
        </i-form-group>
      </i-column>
    </i-row>
  </i-container>
</template>