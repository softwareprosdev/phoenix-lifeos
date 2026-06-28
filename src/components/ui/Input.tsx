import { forwardRef, useState, type InputHTMLAttributes, type ReactNode } from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { Search, X } from "lucide-react";
import { cn } from "../../lib/utils";

const inputVariants = cva(
  "flex w-full rounded border bg-surface-0 px-3 py-2 text-sm text-text-primary placeholder:text-text-muted transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary disabled:cursor-not-allowed disabled:opacity-50 file:border-0 file:bg-transparent file:text-sm file:font-medium",
  {
    variants: {
      variant: {
        default: "border-border hover:border-primary/50",
        search: "border-border hover:border-primary/50 pl-9",
        error: "border-danger hover:border-danger focus-visible:ring-danger",
      },
      size: {
        sm: "h-8 text-xs px-2",
        md: "h-9 text-sm px-3",
        lg: "h-10 text-sm px-3",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "md",
    },
  },
);

export interface InputProps
  extends Omit<InputHTMLAttributes<HTMLInputElement>, 'size'>,
    VariantProps<typeof inputVariants> {
  label?: string;
  leftIcon?: ReactNode;
  rightIcon?: ReactNode;
  clearable?: boolean;
  error?: string;
}

const Input = forwardRef<HTMLInputElement, InputProps>(
  (
    {
      className,
      variant,
      size,
      label,
      leftIcon,
      rightIcon,
      clearable,
      error,
      value,
      onChange,
      ...props
    },
    ref,
  ) => {
    const [localValue, setLocalValue] = useState("");
    const isControlled = value !== undefined;
    const currentValue = isControlled ? value : localValue;

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      if (!isControlled) setLocalValue(e.target.value);
      onChange?.(e);
    };

    const handleClear = () => {
      if (!isControlled) setLocalValue("");
      if (onChange) {
        const nativeEvent = new Event("input", { bubbles: true });
        const fakeEvent = {
          ...nativeEvent,
          target: { value: "" } as EventTarget & HTMLInputElement,
          currentTarget: { value: "" } as EventTarget & HTMLInputElement,
        } as unknown as React.ChangeEvent<HTMLInputElement>;
        onChange(fakeEvent);
      }
    };

    const effectiveVariant = error ? "error" : variant;

    return (
      <div className="w-full">
        {label && (
          <label htmlFor={props.id} className="mb-1.5 block text-xs font-medium text-text-muted">
            {label}
          </label>
        )}
        <div className="relative">
          {(leftIcon || effectiveVariant === "search") && (
            <div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3 text-text-muted">
              {effectiveVariant === "search" && !leftIcon ? (
                <Search className="size-4" aria-hidden="true" />
              ) : (
                leftIcon
              )}
            </div>
          )}
          <input
            className={cn(
              inputVariants({ variant: effectiveVariant, size, className }),
              leftIcon || effectiveVariant === "search" ? "pl-9" : "",
              rightIcon || clearable ? "pr-9" : "",
            )}
            ref={ref}
            value={currentValue}
            onChange={handleChange}
            aria-invalid={!!error}
            aria-describedby={error ? `${props.id}-error` : undefined}
            {...props}
          />
          {(rightIcon || clearable) && (
            <div className="absolute inset-y-0 right-0 flex items-center pr-3">
              {clearable && currentValue ? (
                <button
                  type="button"
                  onClick={handleClear}
                  className="text-text-muted hover:text-text-primary transition-colors"
                  aria-label="Clear input"
                >
                  <X className="size-4" />
                </button>
              ) : (
                rightIcon
              )}
            </div>
          )}
        </div>
        {error && (
          <p
            id={`${props.id}-error`}
            className="mt-1 text-xs text-danger"
            role="alert"
          >
            {error}
          </p>
        )}
      </div>
    );
  },
);
Input.displayName = "Input";

export { Input, inputVariants };
