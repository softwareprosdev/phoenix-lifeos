import { lazy, Suspense } from "react";
import { createBrowserRouter, Navigate } from "react-router-dom";
import AppLayout from "@/App";

const DashboardPage = lazy(() => import("@/features/dashboard/pages/DashboardPage"));
const FinancePage = lazy(() => import("@/features/finance/pages/FinancePage"));
const BudgetPage = lazy(() => import("@/features/budget/pages/BudgetPage"));
const DebtPage = lazy(() => import("@/features/debt/pages/DebtPage"));
const CreditPage = lazy(() => import("@/features/credit/pages/CreditPage"));
const InvestmentsPage = lazy(() => import("@/features/investments/pages/InvestmentsPage"));
const GoalsPage = lazy(() => import("@/features/goals/pages/GoalsPage"));
const VaultPage = lazy(() => import("@/features/vault/pages/VaultPage"));
const CRMPage = lazy(() => import("@/features/crm/pages/CRMPage"));
const MaintenancePage = lazy(() => import("@/features/maintenance/pages/MaintenancePage"));
const BusinessPage = lazy(() => import("@/features/business/pages/BusinessPage"));
const ReportsPage = lazy(() => import("@/features/reports/pages/ReportsPage"));
const SettingsPage = lazy(() => import("@/features/settings/pages/SettingsPage"));

function PageLoader() {
  return (
    <div className="flex items-center justify-center h-full">
      <div className="flex gap-1.5">
        {[...Array(3)].map((_, i) => (
          <div
            key={i}
            className="w-2 h-2 rounded-full bg-[hsl(220_90%_56%)] animate-bounce"
            style={{ animationDelay: `${i * 0.15}s` }}
          />
        ))}
      </div>
    </div>
  );
}

function SuspenseWrapper({ children }: { children: React.ReactNode }) {
  return <Suspense fallback={<PageLoader />}>{children}</Suspense>;
}

export const router = createBrowserRouter([
  {
    path: "/",
    element: <AppLayout />,
    children: [
      { index: true, element: <Navigate to="/dashboard" replace /> },
      {
        path: "dashboard",
        element: (
          <SuspenseWrapper>
            <DashboardPage />
          </SuspenseWrapper>
        ),
      },
      {
        path: "finance",
        element: (
          <SuspenseWrapper>
            <FinancePage />
          </SuspenseWrapper>
        ),
      },
      {
        path: "budget",
        element: (
          <SuspenseWrapper>
            <BudgetPage />
          </SuspenseWrapper>
        ),
      },
      {
        path: "debt",
        element: (
          <SuspenseWrapper>
            <DebtPage />
          </SuspenseWrapper>
        ),
      },
      {
        path: "credit",
        element: (
          <SuspenseWrapper>
            <CreditPage />
          </SuspenseWrapper>
        ),
      },
      {
        path: "investments",
        element: (
          <SuspenseWrapper>
            <InvestmentsPage />
          </SuspenseWrapper>
        ),
      },
      {
        path: "goals",
        element: (
          <SuspenseWrapper>
            <GoalsPage />
          </SuspenseWrapper>
        ),
      },
      {
        path: "vault",
        element: (
          <SuspenseWrapper>
            <VaultPage />
          </SuspenseWrapper>
        ),
      },
      {
        path: "crm",
        element: (
          <SuspenseWrapper>
            <CRMPage />
          </SuspenseWrapper>
        ),
      },
      {
        path: "maintenance",
        element: (
          <SuspenseWrapper>
            <MaintenancePage />
          </SuspenseWrapper>
        ),
      },
      {
        path: "business",
        element: (
          <SuspenseWrapper>
            <BusinessPage />
          </SuspenseWrapper>
        ),
      },
      {
        path: "reports",
        element: (
          <SuspenseWrapper>
            <ReportsPage />
          </SuspenseWrapper>
        ),
      },
      {
        path: "settings",
        element: (
          <SuspenseWrapper>
            <SettingsPage />
          </SuspenseWrapper>
        ),
      },
    ],
  },
]);
