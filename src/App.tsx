import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  LayoutDashboard,
  Wallet,
  Target,
  CreditCard,
  TrendingDown,
  BarChart2,
  Shield,
  Users,
  Wrench,
  Briefcase,
  FileText,
  Settings,
  ChevronLeft,
  ChevronRight,
  Bell,
  Search,
  Flame,
} from "lucide-react";

type NavItem = {
  id: string;
  label: string;
  icon: React.ElementType;
  badge?: number;
  section?: string;
};

const NAV_ITEMS: NavItem[] = [
  { id: "dashboard", label: "Life Hub", icon: LayoutDashboard, section: "Overview" },
  { id: "finance", label: "Finance", icon: Wallet, section: "Money" },
  { id: "budget", label: "Budget", icon: Target },
  { id: "debt", label: "Debt", icon: TrendingDown },
  { id: "credit", label: "Credit", icon: CreditCard },
  { id: "investments", label: "Investments", icon: BarChart2, section: "Assets" },
  { id: "goals", label: "Goals", icon: Target },
  { id: "vault", label: "Vault", icon: Shield, section: "Secure" },
  { id: "crm", label: "Contacts", icon: Users },
  { id: "maintenance", label: "Maintenance", icon: Wrench, section: "Life" },
  { id: "business", label: "Business", icon: Briefcase },
  { id: "reports", label: "Reports", icon: FileText, section: "Reports" },
  { id: "settings", label: "Settings", icon: Settings, section: "System" },
];

function Sidebar({
  activeId,
  onSelect,
  collapsed,
  onToggle,
}: {
  activeId: string;
  onSelect: (id: string) => void;
  collapsed: boolean;
  onToggle: () => void;
}) {
  let lastSection = "";

  return (
    <motion.aside
      animate={{ width: collapsed ? 64 : 220 }}
      transition={{ duration: 0.2, ease: "easeInOut" }}
      className="relative flex flex-col h-full bg-[hsl(220_15%_12%)] border-r border-[hsl(220_15%_20%)] overflow-hidden flex-shrink-0"
    >
      {/* Logo */}
      <div className="flex items-center gap-3 px-4 py-5 border-b border-[hsl(220_15%_20%)]">
        <div className="flex-shrink-0 w-8 h-8 rounded-lg bg-gradient-to-br from-[hsl(220_90%_56%)] to-[hsl(280_80%_60%)] flex items-center justify-center shadow-glow">
          <Flame size={16} className="text-white" />
        </div>
        <AnimatePresence>
          {!collapsed && (
            <motion.div
              initial={{ opacity: 0, x: -8 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -8 }}
              transition={{ duration: 0.15 }}
              className="overflow-hidden"
            >
              <p className="text-sm font-bold text-white leading-none">Phoenix</p>
              <p className="text-[10px] text-[hsl(0_0%_50%)] leading-none mt-0.5">LifeOS</p>
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto py-3 px-2">
        {NAV_ITEMS.map((item) => {
          const showSection = !collapsed && item.section && item.section !== lastSection;
          if (item.section) lastSection = item.section;
          const isActive = activeId === item.id;

          return (
            <div key={item.id}>
              {showSection && (
                <p className="px-2 pt-4 pb-1 text-[10px] font-semibold uppercase tracking-wider text-[hsl(0_0%_40%)]">
                  {item.section}
                </p>
              )}
              <button
                onClick={() => onSelect(item.id)}
                className={`
                  w-full flex items-center gap-3 px-2 py-2 rounded-lg text-sm transition-colors duration-150
                  ${isActive
                    ? "bg-[hsl(220_90%_56%/0.15)] text-[hsl(220_90%_70%)]"
                    : "text-[hsl(0_0%_60%)] hover:bg-[hsl(220_15%_18%)] hover:text-white"
                  }
                `}
                title={collapsed ? item.label : undefined}
              >
                <item.icon
                  size={16}
                  className={`flex-shrink-0 ${isActive ? "text-[hsl(220_90%_60%)]" : ""}`}
                />
                <AnimatePresence>
                  {!collapsed && (
                    <motion.span
                      initial={{ opacity: 0 }}
                      animate={{ opacity: 1 }}
                      exit={{ opacity: 0 }}
                      transition={{ duration: 0.1 }}
                      className="whitespace-nowrap font-medium"
                    >
                      {item.label}
                    </motion.span>
                  )}
                </AnimatePresence>
              </button>
            </div>
          );
        })}
      </nav>

      {/* Collapse toggle */}
      <button
        onClick={onToggle}
        className="absolute top-1/2 -right-3 z-10 w-6 h-6 rounded-full bg-[hsl(220_15%_22%)] border border-[hsl(220_15%_28%)] flex items-center justify-center text-[hsl(0_0%_60%)] hover:text-white transition-colors"
        aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
      >
        {collapsed ? <ChevronRight size={12} /> : <ChevronLeft size={12} />}
      </button>
    </motion.aside>
  );
}

function TopBar({ activeLabel }: { activeLabel: string }) {
  return (
    <header className="h-14 flex items-center justify-between px-5 border-b border-[hsl(220_15%_18%)] bg-[hsl(220_15%_11%/0.8)] backdrop-blur-sm flex-shrink-0">
      <h1 className="text-base font-semibold text-white">{activeLabel}</h1>
      <div className="flex items-center gap-2">
        <button
          className="w-8 h-8 rounded-lg flex items-center justify-center text-[hsl(0_0%_55%)] hover:text-white hover:bg-[hsl(220_15%_18%)] transition-colors"
          aria-label="Search"
        >
          <Search size={15} />
        </button>
        <button
          className="w-8 h-8 rounded-lg flex items-center justify-center text-[hsl(0_0%_55%)] hover:text-white hover:bg-[hsl(220_15%_18%)] transition-colors"
          aria-label="Notifications"
        >
          <Bell size={15} />
        </button>
        <div className="w-7 h-7 rounded-full bg-gradient-to-br from-[hsl(220_90%_56%)] to-[hsl(280_80%_60%)] flex-shrink-0" />
      </div>
    </header>
  );
}

function ComingSoonContent({ module: mod }: { module: string }) {
  return (
    <motion.div
      key={mod}
      initial={{ opacity: 0, y: 12 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.2 }}
      className="flex flex-col items-center justify-center h-full gap-5 text-center"
    >
      <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-[hsl(220_90%_56%/0.2)] to-[hsl(280_80%_60%/0.2)] border border-[hsl(220_90%_56%/0.3)] flex items-center justify-center">
        <Flame size={28} className="text-[hsl(220_90%_65%)]" />
      </div>
      <div>
        <p className="text-lg font-semibold text-white">{mod}</p>
        <p className="text-sm text-[hsl(0_0%_50%)] mt-1 max-w-xs">
          This module is under active development. Check back soon.
        </p>
      </div>
      <div className="flex gap-1.5">
        {[...Array(3)].map((_, i) => (
          <motion.div
            key={i}
            className="w-1.5 h-1.5 rounded-full bg-[hsl(220_90%_56%)]"
            animate={{ opacity: [0.3, 1, 0.3] }}
            transition={{ duration: 1.2, repeat: Infinity, delay: i * 0.2 }}
          />
        ))}
      </div>
    </motion.div>
  );
}

export default function App() {
  const [activeId, setActiveId] = useState("dashboard");
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

  const activeItem = NAV_ITEMS.find((n) => n.id === activeId)!;

  return (
    <div className="flex h-screen overflow-hidden bg-[hsl(220_15%_10%)]">
      <Sidebar
        activeId={activeId}
        onSelect={setActiveId}
        collapsed={sidebarCollapsed}
        onToggle={() => setSidebarCollapsed((c) => !c)}
      />

      <div className="flex flex-col flex-1 min-w-0 overflow-hidden">
        <TopBar activeLabel={activeItem.label} />
        <main className="flex-1 overflow-auto">
          <ComingSoonContent module={activeItem.label} />
        </main>
      </div>
    </div>
  );
}
