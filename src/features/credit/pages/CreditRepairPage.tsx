import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Shield, FileText, Send, Plus, Eye, Printer,
  ChevronDown, ChevronRight, Clock, CheckCircle2, XCircle,
  Briefcase, CreditCard as CreditCardIcon,
  Target, FileSignature, Loader2,
} from "lucide-react";
import { formatCurrency, formatDate, cn } from "@/lib/utils";
import {
  useDisputeCases, useDisputeCase,
  useDisputeLetters, useGenerateLetter, useLetterTemplates,
  useMarkLetterSent, useAddTimelineEntry,
  useCreditAccounts, useCreditInquiries, useDerogatoryItems,
  useBusinessCreditProfiles, useCreateDisputeCase,
  useCreateCreditAccount, useCreateCreditInquiry,
  useCreateDerogatoryItem, useCreateBusinessCreditProfile,
} from "../hooks/useCreditRepairData";
import { DisputeLetterPreview } from "../components/DisputeLetter";

const STATUS_CONFIG: Record<string, { label: string; color: string; icon: React.ElementType }> = {
  won: { label: "Won", color: "text-success bg-success/10 border-success/20", icon: CheckCircle2 },
  in_progress: { label: "In Progress", color: "text-info bg-info/10 border-info/20", icon: Clock },
  draft: { label: "Draft", color: "text-[hsl(0_0%_55%)] bg-[hsl(220_15%_22%/0.5)] border-[hsl(220_15%_28%)]", icon: FileText },
  lost: { label: "Lost", color: "text-danger bg-danger/10 border-danger/20", icon: XCircle },
};

function StatusBadge({ status }: { status: string }) {
  const cfg = STATUS_CONFIG[status] ?? STATUS_CONFIG.draft;
  const Icon = cfg.icon;
  return (
    <span className={cn("inline-flex items-center gap-1 rounded px-2 py-0.5 text-xs font-medium border", cfg.color)}>
      <Icon size={12} />
      {cfg.label}
    </span>
  );
}

function CollapsibleSection({ title, count, defaultOpen, children }: { title: string; count?: number; defaultOpen?: boolean; children: React.ReactNode }) {
  const [open, setOpen] = useState(defaultOpen ?? true);
  return (
    <div className="rounded-lg border border-[hsl(220_15%_22%/0.6)] bg-[hsl(220_15%_14%/0.7)] overflow-hidden">
      <button
        onClick={() => setOpen(!open)}
        className="w-full flex items-center justify-between px-4 py-3 text-sm font-semibold text-white hover:bg-[hsl(220_15%_18%/0.5)] transition-colors"
      >
        <div className="flex items-center gap-2">
          {open ? <ChevronDown size={14} /> : <ChevronRight size={14} />}
          <span>{title}</span>
          {count !== undefined && (
            <span className="text-xs text-[hsl(0_0%_55%)] font-normal">({count})</span>
          )}
        </div>
      </button>
      <AnimatePresence>
        {open && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: "auto", opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            transition={{ duration: 0.15 }}
            className="border-t border-[hsl(220_15%_22%/0.4)]"
          >
            {children}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

function LetterGenerationModal({
  isOpen, onClose, caseId, onGenerated,
}: {
  isOpen: boolean; onClose: () => void; caseId: string; onGenerated: () => void;
}) {
  const { data: templates } = useLetterTemplates();
  const generateLetter = useGenerateLetter();
  const [selectedTemplate, setSelectedTemplate] = useState("");
  const [variables, setVariables] = useState<Record<string, string>>({});
  const [preview, setPreview] = useState(false);

  const template = templates?.find((t) => t.template_code === selectedTemplate);

  const handleGenerate = async () => {
    if (!template) return;
    await generateLetter.mutateAsync({ caseId, templateId: template.id, variables });
    onGenerated();
    onClose();
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4" onClick={onClose}>
      <motion.div
        initial={{ opacity: 0, scale: 0.95, y: 20 }}
        animate={{ opacity: 1, scale: 1, y: 0 }}
        exit={{ opacity: 0, scale: 0.95, y: 20 }}
        className="relative w-full max-w-xl rounded-lg border border-[hsl(220_15%_22%)] bg-[hsl(220_15%_14%)] shadow-lg"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between border-b border-[hsl(220_15%_22%)] px-6 py-4">
          <h3 className="text-base font-semibold text-white">Generate Dispute Letter</h3>
          <button onClick={onClose} className="rounded p-1 text-[hsl(0_0%_55%)] hover:text-white hover:bg-[hsl(220_15%_18%)] transition-colors">
            <XCircle size={16} />
          </button>
        </div>
        <div className="px-6 py-4 space-y-4">
          <div>
            <label className="mb-1.5 block text-xs font-medium text-[hsl(0_0%_55%)]">Template</label>
            <select
              value={selectedTemplate}
              onChange={(e) => { setSelectedTemplate(e.target.value); setVariables({}); setPreview(false); }}
              className="flex w-full appearance-none rounded border border-[hsl(220_15%_22%)] bg-[hsl(220_15%_10%)] px-3 py-2 pr-8 text-sm text-white transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[hsl(220_90%_56%)]"
            >
              <option value="">Select a template...</option>
              {templates?.map((t) => (
                <option key={t.id} value={t.template_code}>{t.template_code} - {t.name}</option>
              ))}
            </select>
          </div>

          {template && (
            <div className="rounded bg-[hsl(220_15%_10%/0.5)] p-3 text-xs text-[hsl(0_0%_60%)] space-y-1">
              <p><span className="text-[hsl(0_0%_50%)]">Legal Basis:</span> {template.legal_basis}</p>
              <p><span className="text-[hsl(0_0%_50%)]">Recipient:</span> {template.recipient_type}</p>
              <p className="text-[hsl(0_0%_50%)] italic">{template.description}</p>
            </div>
          )}

          {template && Object.keys(template.variable_schema).length > 0 && (
            <div className="space-y-3">
              <p className="text-xs font-medium text-[hsl(0_0%_55%)] uppercase tracking-wider">Variables</p>
              {Object.entries(template.variable_schema).map(([key, label]) => (
                <div key={key}>
                  <label className="mb-1 block text-xs text-[hsl(0_0%_55%)]">{label}</label>
                  <input
                    value={variables[key] ?? ""}
                    onChange={(e) => setVariables((v) => ({ ...v, [key]: e.target.value }))}
                    placeholder={`Enter ${label.toLowerCase()}...`}
                    className="flex w-full rounded border border-[hsl(220_15%_22%)] bg-[hsl(220_15%_10%)] px-3 py-2 text-sm text-white placeholder:text-[hsl(0_0%_45%)] transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[hsl(220_90%_56%)]"
                  />
                </div>
              ))}
            </div>
          )}

          {preview && template && (
            <div className="rounded border border-[hsl(220_15%_22%)] bg-[hsl(220_15%_10%/0.8)] p-4">
              <p className="text-xs font-medium text-[hsl(0_0%_55%)] mb-2">Preview</p>
              <div className="text-sm text-[hsl(0_0%_80%)] whitespace-pre-wrap leading-relaxed font-mono">
                {template.default_content}
                {Object.keys(variables).length > 0 && (
                  <div className="mt-2 text-xs text-[hsl(0_0%_50%)]">
                    <p className="font-semibold text-[hsl(0_0%_60%)] mb-1">Variables:</p>
                    {Object.entries(variables).map(([k, v]) => (
                      <p key={k}>{k.replace(/_/g, " ")}: {v || "(not set)"}</p>
                    ))}
                  </div>
                )}
              </div>
            </div>
          )}
        </div>

        <div className="flex items-center justify-end gap-2 border-t border-[hsl(220_15%_22%)] px-6 py-4">
          <button
            onClick={() => setPreview(!preview)}
            className="inline-flex items-center gap-2 rounded border border-[hsl(220_15%_22%)] px-4 py-2 text-sm text-white hover:bg-[hsl(220_15%_18%)] transition-colors"
          >
            <Eye size={14} />
            {preview ? "Hide Preview" : "Preview Letter"}
          </button>
          <button
            onClick={handleGenerate}
            disabled={!template || generateLetter.isPending}
            className="inline-flex items-center gap-2 rounded bg-[hsl(220_90%_56%)] px-4 py-2 text-sm font-medium text-white hover:bg-[hsl(220_90%_48%)] transition-colors disabled:opacity-50"
          >
            {generateLetter.isPending ? <Loader2 size={14} className="animate-spin" /> : <FileSignature size={14} />}
            Generate & Save
          </button>
        </div>
      </motion.div>
    </div>
  );
}

export default function CreditRepairPage() {
  const { data: cases, isLoading: casesLoading } = useDisputeCases();
  const { data: templates } = useLetterTemplates();
  const { data: creditAccounts } = useCreditAccounts();
  const { data: inquiries } = useCreditInquiries();
  const { data: derogatory } = useDerogatoryItems();
  const { data: businessProfiles } = useBusinessCreditProfiles();

  const [selectedCaseId, setSelectedCaseId] = useState<string | null>(null);
  const [showLetterModal, setShowLetterModal] = useState(false);
  const [letterModalCaseId, setLetterModalCaseId] = useState("");
  const [selectedTab, setSelectedTab] = useState<string>("overview");

  const { data: caseDetail } = useDisputeCase(selectedCaseId);
  const { data: caseLetters } = useDisputeLetters(selectedCaseId);
  const markSent = useMarkLetterSent();
  const addTimeline = useAddTimelineEntry();
  const createDisputeCase = useCreateDisputeCase();
  const createCreditAccount = useCreateCreditAccount();
  const createCreditInquiry = useCreateCreditInquiry();
  const createDerogatoryItem = useCreateDerogatoryItem();
  const createBusinessProfile = useCreateBusinessCreditProfile();

  const wonCount = cases?.filter((c) => c.status === "won").length ?? 0;
  const inProgressCount = cases?.filter((c) => c.status === "in_progress").length ?? 0;
  const draftCount = cases?.filter((c) => c.status === "draft").length ?? 0;

  const handleOpenLetterModal = (caseId: string) => {
    setLetterModalCaseId(caseId);
    setShowLetterModal(true);
  };

  const handleMarkSent = async (letterId: string) => {
    const tn = prompt("Enter tracking number:");
    if (tn) {
      await markSent.mutateAsync({ letterId, trackingNumber: tn });
      if (selectedCaseId && caseDetail) {
        await addTimeline.mutateAsync({
          caseId: selectedCaseId,
          eventType: "letter_sent",
          description: `Letter sent via certified mail - tracking: ${tn}`,
        });
      }
    }
  };

  const handleCreateDisputeCase = async () => {
    const creditor = prompt("Creditor or furnisher name:");
    if (!creditor) return;
    const bureau = prompt("Bureau:", "Experian") || "Experian";
    const disputeType = prompt("Dispute type:", "inaccurate_information") || "inaccurate_information";
    const notes = prompt("Dispute notes/reason:", `Dispute ${creditor} as inaccurate or unverifiable.`) || "";
    const created = await createDisputeCase.mutateAsync({
      creditor_name: creditor,
      bureau,
      dispute_type: disputeType,
      reason: notes,
      notes,
      status: "draft",
    });
    setSelectedCaseId(created.id);
  };

  const handleCreateCreditAccount = async () => {
    const creditor = prompt("Creditor name:");
    if (!creditor) return;
    const limit = Number(prompt("Credit limit in dollars:", "1000") || "0");
    const balance = Number(prompt("Current balance in dollars:", "0") || "0");
    await createCreditAccount.mutateAsync({
      creditor_name: creditor,
      account_type: prompt("Account type:", "revolving") || "revolving",
      bureau: prompt("Bureau:", "Experian") || "Experian",
      credit_limit_cents: Math.round(limit * 100),
      balance_cents: Math.round(balance * 100),
      payment_status: prompt("Payment status:", "current") || "current",
      opened_date: new Date().toISOString(),
    });
  };

  const handleCreateInquiry = async () => {
    const creditor = prompt("Inquiry creditor:");
    if (!creditor) return;
    await createCreditInquiry.mutateAsync({
      creditor_name: creditor,
      bureau: prompt("Bureau:", "Experian") || "Experian",
      inquiry_type: prompt("Inquiry type:", "hard") || "hard",
      inquiry_date: new Date().toISOString(),
    });
  };

  const handleCreateDerogatory = async () => {
    const creditor = prompt("Derogatory creditor/collector:");
    if (!creditor) return;
    await createDerogatoryItem.mutateAsync({
      creditor_name: creditor,
      bureau: prompt("Bureau:", "Experian") || "Experian",
      derogatory_type: prompt("Item type:", "collection") || "collection",
      account_number_masked: prompt("Masked account number:", "****") || "****",
      date_reported: new Date().toISOString(),
      date_expected_removal: new Date(new Date().setFullYear(new Date().getFullYear() + 7)).toISOString(),
      description: prompt("Description:", "Disputed derogatory item") || "Disputed derogatory item",
    });
  };

  const handleCreateBusinessProfile = async () => {
    const businessName = prompt("Business name:");
    if (!businessName) return;
    const score = Number(prompt("Business credit score:", "75") || "0");
    const paydex = Number(prompt("Paydex score:", "80") || "0");
    await createBusinessProfile.mutateAsync({
      business_name: businessName,
      ein: prompt("Masked EIN:", "**-*******") || "",
      industry: prompt("Industry:", "General") || "General",
      business_credit_score: score,
      paydex_score: paydex,
    });
  };

  if (casesLoading) {
    return (
      <div className="p-6 space-y-4">
        {Array.from({ length: 3 }).map((_, i) => (
          <div key={i} className="rounded-lg h-24 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)] skeleton" />
        ))}
      </div>
    );
  }

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.2 }}
      className="p-6 h-full overflow-auto space-y-4"
    >
      {/* Subnavigation */}
      <div className="flex items-center gap-1 rounded-lg bg-[hsl(220_15%_14%/0.5)] p-1 border border-[hsl(220_15%_22%/0.4)] w-fit">
        {[
          { id: "overview", label: "Overview", icon: Shield },
          { id: "accounts", label: "Accounts", icon: CreditCardIcon },
          { id: "business", label: "Business", icon: Briefcase },
        ].map((tab) => {
          const Icon = tab.icon;
          return (
            <button
              key={tab.id}
              onClick={() => setSelectedTab(tab.id)}
              className={cn(
                "inline-flex items-center gap-2 rounded px-3 py-1.5 text-xs font-medium transition-colors",
                selectedTab === tab.id
                  ? "bg-[hsl(220_90%_56%/0.15)] text-[hsl(220_90%_70%)]"
                  : "text-[hsl(0_0%_55%)] hover:text-white hover:bg-[hsl(220_15%_18%/0.5)]",
              )}
            >
              <Icon size={14} />
              {tab.label}
            </button>
          );
        })}
      </div>

      {/* Overview Tab */}
      {selectedTab === "overview" && (
        <>
          <div className="flex flex-wrap items-center gap-2">
            <button
              onClick={handleCreateDisputeCase}
              disabled={createDisputeCase.isPending}
              className="inline-flex items-center gap-2 rounded bg-[hsl(220_90%_56%)] px-3 py-2 text-xs font-medium text-white hover:bg-[hsl(220_90%_48%)] transition-colors disabled:opacity-50"
            >
              {createDisputeCase.isPending ? <Loader2 size={14} className="animate-spin" /> : <Plus size={14} />}
              New Dispute Case
            </button>
            <button
              onClick={handleCreateDerogatory}
              disabled={createDerogatoryItem.isPending}
              className="inline-flex items-center gap-2 rounded border border-[hsl(220_15%_22%)] px-3 py-2 text-xs font-medium text-white hover:bg-[hsl(220_15%_18%)] transition-colors disabled:opacity-50"
            >
              {createDerogatoryItem.isPending ? <Loader2 size={14} className="animate-spin" /> : <Plus size={14} />}
              Add Derogatory Item
            </button>
          </div>

          {/* Stats Dashboard */}
          <div className="grid grid-cols-1 sm:grid-cols-4 gap-3">
            <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
              <div className="flex items-center gap-3">
                <div className="w-9 h-9 rounded-lg flex items-center justify-center bg-success/10">
                  <CheckCircle2 size={16} className="text-success" />
                </div>
                <div>
                  <p className="text-lg font-bold text-white">{wonCount}</p>
                  <p className="text-[10px] text-[hsl(0_0%_45%)] uppercase tracking-wider">Cases Won</p>
                </div>
              </div>
            </div>
            <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
              <div className="flex items-center gap-3">
                <div className="w-9 h-9 rounded-lg flex items-center justify-center bg-info/10">
                  <Clock size={16} className="text-info" />
                </div>
                <div>
                  <p className="text-lg font-bold text-white">{inProgressCount}</p>
                  <p className="text-[10px] text-[hsl(0_0%_45%)] uppercase tracking-wider">In Progress</p>
                </div>
              </div>
            </div>
            <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
              <div className="flex items-center gap-3">
                <div className="w-9 h-9 rounded-lg flex items-center justify-center bg-[hsl(220_15%_22%/0.5)]">
                  <FileText size={16} className="text-[hsl(0_0%_55%)]" />
                </div>
                <div>
                  <p className="text-lg font-bold text-white">{draftCount}</p>
                  <p className="text-[10px] text-[hsl(0_0%_45%)] uppercase tracking-wider">Drafts</p>
                </div>
              </div>
            </div>
            <div className="rounded-lg p-4 bg-[hsl(220_15%_14%/0.7)] border border-[hsl(220_15%_22%/0.6)]">
              <div className="flex items-center gap-3">
                <div className="w-9 h-9 rounded-lg flex items-center justify-center bg-[hsl(220_90%_56%/0.1)]">
                  <Target size={16} className="text-[hsl(220_90%_56%)]" />
                </div>
                <div>
                  <p className="text-lg font-bold text-white">{cases?.length ?? 0}</p>
                  <p className="text-[10px] text-[hsl(0_0%_45%)] uppercase tracking-wider">Total Cases</p>
                </div>
              </div>
            </div>
          </div>

          {/* Letter Templates Library */}
          <CollapsibleSection title="Letter Templates Library" count={templates?.length} defaultOpen={false}>
            <div className="p-4">
              <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3">
                {templates?.map((t) => (
                  <div key={t.id} className="rounded-lg p-3 border border-[hsl(220_15%_22%/0.5)] bg-[hsl(220_15%_10%/0.4)] hover:border-[hsl(220_90%_56%/0.3)] transition-colors">
                    <div className="flex items-center justify-between mb-2">
                      <span className="text-[10px] font-mono font-bold text-[hsl(220_90%_60%)]">{t.template_code}</span>
                      <span className="text-[9px] uppercase text-[hsl(0_0%_45%)] px-1.5 py-0.5 rounded bg-[hsl(220_15%_22%/0.5)]">{t.recipient_type}</span>
                    </div>
                    <p className="text-xs font-semibold text-white mb-1 line-clamp-2">{t.name}</p>
                    <p className="text-[10px] text-[hsl(0_0%_50%)] line-clamp-2 mb-2">{t.description}</p>
                    <p className="text-[9px] text-[hsl(0_0%_40%)] font-mono truncate">{t.legal_basis}</p>
                  </div>
                ))}
              </div>
            </div>
          </CollapsibleSection>

          {/* Dispute Cases & Detail */}
          <div className="grid grid-cols-1 xl:grid-cols-2 gap-4">
            {/* Dispute Cases List */}
            <CollapsibleSection title="Dispute Cases" count={cases?.length}>
              <div className="overflow-x-auto">
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b border-[hsl(220_15%_22%/0.4)] text-[10px] uppercase tracking-wider text-[hsl(0_0%_45%)]">
                      <th className="text-left px-4 py-2 font-medium">Case</th>
                      <th className="text-left px-4 py-2 font-medium">Creditor</th>
                      <th className="text-left px-4 py-2 font-medium">Bureau</th>
                      <th className="text-left px-4 py-2 font-medium">Type</th>
                      <th className="text-left px-4 py-2 font-medium">Status</th>
                      <th className="text-left px-4 py-2 font-medium">Date</th>
                      <th className="text-right px-4 py-2 font-medium">Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {cases?.map((c) => (
                      <tr
                        key={c.id}
                        className={cn(
                          "border-b border-[hsl(220_15%_22%/0.2)] transition-colors hover:bg-[hsl(220_15%_18%/0.3)]",
                          selectedCaseId === c.id && "bg-[hsl(220_90%_56%/0.05)]",
                        )}
                      >
                        <td className="px-4 py-2.5">
                          <button
                            onClick={() => setSelectedCaseId(selectedCaseId === c.id ? null : c.id)}
                            className="text-xs font-mono text-[hsl(220_90%_60%)] hover:underline"
                          >
                            {c.case_number}
                          </button>
                        </td>
                        <td className="px-4 py-2.5 text-xs text-white">{c.creditor_name}</td>
                        <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_60%)]">{c.bureau}</td>
                        <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_60%)]">{c.dispute_type}</td>
                        <td className="px-4 py-2.5"><StatusBadge status={c.status} /></td>
                        <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_50%)]">{formatDate(c.created_at)}</td>
                        <td className="px-4 py-2.5 text-right">
                          <div className="flex items-center justify-end gap-1">
                            <button
                              onClick={() => setSelectedCaseId(selectedCaseId === c.id ? null : c.id)}
                              className="rounded p-1.5 text-[hsl(0_0%_55%)] hover:text-white hover:bg-[hsl(220_15%_18%)] transition-colors"
                              title="View Details"
                            >
                              <Eye size={13} />
                            </button>
                            <button
                              onClick={() => handleOpenLetterModal(c.id)}
                              className="rounded p-1.5 text-[hsl(0_0%_55%)] hover:text-white hover:bg-[hsl(220_15%_18%)] transition-colors"
                              title="Generate Letter"
                            >
                              <FileSignature size={13} />
                            </button>
                          </div>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </CollapsibleSection>

            {/* Case Detail */}
            {selectedCaseId && caseDetail && (
              <motion.div
                initial={{ opacity: 0, x: 20 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ duration: 0.2 }}
                className="space-y-3"
              >
                {/* Case Info Card */}
                <div className="rounded-lg border border-[hsl(220_15%_22%/0.6)] bg-[hsl(220_15%_14%/0.7)] p-4">
                  <div className="flex items-center justify-between mb-3">
                    <h3 className="text-sm font-semibold text-white">{caseDetail.case.case_number}</h3>
                    <StatusBadge status={caseDetail.case.status} />
                  </div>
                  <div className="grid grid-cols-2 gap-3 text-xs">
                    <div>
                      <span className="text-[hsl(0_0%_45%)]">Creditor:</span>
                      <p className="text-white">{caseDetail.case.creditor_name}</p>
                    </div>
                    <div>
                      <span className="text-[hsl(0_0%_45%)]">Bureau:</span>
                      <p className="text-white">{caseDetail.case.bureau}</p>
                    </div>
                    <div>
                      <span className="text-[hsl(0_0%_45%)]">Type:</span>
                      <p className="text-white">{caseDetail.case.dispute_type}</p>
                    </div>
                    <div>
                      <span className="text-[hsl(0_0%_45%)]">Reason:</span>
                      <p className="text-white">{caseDetail.case.reason}</p>
                    </div>
                  </div>
                  {caseDetail.case.notes && (
                    <div className="mt-3 pt-3 border-t border-[hsl(220_15%_22%/0.4)]">
                      <span className="text-[10px] text-[hsl(0_0%_45%)]">Notes:</span>
                      <p className="text-xs text-[hsl(0_0%_70%)] mt-0.5">{caseDetail.case.notes}</p>
                    </div>
                  )}
                </div>

                {/* Timeline */}
                <div className="rounded-lg border border-[hsl(220_15%_22%/0.6)] bg-[hsl(220_15%_14%/0.7)] p-4">
                  <div className="flex items-center justify-between mb-3">
                    <h4 className="text-xs font-semibold text-white uppercase tracking-wider">Timeline</h4>
                    <button
                      onClick={async () => {
                        const desc = prompt("Enter timeline event description:");
                        if (desc) {
                          await addTimeline.mutateAsync({ caseId: selectedCaseId, eventType: "manual", description: desc });
                        }
                      }}
                      className="rounded p-1 text-[hsl(0_0%_55%)] hover:text-white hover:bg-[hsl(220_15%_18%)] transition-colors"
                    >
                      <Plus size={13} />
                    </button>
                  </div>
                  <div className="space-y-0">
                    {caseDetail.timeline.map((entry, i) => (
                      <div key={entry.id} className="flex gap-3">
                        <div className="flex flex-col items-center">
                          <div className={cn(
                            "w-2 h-2 rounded-full mt-1.5",
                            entry.event_type === "case_created" ? "bg-[hsl(220_90%_56%)]" :
                            entry.event_type === "letter_sent" ? "bg-info" :
                            entry.event_type === "case_won" ? "bg-success" :
                            entry.event_type === "letter_generated" ? "bg-warning" :
                            "bg-[hsl(0_0%_40%)]",
                          )} />
                          {i < caseDetail.timeline.length - 1 && (
                            <div className="w-px flex-1 bg-[hsl(220_15%_22%/0.5)]" />
                          )}
                        </div>
                        <div className="pb-4 flex-1 min-w-0">
                          <p className="text-xs text-white">{entry.description}</p>
                          <p className="text-[10px] text-[hsl(0_0%_45%)]">{formatDate(entry.created_at)}</p>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>

                {/* Generated Letters */}
                <div className="rounded-lg border border-[hsl(220_15%_22%/0.6)] bg-[hsl(220_15%_14%/0.7)] p-4">
                  <div className="flex items-center justify-between mb-3">
                    <h4 className="text-xs font-semibold text-white uppercase tracking-wider">Generated Letters</h4>
                    <button
                      onClick={() => handleOpenLetterModal(selectedCaseId)}
                      className="rounded p-1 text-[hsl(0_0%_55%)] hover:text-white hover:bg-[hsl(220_15%_18%)] transition-colors"
                    >
                      <Plus size={13} />
                    </button>
                  </div>
                  {caseLetters && caseLetters.length > 0 ? (
                    <div className="space-y-2">
                      {caseLetters.map((letter) => (
                        <div key={letter.id} className="rounded border border-[hsl(220_15%_22%/0.4)] bg-[hsl(220_15%_10%/0.4)] p-3">
                          <div className="flex items-center justify-between mb-1">
                            <span className="text-xs font-mono text-[hsl(220_90%_60%)]">{letter.template_code}</span>
                            <div className="flex items-center gap-1">
                              {letter.sent_at ? (
                                <span className="text-[10px] text-success flex items-center gap-1">
                                  <CheckCircle2 size={10} /> Sent {formatDate(letter.sent_at)}
                                </span>
                              ) : (
                                <>
                                  <button
                                    onClick={() => handleMarkSent(letter.id)}
                                    className="rounded p-1 text-[hsl(0_0%_55%)] hover:text-info hover:bg-[hsl(220_15%_18%)] transition-colors"
                                    title="Mark as Sent"
                                  >
                                    <Send size={12} />
                                  </button>
                                  <button
                                    onClick={() => handleOpenLetterModal(selectedCaseId)}
                                    className="rounded p-1 text-[hsl(0_0%_55%)] hover:text-white hover:bg-[hsl(220_15%_18%)] transition-colors"
                                    title="Print Letter"
                                  >
                                    <Printer size={12} />
                                  </button>
                                </>
                              )}
                            </div>
                          </div>
                          <p className="text-[10px] text-[hsl(0_0%_50%)]">
                            To: {letter.recipient_name} | {formatDate(letter.created_at)}
                          </p>
                          {letter.tracking_number && (
                            <p className="text-[10px] text-info">Tracking: {letter.tracking_number}</p>
                          )}
                          <div className="mt-2">
                            <DisputeLetterPreview letter={letter} />
                          </div>
                        </div>
                      ))}
                    </div>
                  ) : (
                    <p className="text-xs text-[hsl(0_0%_45%)] py-4 text-center">No letters generated yet</p>
                  )}
                </div>
              </motion.div>
            )}
          </div>

          {/* Derogatory Items */}
          {derogatory && derogatory.length > 0 && (
            <CollapsibleSection title="Derogatory Items" count={derogatory.length} defaultOpen={false}>
              <div className="overflow-x-auto">
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b border-[hsl(220_15%_22%/0.4)] text-[10px] uppercase tracking-wider text-[hsl(0_0%_45%)]">
                      <th className="text-left px-4 py-2 font-medium">Creditor</th>
                      <th className="text-left px-4 py-2 font-medium">Type</th>
                      <th className="text-left px-4 py-2 font-medium">Bureau</th>
                      <th className="text-left px-4 py-2 font-medium">Reported</th>
                      <th className="text-left px-4 py-2 font-medium">Removal</th>
                      <th className="text-left px-4 py-2 font-medium">Description</th>
                    </tr>
                  </thead>
                  <tbody>
                    {derogatory.map((d) => (
                      <tr key={d.id} className="border-b border-[hsl(220_15%_22%/0.2)] hover:bg-[hsl(220_15%_18%/0.3)]">
                        <td className="px-4 py-2.5 text-xs text-white">{d.creditor_name}</td>
                        <td className="px-4 py-2.5">
                          <span className={cn(
                            "text-[10px] px-1.5 py-0.5 rounded font-medium",
                            d.derogatory_type === "late_payment" ? "bg-warning/10 text-warning" :
                            d.derogatory_type === "collection" ? "bg-danger/10 text-danger" :
                            "bg-info/10 text-info",
                          )}>
                            {d.derogatory_type.replace(/_/g, " ")}
                          </span>
                        </td>
                        <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_60%)]">{d.bureau}</td>
                        <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_50%)]">{formatDate(d.date_reported)}</td>
                        <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_50%)]">{formatDate(d.date_expected_removal)}</td>
                        <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_60%)]">{d.description}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </CollapsibleSection>
          )}
        </>
      )}

      {/* Accounts Tab */}
      {selectedTab === "accounts" && (
        <>
          <div className="flex flex-wrap items-center gap-2">
            <button
              onClick={handleCreateCreditAccount}
              disabled={createCreditAccount.isPending}
              className="inline-flex items-center gap-2 rounded bg-[hsl(220_90%_56%)] px-3 py-2 text-xs font-medium text-white hover:bg-[hsl(220_90%_48%)] transition-colors disabled:opacity-50"
            >
              {createCreditAccount.isPending ? <Loader2 size={14} className="animate-spin" /> : <Plus size={14} />}
              Add Credit Account
            </button>
            <button
              onClick={handleCreateInquiry}
              disabled={createCreditInquiry.isPending}
              className="inline-flex items-center gap-2 rounded border border-[hsl(220_15%_22%)] px-3 py-2 text-xs font-medium text-white hover:bg-[hsl(220_15%_18%)] transition-colors disabled:opacity-50"
            >
              {createCreditInquiry.isPending ? <Loader2 size={14} className="animate-spin" /> : <Plus size={14} />}
              Add Inquiry
            </button>
            <button
              onClick={handleCreateDerogatory}
              disabled={createDerogatoryItem.isPending}
              className="inline-flex items-center gap-2 rounded border border-[hsl(220_15%_22%)] px-3 py-2 text-xs font-medium text-white hover:bg-[hsl(220_15%_18%)] transition-colors disabled:opacity-50"
            >
              {createDerogatoryItem.isPending ? <Loader2 size={14} className="animate-spin" /> : <Plus size={14} />}
              Add Derogatory
            </button>
          </div>

          {/* Credit Accounts */}
          <CollapsibleSection title="Credit Accounts" count={creditAccounts?.length}>
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b border-[hsl(220_15%_22%/0.4)] text-[10px] uppercase tracking-wider text-[hsl(0_0%_45%)]">
                    <th className="text-left px-4 py-2 font-medium">Creditor</th>
                    <th className="text-left px-4 py-2 font-medium">Type</th>
                    <th className="text-left px-4 py-2 font-medium">Bureau</th>
                    <th className="text-right px-4 py-2 font-medium">Balance</th>
                    <th className="text-right px-4 py-2 font-medium">Limit</th>
                    <th className="text-right px-4 py-2 font-medium">Utilization</th>
                    <th className="text-left px-4 py-2 font-medium">Status</th>
                  </tr>
                </thead>
                <tbody>
                  {creditAccounts?.map((a) => {
                    const util = a.credit_limit_cents > 0 ? (a.balance_cents / a.credit_limit_cents) * 100 : 0;
                    const utilColor = util > 50 ? "text-danger" : util > 30 ? "text-warning" : "text-success";
                    return (
                      <tr key={a.id} className="border-b border-[hsl(220_15%_22%/0.2)] hover:bg-[hsl(220_15%_18%/0.3)]">
                        <td className="px-4 py-2.5 text-xs text-white">{a.creditor_name}</td>
                        <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_60%)]">{a.account_type}</td>
                        <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_60%)]">{a.bureau}</td>
                        <td className="px-4 py-2.5 text-xs text-right text-white">{formatCurrency(a.balance_cents)}</td>
                        <td className="px-4 py-2.5 text-xs text-right text-[hsl(0_0%_60%)]">{formatCurrency(a.credit_limit_cents)}</td>
                        <td className={cn("px-4 py-2.5 text-xs text-right font-medium", utilColor)}>{util.toFixed(0)}%</td>
                        <td className="px-4 py-2.5">
                          <span className={cn(
                            "text-[10px] px-1.5 py-0.5 rounded font-medium",
                            a.payment_status === "current" ? "bg-success/10 text-success" :
                            a.payment_status === "collections" ? "bg-danger/10 text-danger" :
                            "bg-warning/10 text-warning",
                          )}>
                            {a.payment_status}
                          </span>
                        </td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            </div>
          </CollapsibleSection>

          {/* Inquiries */}
          <CollapsibleSection title="Recent Inquiries" count={inquiries?.length} defaultOpen={false}>
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b border-[hsl(220_15%_22%/0.4)] text-[10px] uppercase tracking-wider text-[hsl(0_0%_45%)]">
                    <th className="text-left px-4 py-2 font-medium">Creditor</th>
                    <th className="text-left px-4 py-2 font-medium">Bureau</th>
                    <th className="text-left px-4 py-2 font-medium">Date</th>
                    <th className="text-left px-4 py-2 font-medium">Type</th>
                  </tr>
                </thead>
                <tbody>
                  {inquiries?.map((inq) => (
                    <tr key={inq.id} className="border-b border-[hsl(220_15%_22%/0.2)] hover:bg-[hsl(220_15%_18%/0.3)]">
                      <td className="px-4 py-2.5 text-xs text-white">{inq.creditor_name}</td>
                      <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_60%)]">{inq.bureau}</td>
                      <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_50%)]">{formatDate(inq.inquiry_date)}</td>
                      <td className="px-4 py-2.5">
                        <span className={cn(
                          "text-[10px] px-1.5 py-0.5 rounded font-medium",
                          inq.inquiry_type === "hard" ? "bg-warning/10 text-warning" : "bg-info/10 text-info",
                        )}>
                          {inq.inquiry_type}
                        </span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </CollapsibleSection>

          {/* Derogatory Items */}
          <CollapsibleSection title="Derogatory Items" count={derogatory?.length} defaultOpen={false}>
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b border-[hsl(220_15%_22%/0.4)] text-[10px] uppercase tracking-wider text-[hsl(0_0%_45%)]">
                    <th className="text-left px-4 py-2 font-medium">Creditor</th>
                    <th className="text-left px-4 py-2 font-medium">Type</th>
                    <th className="text-left px-4 py-2 font-medium">Bureau</th>
                    <th className="text-left px-4 py-2 font-medium">Expected Removal</th>
                    <th className="text-left px-4 py-2 font-medium">Description</th>
                  </tr>
                </thead>
                <tbody>
                  {derogatory?.map((d) => (
                    <tr key={d.id} className="border-b border-[hsl(220_15%_22%/0.2)] hover:bg-[hsl(220_15%_18%/0.3)]">
                      <td className="px-4 py-2.5 text-xs text-white">{d.creditor_name}</td>
                      <td className="px-4 py-2.5">
                        <span className={cn(
                          "text-[10px] px-1.5 py-0.5 rounded font-medium",
                          d.derogatory_type === "late_payment" ? "bg-warning/10 text-warning" :
                          d.derogatory_type === "collection" ? "bg-danger/10 text-danger" :
                          "bg-info/10 text-info",
                        )}>
                          {d.derogatory_type.replace(/_/g, " ")}
                        </span>
                      </td>
                      <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_60%)]">{d.bureau}</td>
                      <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_50%)]">{formatDate(d.date_expected_removal)}</td>
                      <td className="px-4 py-2.5 text-xs text-[hsl(0_0%_60%)]">{d.description}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </CollapsibleSection>
        </>
      )}

      {/* Business Tab */}
      {selectedTab === "business" && (
        <>
          <div className="flex flex-wrap items-center gap-2">
            <button
              onClick={handleCreateBusinessProfile}
              disabled={createBusinessProfile.isPending}
              className="inline-flex items-center gap-2 rounded bg-[hsl(220_90%_56%)] px-3 py-2 text-xs font-medium text-white hover:bg-[hsl(220_90%_48%)] transition-colors disabled:opacity-50"
            >
              {createBusinessProfile.isPending ? <Loader2 size={14} className="animate-spin" /> : <Plus size={14} />}
              Add Business Profile
            </button>
          </div>

          {businessProfiles && businessProfiles.length > 0 ? (
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
              {businessProfiles.map((bp) => (
                <div key={bp.id} className="rounded-lg border border-[hsl(220_15%_22%/0.6)] bg-[hsl(220_15%_14%/0.7)] p-4">
                  <div className="flex items-start justify-between mb-3">
                    <div>
                      <h3 className="text-sm font-semibold text-white">{bp.business_name}</h3>
                      <p className="text-[10px] text-[hsl(0_0%_45%)]">{bp.industry}</p>
                    </div>
                    <span className="text-[10px] text-[hsl(0_0%_45%)] px-1.5 py-0.5 rounded bg-[hsl(220_15%_22%/0.5)]">{bp.data_provider}</span>
                  </div>
                  <div className="grid grid-cols-3 gap-3 text-center">
                    <div className="rounded bg-[hsl(220_15%_10%/0.5)] p-2">
                      <p className="text-lg font-bold text-white">{bp.business_credit_score}</p>
                      <p className="text-[9px] text-[hsl(0_0%_45%)]">Biz Score</p>
                    </div>
                    <div className="rounded bg-[hsl(220_15%_10%/0.5)] p-2">
                      <p className="text-lg font-bold text-white">{bp.paydex_score ?? "—"}</p>
                      <p className="text-[9px] text-[hsl(0_0%_45%)]">PayDex</p>
                    </div>
                    <div className="rounded bg-[hsl(220_15%_10%/0.5)] p-2">
                      <p className="text-lg font-bold text-white">{formatCurrency(bp.credit_limit_cents)}</p>
                      <p className="text-[9px] text-[hsl(0_0%_45%)]">Credit Limit</p>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="rounded-lg border border-[hsl(220_15%_22%/0.6)] bg-[hsl(220_15%_14%/0.7)] p-8 text-center">
              <Briefcase size={24} className="mx-auto text-[hsl(0_0%_40%)] mb-2" />
              <p className="text-sm text-[hsl(0_0%_55%)]">No business credit profiles found</p>
            </div>
          )}
        </>
      )}

      {/* Letter Generation Modal */}
      <LetterGenerationModal
        isOpen={showLetterModal}
        onClose={() => { setShowLetterModal(false); setLetterModalCaseId(""); }}
        caseId={letterModalCaseId}
        onGenerated={() => {
          setSelectedCaseId(letterModalCaseId);
        }}
      />
    </motion.div>
  );
}
