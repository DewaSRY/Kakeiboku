<template>
  <UForm
    :schema="LoginPayloadSchema"
    :state="formState"
    @submit="handleSubmit"
    class="space-y-4"
  >
    <UFormField :label="$t('common.email')" name="email">
      <UInput
        v-model="formState.email"
        type="email"
        :placeholder="$t('auth.emailPlaceholder')"
        size="lg"
        class="w-full"
      />
    </UFormField>

    <UFormField :label="$t('common.password')" name="password">
      <UInput
        v-model="formState.password"
        type="password"
        :placeholder="$t('auth.passwordPlaceholder')"
        size="lg"
        class="w-full"
      />
    </UFormField>

    <UButton type="submit" color="primary" size="lg" block :loading="isLoading">
      {{ $t("common.signIn") }}
    </UButton>
  </UForm>

  <UAlert
    v-if="error"
    color="error"
    variant="soft"
    :title="error"
    class="mt-4"
  />
</template>

<script setup lang="ts">
import {
  LoginPayloadSchema,
  type LoginPayload,
  type UserProfile,
} from "~/dtos";

const { t } = useI18n();
const toast = useToast();

const formState = reactive<LoginPayload>({
  email: "",
  password: "",
});

const isLoading = ref(false);
const error = ref<string | null>(null);

async function handleSubmit() {
  isLoading.value = true;
  error.value = null;

  try {
    await $fetch<UserProfile>("/api/auth/login", {
      method: "POST",
      body: formState,
    });

    toast.add({ title: t("auth.loginSuccess"), color: "success" });
    navigateTo("/user/dashboard");
  } catch (e: any) {
    error.value = e.response?.data?.message || t("auth.loginFailed");
  } finally {
    isLoading.value = false;
  }
}
</script>
