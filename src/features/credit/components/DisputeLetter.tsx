import type { DisputeLetter as DisputeLetterType } from "@/types";
import { formatDate } from "@/lib/utils";

interface DisputeLetterProps {
  letter: DisputeLetterType;
  consumerName?: string;
  consumerAddress?: string;
}

export function DisputeLetter({ letter, consumerName = "Consumer Name", consumerAddress = "123 Main Street, City, State ZIP" }: DisputeLetterProps) {
  return (
    <div className="print-container">
      <div className="print-actions no-print flex gap-2 mb-4">
        <button
          onClick={() => window.print()}
          className="inline-flex items-center gap-2 rounded bg-primary text-primary-foreground px-4 py-2 text-sm font-medium hover:bg-primary-hover transition-colors"
        >
          Print Letter
        </button>
      </div>

      <div className="letter-page">
        <div className="letter-header">
          <p className="consumer-name">{consumerName}</p>
          <p className="consumer-address">{consumerAddress}</p>
          <p className="letter-date">{formatDate(letter.created_at)}</p>
        </div>

        <div className="recipient-block">
          <p>{letter.recipient_name}</p>
          <p>{letter.recipient_address}</p>
        </div>

        <div className="letter-subject">
          <p className="font-semibold">RE: {letter.template_code} - Dispute Letter</p>
        </div>

        <div className="letter-body">
          <p>{letter.content}</p>
        </div>

        <div className="letter-footer">
          <p>Sincerely,</p>
          <p className="consumer-name">{consumerName}</p>
        </div>
      </div>

      <style>{`
        .print-container {
          font-family: 'Inter', system-ui, sans-serif;
        }
        .letter-page {
          background: #fff;
          color: #111;
          padding: 2.5rem 2rem;
          max-width: 800px;
          margin: 0 auto;
          border-radius: 8px;
          box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        }
        .letter-header {
          margin-bottom: 2rem;
        }
        .consumer-name {
          font-weight: 600;
          font-size: 1rem;
          margin: 0;
        }
        .consumer-address {
          font-size: 0.875rem;
          color: #555;
          margin: 0.25rem 0;
        }
        .letter-date {
          font-size: 0.875rem;
          color: #555;
          margin-top: 0.5rem;
        }
        .recipient-block {
          margin-bottom: 1.5rem;
          font-size: 0.875rem;
        }
        .recipient-block p {
          margin: 0;
        }
        .letter-subject {
          margin-bottom: 1.5rem;
          font-size: 0.875rem;
          border-bottom: 1px solid #ddd;
          padding-bottom: 0.75rem;
        }
        .letter-body {
          font-size: 0.9375rem;
          line-height: 1.7;
          margin-bottom: 2rem;
          white-space: pre-wrap;
        }
        .letter-footer {
          font-size: 0.875rem;
        }
        .letter-footer p {
          margin: 0.25rem 0;
        }
        @media print {
          .no-print {
            display: none !important;
          }
          body {
            background: white;
            margin: 0;
            padding: 0;
          }
          .print-container {
            padding: 0;
          }
          .letter-page {
            box-shadow: none;
            border-radius: 0;
            padding: 0.75in;
            max-width: none;
          }
          .letter-body {
            font-size: 12pt;
          }
        }
      `}</style>
    </div>
  );
}

export function DisputeLetterPreview({ letter }: { letter: DisputeLetterType }) {
  return (
    <div className="rounded-lg border border-border bg-[hsl(220_15%_14%/0.7)] p-6 font-mono text-sm leading-relaxed text-[hsl(0_0%_80%)] whitespace-pre-wrap max-h-96 overflow-y-auto">
      {letter.content}
    </div>
  );
}
