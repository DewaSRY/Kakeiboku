<template>
  <UDropdownMenu :items="localeItems">
    <UButton
      icon="i-lucide-languages"
      color="neutral"
      variant="ghost"
      :aria-label="$t('language.title')"
    />
  </UDropdownMenu>
</template>

<script setup lang="ts">
const { locale, locales, setLocale } = useI18n()

const localeItems = computed(() => [
  (locales.value as Array<{ code: "en" | "id"; name: string }>).map((loc) => ({
    label: loc.name,
    icon: locale.value === loc.code ? 'i-lucide-check' : undefined,
    onSelect: () => setLocale(loc.code ),
  })),
])
</script>
