<template>
  <div class="w-full max-w-md p-8 bg-white rounded-2xl shadow-xl">
    <div class="text-center mb-8">
      <div class="w-12 h-12 bg-amber-400 rounded-xl flex items-center justify-center mx-auto mb-4">
        <span class="text-white font-bold text-xl">K</span>
      </div>
      <h1 class="text-2xl font-bold text-gray-900">Welcome Back</h1>
      <p class="text-gray-600 mt-2">Sign in to your Kakeibo account</p>
    </div>

    <UForm :schema="LoginPayloadSchema" :state="formState" @submit="handleSubmit" class="space-y-4">
      <UFormField label="Email" name="email">
        <UInput 
          v-model="formState.email" 
          type="email" 
          placeholder="you@example.com"
          size="lg"
          class="w-full"
        />
      </UFormField>

      <UFormField label="Password" name="password">
        <UInput 
          v-model="formState.password" 
          type="password" 
          placeholder="••••••••"
          size="lg"
          class="w-full"
        />
      </UFormField>

      <UButton 
        type="submit" 
        color="primary" 
        size="lg" 
        block 
        :loading="isLoading"
      >
        Sign In
      </UButton>
    </UForm>

    <div class="mt-6 text-center">
      <p class="text-gray-600">
        Don't have an account?
        <NuxtLink to="/signup" class="text-amber-500 hover:text-amber-600 font-medium">
          Sign up
        </NuxtLink>
      </p>
    </div>

    <UAlert 
      v-if="error" 
      color="error" 
      variant="soft" 
      :title="error"
      class="mt-4"
    />
  </div>
</template>

<script setup lang="ts">
import { LoginPayloadSchema, type LoginPayload } from '~/dtos'
import { useAuthService } from '~/services'

definePageMeta({
  layout: 'auth'
})

const authService = useAuthService()
const toast = useToast()

const formState = reactive<LoginPayload>({
  email: '',
  password: ''
})

const isLoading = ref(false)
const error = ref<string | null>(null)

async function handleSubmit() {
  isLoading.value = true
  error.value = null
  
  try {
    await authService.login(formState)
    toast.add({ title: 'Welcome back!', color: 'success' })
    navigateTo('/user/dashboard')
  } catch (e: any) {
    error.value = e.response?.data?.message || 'Login failed. Please try again.'
  } finally {
    isLoading.value = false
  }
}
</script>
