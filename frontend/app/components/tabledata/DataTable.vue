<template>
  <div>
    <UTable :columns="columns" :data="data">
      <template v-for="(_, name) in $slots" :key="name" #[name]="slotProps">
        <slot :name="name" v-bind="slotProps" />
      </template>
    </UTable>
    
    <TableEmptyState 
      v-if="data.length === 0"
      :icon="emptyIcon"
      :message="emptyMessage"
    />
  </div>
</template>

<script setup lang="ts" generic="T">
import type { TableColumn } from '@nuxt/ui'

interface Props {
  columns: TableColumn<T>[]
  data: T[]
  emptyIcon?: string
  emptyMessage?: string
}

withDefaults(defineProps<Props>(), {
  emptyIcon: 'i-lucide-file-text',
  emptyMessage: 'No data found.'
})
</script>
