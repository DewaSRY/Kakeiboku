<template>
  <UForm
    :schema="RegisterPayloadSchema"
    :state="formState"
    @submit="handleSubmit"
    class="space-y-4"
  >
    <div class="grid grid-cols-2 gap-4">
      <UiTextInputUi
        v-model="formState.first_name"
        :label="$t('auth.firstName')"
        name="first_name"
        :placeholder="$t('auth.firstNamePlaceholder')"
        icon="i-lucide-user"
      />

      <UiTextInputUi
        v-model="formState.last_name"
        :label="$t('auth.lastName')"
        name="last_name"
        :placeholder="$t('auth.lastNamePlaceholder')"
        icon="i-lucide-user"
      />
    </div>

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
      :placeholder="$t('auth.minPasswordLength')"
    />

    <UiSubmitButtonUi :loading="isLoading">
      {{ $t("auth.createAccount") }}
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
