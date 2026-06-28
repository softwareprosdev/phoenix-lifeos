import { motion } from "framer-motion";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "../../lib/utils";

const progressTrackVariants = cva("w-full overflow-hidden rounded-full bg-surface-2", {
  variants: {
    size: {
      sm: "h-1.5",
      md: "h-2.5",
      lg: "h-4",
    },
  },
  defaultVariants: {
    size: "md",
  },
});

const progressBarVariants = cva("h-full rounded-full transition-colors", {
  variants: {
    variant: {
      primary: "bg-primary",
      success: "bg-success",
      warning: "bg-warning",
      danger: "bg-danger",
    },
  },
  defaultVariants: {
    variant: "primary",
  },
});

export interface ProgressBarProps
  extends VariantProps<typeof progressTrackVariants>,
    VariantProps<typeof progressBarVariants> {
  value: number;
  max?: number;
  showLabel?: boolean;
  className?: string;
}

function ProgressBar({
  value,
  max = 100,
  variant,
  size,
  showLabel = false,
  className,
}: ProgressBarProps) {
  const percentage = Math.min(Math.max(0, (value / max) * 100), 100);

  return (
    <div className={cn("w-full", className)}>
      <div
        className={progressTrackVariants({ size })}
        role="progressbar"
        aria-valuenow={value}
        aria-valuemin={0}
        aria-valuemax={max}
      >
        <motion.div
          className={progressBarVariants({ variant })}
          initial={{ width: 0 }}
          animate={{ width: `${percentage}%` }}
          transition={{ duration: 0.5, ease: "easeOut" }}
        />
      </div>
      {showLabel && (
        <span className="mt-1 block text-xs text-text-muted">
          {Math.round(percentage)}%
        </span>
      )}
    </div>
  );
}

export { ProgressBar, progressTrackVariants, progressBarVariants };
