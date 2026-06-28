import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "../../lib/utils";

const badgeVariants = cva(
  "inline-flex items-center gap-1.5 rounded px-2 py-0.5 text-xs font-medium",
  {
    variants: {
      variant: {
        default: "bg-primary/15 text-primary",
        primary: "bg-primary/15 text-primary",
        success: "bg-success/15 text-success",
        warning: "bg-warning/15 text-warning",
        danger: "bg-danger/15 text-danger",
        info: "bg-info/15 text-info",
        neutral: "bg-surface-2 text-text-muted",
      },
      size: {
        sm: "px-1.5 py-0.5 text-[10px]",
        md: "px-2 py-0.5 text-xs",
      },
    },
    defaultVariants: {
      variant: "primary",
      size: "md",
    },
  }
);

export interface BadgeProps
  extends VariantProps<typeof badgeVariants> {
  className?: string;
  children?: React.ReactNode;
  dot?: boolean;
}

function Badge({ variant, size, dot, className, children }: BadgeProps) {
  return (
    <span className={cn(badgeVariants({ variant, size, className }))}>
      {dot && (
        <span
          className={cn(
            "size-1.5 rounded-full",
            variant === "primary" && "bg-primary",
            variant === "success" && "bg-success",
            variant === "warning" && "bg-warning",
            variant === "danger" && "bg-danger",
            variant === "info" && "bg-info",
            variant === "neutral" && "bg-text-muted",
          )}
        />
      )}
      {children}
    </span>
  );
}

export { Badge, badgeVariants };
