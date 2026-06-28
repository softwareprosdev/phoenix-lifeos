import { useState, type ReactNode } from "react";
import { motion } from "framer-motion";
import { cn } from "../../lib/utils";

export interface Tab {
  id: string;
  label: string;
  content?: ReactNode;
  disabled?: boolean;
}

export interface TabsProps {
  tabs: Tab[];
  defaultTab?: string;
  activeTab?: string;
  active?: string;
  onChange?: (tabId: string) => void;
  orientation?: "horizontal" | "vertical";
  className?: string;
  tabListClassName?: string;
  tabPanelClassName?: string;
}

function Tabs({
  tabs,
  defaultTab,
  activeTab,
  active: activeProp,
  onChange,
  orientation = "horizontal",
  className,
  tabListClassName,
  tabPanelClassName,
}: TabsProps) {
  const controlledTab = activeTab ?? activeProp;
  const [internalTab, setInternalTab] = useState(defaultTab || tabs[0]?.id || "");
  const isControlled = controlledTab !== undefined;
  const currentTab = isControlled ? controlledTab : internalTab;
  const activeTabData = tabs.find((t) => t.id === currentTab);

  const handleTabClick = (tabId: string) => {
    if (!isControlled) setInternalTab(tabId);
    onChange?.(tabId);
  };

  return (
    <div
      className={cn(
        orientation === "vertical" ? "flex gap-6" : "flex flex-col",
        className,
      )}
    >
      <div
        className={cn(
          "relative flex",
          orientation === "vertical"
            ? "flex-col border-l border-border"
            : "border-b border-border",
          tabListClassName,
        )}
        role="tablist"
        aria-orientation={orientation}
      >
        {tabs.map((tab) => (
          <button
            key={tab.id}
            role="tab"
            aria-selected={currentTab === tab.id}
            aria-disabled={tab.disabled}
            disabled={tab.disabled}
            onClick={() => handleTabClick(tab.id)}
            className={cn(
              "relative whitespace-nowrap px-4 py-2.5 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary",
              tab.disabled && "cursor-not-allowed opacity-50",
              currentTab === tab.id
                ? "text-primary"
                : "text-text-muted hover:text-text-primary",
            )}
          >
            {tab.label}
            {currentTab === tab.id && (
              <motion.div
                layoutId="tab-indicator"
                className={cn(
                  "absolute bg-primary",
                  orientation === "horizontal"
                    ? "bottom-0 left-0 right-0 h-0.5"
                    : "left-0 top-0 bottom-0 w-0.5",
                )}
                transition={{ type: "spring", stiffness: 500, damping: 35 }}
              />
            )}
          </button>
        ))}
      </div>
      {activeTabData?.content && (
        <div
          role="tabpanel"
          aria-labelledby={activeTabData.id}
          className={cn("pt-4", tabPanelClassName)}
        >
          {activeTabData.content}
        </div>
      )}
    </div>
  );
}

export { Tabs };
