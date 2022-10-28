<script setup lang="ts">
import { computed, reactive } from 'vue';
import type {ApiFilters} from "./ApiModels";

interface Props {
  filters?: ApiFilters
}

const {filters = {contains:[],not_contains:[]}} = defineProps<Props>();
const emit = defineEmits<{(e: 'change', config: ApiFilters): void}>();
interface Filter {
  contains: {id: Number, label: String},
  value: String|null
}

const options = computed(() => [
  { id: 1, label: 'Contains' },
  { id: 2, label: 'Not Contains' },
]);
const localFilters: Filter[] = reactive(
  filters.contains.map<Filter>(f => ({contains: options.value[0], value: f}))
    .concat(filters.not_contains.map<Filter>(f => ({contains: options.value[1], value: f})))
);

const addRow = () => {
  localFilters.push({contains: options.value[0], value: null})
}
const deleteRow = (index: number) => {
  localFilters.splice(index, 1);
  onChange();
}

const onChange = () => {
  const mapped: ApiFilters = {
    contains: localFilters
      .filter(f => f.contains.id == 1 && f.value)
      .map(f => f.value!),
    not_contains: localFilters
      .filter(f => f.contains.id == 2 && f.value)
      .map(f => f.value!)
  };
  emit('change', mapped);
}
</script>

<template>
  <i-container>
    <i-layout>
      <i-layout-header><h3>Filters</h3></i-layout-header>
      <i-layout-content>
        <i-table>
          <thead>
            <tr>
              <th>Contains</th>
              <th>Filter</th>
              <th><i-button size="sm" @click="addRow" type="button" block><i-icon name="ink-plus" /></i-button></th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(filter, index) in localFilters">
              <td><i-select v-model="filter.contains" :options="options" v-on:update:modelValue="onChange" /></td>
              <td><i-input v-model="filter.value" placeholder="1080p" @change="onChange" /></td>
              <td><i-button @click="deleteRow(index)" type="button" block><i-icon name="ink-minus" /></i-button></td>
            </tr>
          </tbody>
        </i-table>
      </i-layout-content>
    </i-layout>
  </i-container>
</template>