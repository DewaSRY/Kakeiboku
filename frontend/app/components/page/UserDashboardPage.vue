<template>
  <UDashboardPanel id="user-dashboard">
    <template #header>
      <UDashboardNavbar title="Home" :ui="{ right: 'gap-3' }">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>
    <template #body>
      <!-- Money Stash Section -->
      <div class="mb-10 animate-fade-in">
        <h1 class="text-2xl font-extrabold text-gray-900 dark:text-white mb-6">
          {{ $t("dashboard.moneyStash") }}
        </h1>
        
        <!-- Total Save & Spend Row -->
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-6">
          <div class="flex items-center gap-3 bg-emerald-50 dark:bg-emerald-950/30 border border-emerald-200 dark:border-emerald-800/40 rounded-2xl px-5 py-4">
            <div class="w-10 h-10 bg-emerald-100 dark:bg-emerald-900/50 rounded-xl flex items-center justify-center">
              <UIcon name="i-lucide-trending-up" class="w-5 h-5 text-emerald-600 dark:text-emerald-400" />
            </div>
            <div>
              <span class="text-xs font-medium text-emerald-600 dark:text-emerald-400 uppercase tracking-wide">{{ $t("dashboard.totalSave") }}</span>
              <p class="text-lg font-bold text-gray-900 dark:text-white">{{ formatCurrency(moneyStash?.money_stash.total_save || 0) }}</p>
            </div>
          </div>
          <div class="flex items-center gap-3 bg-rose-50 dark:bg-rose-950/30 border border-rose-200 dark:border-rose-800/40 rounded-2xl px-5 py-4">
            <div class="w-10 h-10 bg-rose-100 dark:bg-rose-900/50 rounded-xl flex items-center justify-center">
              <UIcon name="i-lucide-trending-down" class="w-5 h-5 text-rose-600 dark:text-rose-400" />
            </div>
            <div>
              <span class="text-xs font-medium text-rose-600 dark:text-rose-400 uppercase tracking-wide">{{ $t("dashboard.totalSpend") }}</span>
              <p class="text-lg font-bold text-gray-900 dark:text-white">{{ formatCurrency(moneyStash?.money_stash.total_spend || 0) }}</p>
            </div>
          </div>
        </div>

        <!-- Category Percentages -->
        <div class="flex flex-wrap gap-3 mb-6 pb-6 border-b border-gray-200 dark:border-gray-800">
          <div 
            v-for="category in moneyStash?.branch_category_percentages || []" 
            :key="category.branch_category_name"
            class="bg-white dark:bg-gray-900 border border-gray-100 dark:border-gray-800 rounded-xl px-4 py-3 text-center min-w-25 card-hover"
          >
            <p class="text-xs font-medium text-gray-500 dark:text-gray-400 mb-1">{{ category.branch_category_name }}</p>
            <p class="text-xl font-bold bg-linear-to-r from-amber-500 to-orange-500 bg-clip-text text-transparent">{{ category.total_percent }}%</p>
          </div>
        </div>

        <!-- Main Branch and Actions -->
        <div class="flex flex-wrap items-center gap-5">
          <div class="flex items-center gap-3">
            <span class="text-sm font-medium text-gray-500 dark:text-gray-400">{{ $t("dashboard.mainBranch") }}</span>
            <div class="px-5 py-2.5 bg-linear-to-r from-amber-400 to-amber-500 rounded-xl font-bold text-gray-900 shadow-lg shadow-amber-400/25 animate-pulse-glow">
              RP. {{ formatNumber(moneyStash?.money_stash.main_branch || 0) }}
            </div>
          </div>
          <div class="flex flex-wrap gap-2">
            <UButton variant="soft" color="success" size="sm" @click="showDeposit = true">
              <UIcon name="i-lucide-plus-circle" class="w-4 h-4 mr-1" />
              {{ $t("dashboard.deposite") }}
            </UButton>
            <UButton variant="soft" color="primary" size="sm" @click="showAllocate = true">
              <UIcon name="i-lucide-arrow-right-left" class="w-4 h-4 mr-1" />
              {{ $t("dashboard.alocated") }}
            </UButton>
            <UButton variant="soft" color="error" size="sm" @click="showSpend = true">
              <UIcon name="i-lucide-minus-circle" class="w-4 h-4 mr-1" />
              {{ $t("dashboard.spend") }}
            </UButton>
            <UButton variant="soft" color="neutral" size="sm" @click="showCreateBasket = true">
              <UIcon name="i-lucide-archive" class="w-4 h-4 mr-1" />
              {{ $t("dashboard.createBranchBasket") }}
            </UButton>
          </div>
        </div>
      </div>

      <!-- Allocated Money Section -->
      <div class="mb-8 animate-fade-in" style="animation-delay: 0.15s;">
        <div class="flex items-center justify-between mb-5">
          <h2 class="text-xl font-bold text-gray-900 dark:text-white">
            {{ $t("dashboard.alocatedMoney") }}
          </h2>
          <div class="flex gap-2">
            <UButton 
              :variant="dateFilter === 'this_month' ? 'solid' : 'outline'" 
              color="neutral" 
              size="sm"
              @click="filterThisMonth"
            >
              {{ $t("dashboard.thisMonth") }}
            </UButton>
            <UButton variant="outline" color="neutral" size="sm" @click="showDateFilter = true">
              <UIcon name="i-lucide-calendar" class="w-4 h-4 mr-1" />
              {{ $t("dashboard.filterByDate") }}
            </UButton>
          </div>
        </div>

        <!-- Bar Chart -->
        <div class="bg-white dark:bg-gray-900 rounded-2xl border border-gray-100 dark:border-gray-800 p-5 min-h-75">
          <VisXYContainer :data="chartData" :height="280">
            <VisGroupedBar
              :x="(d: ChartDataItem) => d.index"
              :y="(d: ChartDataItem) => d.total"
              :color="'#86efac'"
              :roundedCorners="8"
              :barPadding="0.3"
            />
            <VisAxis type="x" :tickFormat="(i: number) => chartData[i]?.name || ''" />
            <VisAxis type="y" :tickFormat="(v: number) => formatCompactNumber(v)" />
          </VisXYContainer>
        </div>
      </div>

      <!-- Create Basket Modal -->
      <UModal v-model:open="showCreateBasket">
        <template #content>
          <CreateBasketForm
            @success="handleBasketCreated"
            @cancel="showCreateBasket = false"
          />
        </template>
      </UModal>

      <!-- Deposit Modal -->
      <UModal v-model:open="showDeposit">
        <template #content>
          <DepositForm
            @success="handleTransactionSuccess('deposit')"
            @cancel="showDeposit = false"
          />
        </template>
      </UModal>

      <!-- Allocate Modal -->
      <UModal v-model:open="showAllocate">
        <template #content>
          <AllocateForm
            @success="handleTransactionSuccess('allocate')"
            @cancel="showAllocate = false"
          />
        </template>
      </UModal>

      <!-- Spend Modal -->
      <UModal v-model:open="showSpend">
        <template #content>
          <SpendForm
            @success="handleTransactionSuccess('spend')"
            @cancel="showSpend = false"
          />
        </template>
      </UModal>

      <!-- Date Filter Modal -->
      <UModal v-model:open="showDateFilter">
        <template #content>
          <div class="p-6">
            <h3 class="text-lg font-semibold mb-4">{{ $t("dashboard.filterByDate") }}</h3>
            <div class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  {{ $t("dashboard.startDate") }}
                </label>
                <UInput v-model="startDate" type="date" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  {{ $t("dashboard.endDate") }}
                </label>
                <UInput v-model="endDate" type="date" />
              </div>
              <div class="flex justify-end gap-2">
                <UButton variant="outline" color="neutral" @click="showDateFilter = false">
                  {{ $t("common.cancel") }}
                </UButton>
                <UButton color="primary" @click="applyDateFilter">
                  {{ $t("common.apply") }}
                </UButton>
              </div>
            </div>
          </div>
        </template>
      </UModal>
    </template>
  </UDashboardPanel>
</template>

<script setup lang="ts">
import { VisXYContainer, VisGroupedBar, VisAxis } from "@unovis/vue";
import type { UserMoneyStashResponse, BranchSummaryResponse } from "~/dtos";
import { useDashboard } from "#imports";

definePageMeta({
  layout: "dashboard",
});

interface ChartDataItem {
  index: number;
  name: string;
  total: number;
}

const { t } = useI18n();
const dashboardService = useDashboard();
const toast = useToast();

const moneyStash = ref<UserMoneyStashResponse | null>(null);
const branchSummary = ref<BranchSummaryResponse | null>(null);
const showCreateBasket = ref(false);
const showDeposit = ref(false);
const showAllocate = ref(false);
const showSpend = ref(false);
const showDateFilter = ref(false);
const dateFilter = ref<'this_month' | 'custom'>('this_month');
const startDate = ref('');
const endDate = ref('');

const chartData = computed<ChartDataItem[]>(() => {
  if (!branchSummary.value?.data) return [];
  return branchSummary.value.data.map((item, index) => ({
    index,
    name: item.name,
    total: item.total
  }));
});

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat("id-ID", {
    style: "currency",
    currency: "IDR",
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount);
};

const formatNumber = (amount: number) => {
  return new Intl.NumberFormat("id-ID", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount);
};

const formatCompactNumber = (value: number) => {
  if (value >= 1000000) {
    return `${(value / 1000000).toFixed(1)}M`;
  }
  if (value >= 1000) {
    return `${(value / 1000).toFixed(0)}K`;
  }
  return value.toString();
};

async function loadDashboardData(params?: { start_date?: string; end_date?: string }) {
  try {
    const [stashData, summaryData] = await Promise.all([
      dashboardService.fetchMoneyStash(),
      dashboardService.fetchBranchSummary(params)
    ]);

    moneyStash.value = stashData;
    branchSummary.value = summaryData;
  } catch (error) {
    toast.add({ title: t("common.error"), color: "error" });
  }
}

function filterThisMonth() {
  dateFilter.value = 'this_month';
  const now = new Date();
  const firstDay = new Date(now.getFullYear(), now.getMonth(), 1);
  const lastDay = new Date(now.getFullYear(), now.getMonth() + 1, 0);
  
  loadDashboardData({
    start_date: firstDay.toISOString().split('T')[0],
    end_date: lastDay.toISOString().split('T')[0]
  });
}

function applyDateFilter() {
  if (startDate.value && endDate.value) {
    dateFilter.value = 'custom';
    showDateFilter.value = false;
    loadDashboardData({
      start_date: startDate.value,
      end_date: endDate.value
    });
  }
}

function handleBasketCreated() {
  showCreateBasket.value = false;
  loadDashboardData();
  toast.add({ title: t("baskets.basketCreated"), color: "success" });
}

function handleTransactionSuccess(type: 'deposit' | 'allocate' | 'spend') {
  showDeposit.value = false;
  showAllocate.value = false;
  showSpend.value = false;
  filterThisMonth();
  toast.add({ title: t("transactions.transactionCreated"), color: "success" });
}

onMounted(() => {
  filterThisMonth();
});
</script>
