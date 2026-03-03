<template>
  <UForm
    :schema="RegisterPayloadSchema"
    :state="formState"
    @submit="handleSubmit"
    class="space-y-4"
  >
    <div class="grid grid-cols-2 gap-4">
      <UFormField :label="$t('auth.firstName')" name="first_name">
        <UInput
          v-model="formState.first_name"
          :placeholder="$t('auth.firstNamePlaceholder')"
          size="lg"
        />
      </UFormField>

      <UFormField :label="$t('auth.lastName')" name="last_name">
        <UInput
          v-model="formState.last_name"
          :placeholder="$t('auth.lastNamePlaceholder')"
          size="lg"
        />
      </UFormField>
    </div>

    <UFormField :label="$t('common.email')" name="email">
      <UInput
        v-model="formState.email"
        type="email"
        :placeholder="$t('auth.emailPlaceholder')"
        size="lg"
      />
    </UFormField>

    <UFormField :label="$t('common.password')" name="password">
      <UInput
        v-model="formState.password"
        type="password"
        :placeholder="$t('auth.minPasswordLength')"
        size="lg"
      />
    </UFormField>

    <UButton type="submit" color="primary" size="lg" block :loading="isLoading">
      {{ $t("auth.createAccount") }}
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
  RegisterPayloadSchema,
  type RegisterPayload,
  type UserProfile,
} from "~/dtos";

const { t } = useI18n();
const toast = useToast();

const formState = reactive<RegisterPayload>({
  first_name: "",
  last_name: "",
  email: "",
  password: "",
});

const isLoading = ref(false);
const error = ref<string | null>(null);

async function handleSubmit() {
  isLoading.value = true;
  error.value = null;

  try {
    await $fetch<UserProfile>("/api/auth/register", {
      method: "POST",
      body: formState,
    });
    toast.add({ title: t("auth.registerSuccess"), color: "success" });
    navigateTo("/user/dashboard");
  } catch (e: any) {
    error.value = e.response?.data?.message || t("auth.registerFailed");
  } finally {
    isLoading.value = false;
  }
}
</script>
