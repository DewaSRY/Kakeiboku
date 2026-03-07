<template>
  <UForm
    :schema="LoginPayloadSchema"
    :state="formState"
    @submit="handleSubmit"
    class="space-y-4"
  >
    <UiEmailInputUi
      v-model="formState.email"
      :label="$t('common.email')"
      name="email"
      :placeholder="$t('auth.emailPlaceholder')"
    />

    <UiPasswordInputUi
      v-model="formState.password"
      :label="$t('common.password')"
      name="password"
      :placeholder="$t('auth.passwordPlaceholder')"
    />

    <UiSubmitButtonUi :loading="isLoading">
      {{ $t("common.signIn") }}
    </UiSubmitButtonUi>
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
