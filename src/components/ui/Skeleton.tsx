import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "../../lib/utils";

const skeletonVariants = cva("skeleton", {
  variants: {
    variant: {
      text: "h-4 w-full",
      circle: "rounded-full",
      rect: "h-20 w-full rounded-md",
      card: "h-40 w-full rounded-lg",
    },
  },
  defaultVariants: {
    variant: "text",
  },
});

export interface SkeletonProps
  extends VariantProps<typeof skeletonVariants> {
  className?: string;
}

function Skeleton({ variant, className }: SkeletonProps) {
  return (
    <div
      className={cn(skeletonVariants({ variant, className }))}
      aria-hidden="true"
    />
  );
}

function CardSkeleton({ className }: { className?: string }) {
  return (
    <div className={cn("rounded-lg border border-border bg-surface-1 p-4", className)}>
      <Skeleton variant="text" className="w-2/3 mb-2" />
      <Skeleton variant="text" className="w-1/2 mb-1" />
      <Skeleton variant="text" className="w-3/4" />
    </div>
  );
}

export { Skeleton, CardSkeleton, skeletonVariants };
