import * as React from "react";
import { clsx } from "clsx";

export interface InputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {}

export const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, type = "text", ...props }, ref) => {
    return (
      <input
        ref={ref}
        type={type}
        className={clsx(
          "flex h-10 w-full rounded-lg border border-border bg-surface/80 px-4 py-2 text-sm text-zinc-100 placeholder:text-muted transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent/50 focus-visible:border-accent/50",
          className
        )}
        {...props}
      />
    );
  }
);

Input.displayName = "Input";
