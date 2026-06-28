import toast from "react-hot-toast";
import { CheckCircle, XCircle, Info, AlertTriangle } from "lucide-react";

const defaultStyle = {
  background: "hsl(var(--surface-1))",
  color: "hsl(var(--text-primary))",
  border: "1px solid hsl(var(--border))",
  borderRadius: "8px",
  fontSize: "14px",
  padding: "12px 16px",
};

export function showSuccess(message: string) {
  toast.success(message, {
    style: defaultStyle,
    icon: <CheckCircle className="size-5 text-success" aria-hidden="true" />,
    duration: 4000,
  });
}

export function showError(message: string) {
  toast.error(message, {
    style: defaultStyle,
    icon: <XCircle className="size-5 text-danger" aria-hidden="true" />,
    duration: 5000,
  });
}

export function showInfo(message: string) {
  toast(message, {
    style: defaultStyle,
    icon: <Info className="size-5 text-info" aria-hidden="true" />,
    duration: 4000,
  });
}

export function showWarning(message: string) {
  toast(message, {
    style: defaultStyle,
    icon: <AlertTriangle className="size-5 text-warning" aria-hidden="true" />,
    duration: 4000,
  });
}

export { toast };
