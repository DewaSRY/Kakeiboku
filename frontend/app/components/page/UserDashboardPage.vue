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
      <div class="mb-8">
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">
          {{ $t("dashboard.moneyStash") }}
        </h1>
        
        <!-- Total Save & Spend Row -->
        <div class="flex gap-4 mb-6">
          <div class="flex items-center gap-2">
            <span class="text-sm text-gray-600 dark:text-gray-400">{{ $t("dashboard.totalSave") }}</span>
            <div class="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded">
              <span class="font-medium text-gray-900 dark:text-white">{{ formatCurrency(moneyStash?.money_stash.total_save || 0) }}</span>
            </div>
          </div>
          <div class="flex items-center gap-2">
            <span class="text-sm text-gray-600 dark:text-gray-400">{{ $t("dashboard.totalSpend") }}</span>
            <div class="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded">
              <span class="font-medium text-gray-900 dark:text-white">{{ formatCurrency(moneyStash?.money_stash.total_spend || 0) }}</span>
            </div>
          </div>
        </div>

        <!-- Category Percentages -->
        <div class="flex flex-wrap gap-4 mb-6 border-b border-gray-200 dark:border-gray-700 pb-4">
          <div 
            v-for="category in moneyStash?.branch_category_percentages || []" 
            :key="category.branch_category_name"
            class="text-center"
          >
            <p class="text-xs text-gray-500 dark:text-gray-400">{{ category.branch_category_name }}</p>
            <p class="text-lg font-semibold text-gray-900 dark:text-white">{{ category.total_percent }} %</p>
          </div>
        </div>

        <!-- Main Branch and Actions -->
        <div class="flex flex-wrap items-center gap-4">
          <div class="flex items-center gap-2">
            <span class="text-sm text-gray-600 dark:text-gray-400">{{ $t("dashboard.mainBranch") }}</span>
            <div class="px-4 py-2 bg-amber-400 rounded font-bold text-gray-900">
              RP. {{ formatNumber(moneyStash?.money_stash.main_branch || 0) }}
            </div>
          </div>
          <div class="flex flex-wrap gap-2">
            <UButton variant="outline" color="neutral" size="sm">
              {{ $t("dashboard.deposite") }}
            </UButton>
            <UButton variant="outline" color="neutral" size="sm">
              {{ $t("dashboard.alocated") }}
            </UButton>
            <UButton variant="outline" color="neutral" size="sm">
              {{ $t("dashboard.spend") }}
            </UButton>
            <UButton variant="outline" color="neutral" size="sm" class="w-full sm:w-auto" @click="showCreateBasket = true">
              {{ $t("dashboard.createBranchBasket") }}
            </UButton>
          </div>
        </div>
      </div>

      <!-- Allocated Money Section -->
      <div class="mb-8">
        <div class="flex items-center justify-between mb-4">
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
              {{ $t("dashboard.filterByDate") }}
            </UButton>
          </div>
        </div>

        <!-- Bar Chart -->
        <div class="bg-white dark:bg-gray-800 rounded-lg p-4 min-h-75">
          <VisXYContainer :data="chartData" :height="280">
            <VisGroupedBar
              :x="(d: ChartDataItem) => d.index"
              :y="(d: ChartDataItem) => d.total"
              :color="'#86efac'"
              :roundedCorners="4"
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

onMounted(() => {
  filterThisMonth();
});
</script>
