"use client";

type Status = "idle" | "signing" | "submitting" | "success" | "error";

interface TxStatusProps {
  status: Status;
  message?: string;
}

export function TxStatus({ status, message }: TxStatusProps) {
  if (status === "idle") return null;

  const styles: Record<Status, string> = {
    idle: "",
    signing: "bg-levee-mid/30 text-levee-light",
    submitting: "bg-levee-mid/30 text-levee-light",
    success: "bg-green-900/30 text-green-300",
    error: "bg-red-900/30 text-red-300",
  };

  const labels: Record<Status, string> = {
    idle: "",
    signing: "Waiting for signature...",
    submitting: "Submitting transaction...",
    success: message || "Transaction confirmed",
    error: message || "Transaction failed",
  };

  return (
    <div className={`rounded-lg px-4 py-3 text-sm ${styles[status]}`}>
      {labels[status]}
    </div>
  );
}
