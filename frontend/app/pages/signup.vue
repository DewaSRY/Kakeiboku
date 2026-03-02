<template>
  <div class="w-full max-w-md p-8 bg-white rounded-2xl shadow-xl">
    <div class="text-center mb-8">
      <div class="w-12 h-12 bg-amber-400 rounded-xl flex items-center justify-center mx-auto mb-4">
        <span class="text-white font-bold text-xl">K</span>
      </div>
      <h1 class="text-2xl font-bold text-gray-900">Create Account</h1>
      <p class="text-gray-600 mt-2">Start your Kakeibo journey today</p>
    </div>

    <UForm :schema="RegisterPayloadSchema" :state="formState" @submit="handleSubmit" class="space-y-4">
      <div class="grid grid-cols-2 gap-4">
        <UFormField label="First Name" name="first_name">
          <UInput 
            v-model="formState.first_name" 
            placeholder="John"
            size="lg"
          />
        </UFormField>

        <UFormField label="Last Name" name="last_name">
          <UInput 
            v-model="formState.last_name" 
            placeholder="Doe"
            size="lg"
          />
        </UFormField>
      </div>

      <UFormField label="Email" name="email">
        <UInput 
          v-model="formState.email" 
          type="email" 
          placeholder="you@example.com"
          size="lg"
        />
      </UFormField>

      <UFormField label="Password" name="password">
        <UInput 
          v-model="formState.password" 
          type="password" 
          placeholder="Min. 6 characters"
          size="lg"
        />
      </UFormField>

      <UButton 
        type="submit" 
        color="primary" 
        size="lg" 
        block 
        :loading="isLoading"
      >
        Create Account
      </UButton>
    </UForm>

    <div class="mt-6 text-center">
      <p class="text-gray-600">
        Already have an account?
        <NuxtLink to="/signin" class="text-amber-500 hover:text-amber-600 font-medium">
          Sign in
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
import { RegisterPayloadSchema, type RegisterPayload } from '~/dtos'
import { useAuthService } from '~/services'

definePageMeta({
  layout: 'auth'
})

const authService = useAuthService()
const toast = useToast()

const formState = reactive<RegisterPayload>({
  first_name: '',
  last_name: '',
  email: '',
  password: ''
})

const isLoading = ref(false)
const error = ref<string | null>(null)

async function handleSubmit() {
  isLoading.value = true
  error.value = null
  
  try {
    await authService.register(formState)
    toast.add({ title: 'Account created successfully!', color: 'success' })
    navigateTo('/user/dashboard')
  } catch (e: any) {
    error.value = e.response?.data?.message || 'Registration failed. Please try again.'
  } finally {
    isLoading.value = false
  }
}
</script>
