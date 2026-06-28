import { createContext, useContext, type HTMLAttributes, type ReactNode } from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "../../lib/utils";

const cardVariants = cva("rounded-lg border", {
  variants: {
    variant: {
      default: "border-border bg-surface-1",
      glass: "glass",
      interactive:
        "border-border bg-surface-1 cursor-pointer transition-all hover:shadow-md hover:-translate-y-0.5 active:translate-y-0",
    },
    padding: {
      sm: "p-3",
      md: "p-4",
      lg: "p-6",
    },
  },
  defaultVariants: {
    variant: "default",
    padding: "md",
  },
});

type Padding = "sm" | "md" | "lg";

const CardPaddingContext = createContext<Padding>("md");

const useCardPadding = () => useContext(CardPaddingContext);

export interface CardProps
  extends HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof cardVariants> {
  children?: ReactNode;
}

function Card({ className, variant, padding, children, ...props }: CardProps) {
  return (
    <CardPaddingContext.Provider value={padding || "md"}>
      <div
        className={cn(cardVariants({ variant, padding, className }))}
        {...props}
      >
        {children}
      </div>
    </CardPaddingContext.Provider>
  );
}

export interface CardHeaderProps extends HTMLAttributes<HTMLDivElement> {
  children?: ReactNode;
}

function CardHeader({ className, children, ...props }: CardHeaderProps) {
  const padding = useCardPadding();
  const paddingClass =
    padding === "sm" ? "p-3" : padding === "lg" ? "p-6" : "p-4";

  return (
    <div
      className={cn(
        "flex items-center justify-between border-b border-border",
        paddingClass,
        "pb-3",
        className,
      )}
      {...props}
    >
      {children}
    </div>
  );
}

export interface CardBodyProps extends HTMLAttributes<HTMLDivElement> {
  children?: ReactNode;
}

function CardBody({ className, children, ...props }: CardBodyProps) {
  const padding = useCardPadding();
  const paddingClass =
    padding === "sm" ? "p-3" : padding === "lg" ? "p-6" : "p-4";

  return (
    <div className={cn(paddingClass, className)} {...props}>
      {children}
    </div>
  );
}

export interface CardFooterProps extends HTMLAttributes<HTMLDivElement> {
  children?: ReactNode;
}

function CardFooter({ className, children, ...props }: CardFooterProps) {
  const padding = useCardPadding();
  const paddingClass =
    padding === "sm" ? "p-3" : padding === "lg" ? "p-6" : "p-4";

  return (
    <div
      className={cn(
        "flex items-center justify-between border-t border-border",
        paddingClass,
        "pt-3",
        className,
      )}
      {...props}
    >
      {children}
    </div>
  );
}

Card.Header = CardHeader;
Card.Body = CardBody;
Card.Footer = CardFooter;

export { Card, cardVariants };
export type { Padding };
