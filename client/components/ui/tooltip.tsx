import * as React from "react";

interface TooltipContextValue {
  open: boolean;
  setOpen(open: boolean): void;
}

const TooltipContext = React.createContext<TooltipContextValue | null>(null);

export function TooltipProvider({ children }: { children: React.ReactNode }) {
  return <>{children}</>;
}

export function Tooltip({
  children
}: {
  children: React.ReactNode;
}): JSX.Element {
  const [open, setOpen] = React.useState(false);

  return (
    <TooltipContext.Provider value={{ open, setOpen }}>
      {children}
    </TooltipContext.Provider>
  );
}

export function TooltipTrigger({
  children
}: {
  children: React.ReactElement;
}) {
  const ctx = React.useContext(TooltipContext);
  if (!ctx) return children;

  return React.cloneElement(children, {
    onMouseEnter: () => ctx.setOpen(true),
    onMouseLeave: () => ctx.setOpen(false)
  });
}

export function TooltipContent({
  children
}: {
  children: React.ReactNode;
}) {
  const ctx = React.useContext(TooltipContext);
  if (!ctx || !ctx.open) return null;

  return (
    <div className="pointer-events-none fixed z-50 rounded-md border border-slate-700 bg-slate-900/90 px-3 py-1.5 text-xs text-slate-50 shadow-lg">
      {children}
    </div>
  );
}

