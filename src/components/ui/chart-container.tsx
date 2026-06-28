import type { ReactNode, ComponentType } from "react";
import { cn } from "../../lib/utils";
import { Spinner } from "./spinner";
import { EmptyState } from "./empty-state";

export interface ChartContainerProps {
  title?: string;
  subtitle?: string;
  children?: ReactNode;
  actions?: ReactNode;
  isLoading?: boolean;
  isEmpty?: boolean;
  emptyMessage?: string;
  emptyIcon?: ComponentType<{ className?: string }>;
  className?: string;
  bodyClassName?: string;
}

function ChartContainer({
  title,
  subtitle,
  children,
  actions,
  isLoading,
  isEmpty,
  emptyMessage = "No data available",
  emptyIcon,
  className,
  bodyClassName,
}: ChartContainerProps) {
  return (
    <div
      className={cn(
        "rounded-lg border border-border bg-surface-1",
        className,
      )}
    >
      {(title || subtitle || actions) && (
        <div className="flex items-start justify-between border-b border-border px-4 py-3">
          <div className="min-w-0">
            {title && (
              <h3 className="text-sm font-semibold text-text-primary truncate">
                {title}
              </h3>
            )}
            {subtitle && (
              <p className="text-xs text-text-muted truncate">{subtitle}</p>
            )}
          </div>
          {actions && (
            <div className="flex items-center gap-2 ml-4 shrink-0">
              {actions}
            </div>
          )}
        </div>
      )}
      <div className={cn("p-4", bodyClassName)}>
        {isLoading ? (
          <div className="flex h-64 items-center justify-center">
            <Spinner size="lg" />
          </div>
        ) : isEmpty ? (
          <EmptyState
            icon={emptyIcon}
            title={emptyMessage}
          />
        ) : (
          children
        )}
      </div>
    </div>
  );
}

export { ChartContainer };
