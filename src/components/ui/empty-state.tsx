import type { ReactNode, ComponentType } from "react";
import { cn } from "../../lib/utils";
import { Button, type ButtonProps } from "./Button";

export interface EmptyStateProps {
  icon?: ComponentType<{ className?: string }>;
  title?: string;
  description?: string;
  action?: {
    label: string;
    onClick: () => void;
    variant?: ButtonProps["variant"];
  };
  className?: string;
  children?: ReactNode;
}

function EmptyState({
  icon: Icon,
  title = "No data",
  description,
  action,
  className,
  children,
}: EmptyStateProps) {
  return (
    <div
      className={cn(
        "flex flex-col items-center justify-center py-12 text-center",
        className,
      )}
    >
      {Icon && (
        <div className="mb-4 flex size-12 items-center justify-center rounded-full bg-surface-2">
          <Icon className="size-5 text-text-muted" aria-hidden="true" />
        </div>
      )}
      {title && (
        <h3 className="text-sm font-semibold text-text-primary">{title}</h3>
      )}
      {description && (
        <p className="mt-1 max-w-sm text-xs text-text-muted">{description}</p>
      )}
      {action && (
        <Button
          variant={action.variant || "secondary"}
          size="sm"
          className="mt-4"
          onClick={action.onClick}
        >
          {action.label}
        </Button>
      )}
      {children}
    </div>
  );
}

export { EmptyState };
