<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
    <DashboardNavbar />
    <div class="flex">
      <DashboardSidebar />
      <main class="flex-1 p-6 ml-64">
        <slot />
      </main>
    </div>
  </div>
</template>
