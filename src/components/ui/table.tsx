import { type ReactNode } from "react";
import { ArrowUpDown, ArrowUp, ArrowDown } from "lucide-react";
import { cn } from "../../lib/utils";
import { Skeleton } from "./Skeleton";
import { EmptyState } from "./empty-state";

export interface TableColumn<T> {
  key: string;
  header: string;
  sortable?: boolean;
  render?: (item: T) => ReactNode;
  className?: string;
  headerClassName?: string;
}

export interface SortConfig {
  key: string;
  direction: "asc" | "desc";
}

export interface TableProps<T> {
  columns: TableColumn<T>[];
  data: T[];
  keyExtractor: (item: T) => string | number;
  isLoading?: boolean;
  emptyMessage?: string;
  emptyIcon?: React.ComponentType<{ className?: string }>;
  stickyHeader?: boolean;
  onSort?: (key: string, direction: "asc" | "desc") => void;
  sortConfig?: SortConfig | null;
  className?: string;
  rowClassName?: string;
}

function Table<T extends Record<string, unknown>>({
  columns,
  data,
  keyExtractor,
  isLoading,
  emptyMessage = "No data",
  emptyIcon: EmptyIcon,
  onSort,
  sortConfig,
  className,
  rowClassName,
}: TableProps<T>) {
  const renderSortIcon = (column: TableColumn<T>) => {
    if (!column.sortable) return null;
    const isActive = sortConfig?.key === column.key;
    if (isActive) {
      return sortConfig?.direction === "asc" ? (
        <ArrowUp className="ml-1 size-3 text-primary" aria-hidden="true" />
      ) : (
        <ArrowDown className="ml-1 size-3 text-primary" aria-hidden="true" />
      );
    }
    return (
      <ArrowUpDown className="ml-1 size-3 text-text-muted" aria-hidden="true" />
    );
  };

  if (isLoading) {
    return (
      <div className={cn("overflow-x-auto", className)}>
        <table className="w-full">
          <thead>
            <tr className="border-b border-border">
              {columns.map((col) => (
                <th
                  key={col.key}
                  className="px-4 py-3 text-left text-xs font-medium text-text-muted"
                >
                  {col.header}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {Array.from({ length: 5 }).map((_, rowIdx) => (
              <tr key={rowIdx} className="border-b border-border last:border-0">
                {columns.map((col) => (
                  <td key={col.key} className="px-4 py-3">
                    <Skeleton variant="text" className="h-4 w-3/4" />
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    );
  }

  if (data.length === 0) {
    return (
      <EmptyState
        icon={EmptyIcon}
        title={emptyMessage}
        className={className}
      />
    );
  }

  return (
    <div className={cn("overflow-x-auto", className)}>
      <table className="w-full">
        <thead>
          <tr className="border-b border-border">
            {columns.map((col) => (
              <th
                key={col.key}
                className={cn(
                  "px-4 py-3 text-left text-xs font-medium text-text-muted transition-colors",
                  col.sortable && "cursor-pointer select-none hover:text-text-primary",
                  col.headerClassName,
                )}
                onClick={() => {
                  if (col.sortable && onSort) {
                    const direction =
                      sortConfig?.key === col.key && sortConfig?.direction === "asc"
                        ? "desc"
                        : "asc";
                    onSort(col.key, direction);
                  }
                }}
                scope="col"
              >
                <div className="inline-flex items-center">
                  {col.header}
                  {renderSortIcon(col)}
                </div>
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {data.map((item) => (
            <tr
              key={keyExtractor(item)}
              className={cn(
                "border-b border-border last:border-0 transition-colors hover:bg-surface-2/50",
                rowClassName,
              )}
            >
              {columns.map((col) => (
                <td
                  key={col.key}
                  className={cn("px-4 py-3 text-sm text-text-primary", col.className)}
                >
                  {col.render
                    ? col.render(item)
                    : String(item[col.key as keyof T] ?? "")}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export { Table };
