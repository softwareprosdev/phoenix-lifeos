import { safeInvoke } from "./commandClient";
import type {
  CreditReport, ScoreHistoryPoint,
  DisputeCase, DisputeLetter, DisputeLetterTemplate,
  DisputeTimelineEntry, DisputeCaseDetail, GenerateLetterInput,
  CreditAccount, CreditInquiry, CreditDerogatory,
  BusinessCreditProfile, NegativeItem,
} from "@/types";

type RawDisputeCase = {
  id: string;
  negative_item_id?: string | null;
  dispute_type: string;
  status: string;
  status_basis?: string | null;
  bureau: string;
  statute_basis?: string | null;
  notes?: string | null;
  creditor_name?: string | null;
  created_at: string;
  updated_at: string;
  timeline?: RawTimelineEntry[];
};

type RawTimelineEntry = {
  id: string;
  dispute_case_id?: string;
  case_id?: string;
  event_type: string;
  description: string;
  created_at: string;
};

type RawDisputeLetter = {
  id: string;
  dispute_case_id?: string;
  case_id?: string;
  template_id?: string | null;
  template_code?: string;
  recipient_type?: string;
  recipient_name?: string;
  recipient_address?: string | null;
  generated_content_encrypted?: string | null;
  content?: string;
  tracking_number?: string | null;
  sent_date?: string | null;
  sent_at?: string | null;
  created_at: string;
};

type RawTemplate = {
  id: string;
  template_code: string;
  name: string;
  description?: string | null;
  legal_basis?: string | null;
  recipient_type: string;
  body_template?: string;
  default_content?: string;
  variable_schema?: Record<string, string> | string | null;
  created_at?: string;
};

type RawCreditAccount = Partial<CreditAccount> & {
  name?: string;
  issuer?: string;
  apr?: string;
  opened_date?: string | null;
};

type RawDerogatory = Partial<CreditDerogatory> & {
  item_type?: string;
  status?: string;
  opened_date?: string | null;
  expected_removal_date?: string | null;
  amount_cents?: number;
};

type RawBusinessProfile = Partial<BusinessCreditProfile> & {
  duns_number?: string | null;
};

function normalizeDisputeCase(raw: RawDisputeCase | DisputeCase): DisputeCase {
  if ("case_number" in raw) return raw;

  return {
    id: raw.id,
    case_number: `CR-${raw.id.slice(0, 8).toUpperCase()}`,
    dispute_type: raw.dispute_type,
    status: raw.status,
    creditor_name: raw.creditor_name ?? "Unknown creditor",
    bureau: raw.bureau,
    reason: raw.status_basis ?? raw.statute_basis ?? raw.dispute_type,
    notes: raw.notes ?? "",
    created_at: raw.created_at,
    updated_at: raw.updated_at,
    won_at: raw.status === "won" ? raw.updated_at : null,
  };
}

function normalizeTimelineEntry(raw: RawTimelineEntry | DisputeTimelineEntry): DisputeTimelineEntry {
  return {
    id: raw.id,
    case_id: ("case_id" in raw ? raw.case_id : raw.dispute_case_id) ?? "",
    event_type: raw.event_type,
    description: raw.description,
    created_at: raw.created_at,
  };
}

function normalizeDisputeLetter(raw: RawDisputeLetter | DisputeLetter): DisputeLetter {
  if ("case_id" in raw && "content" in raw && raw.case_id) return raw as DisputeLetter;
  const letter = raw as RawDisputeLetter;

  return {
    id: letter.id,
    case_id: letter.case_id ?? letter.dispute_case_id ?? "",
    template_id: letter.template_id ?? "",
    template_code: letter.template_code ?? letter.recipient_type ?? "CUSTOM",
    recipient_name: letter.recipient_name ?? "Credit Bureau",
    recipient_address: letter.recipient_address ?? "",
    content: letter.content ?? letter.generated_content_encrypted ?? "",
    tracking_number: letter.tracking_number ?? null,
    sent_at: letter.sent_at ?? letter.sent_date ?? null,
    created_at: letter.created_at,
  };
}

function normalizeTemplate(raw: RawTemplate | DisputeLetterTemplate): DisputeLetterTemplate {
  if ("default_content" in raw && typeof raw.variable_schema === "object") {
    return raw as DisputeLetterTemplate;
  }

  let variableSchema: Record<string, string> = {};
  if (typeof raw.variable_schema === "string") {
    try {
      const parsed = JSON.parse(raw.variable_schema);
      variableSchema = Array.isArray(parsed)
        ? Object.fromEntries(parsed.map((key) => [key, key.replace(/_/g, " ")]))
        : parsed;
    } catch {
      variableSchema = {};
    }
  } else if (raw.variable_schema) {
    variableSchema = raw.variable_schema;
  }

  return {
    id: raw.id,
    template_code: raw.template_code,
    name: raw.name,
    description: raw.description ?? "",
    legal_basis: raw.legal_basis ?? "FCRA",
    recipient_type: raw.recipient_type,
    variable_schema: variableSchema,
    default_content:
      "body_template" in raw
        ? raw.default_content ?? raw.body_template ?? ""
        : raw.default_content ?? "",
    created_at: raw.created_at ?? new Date().toISOString(),
  };
}

function normalizeCreditAccount(raw: RawCreditAccount): CreditAccount {
  return {
    id: raw.id ?? "",
    creditor_name: raw.creditor_name ?? raw.issuer ?? raw.name ?? "Unknown creditor",
    account_type: raw.account_type ?? "credit_card",
    account_number_masked: raw.account_number_masked ?? "****",
    bureau: raw.bureau ?? "Experian",
    opened_date: raw.opened_date ?? raw.created_at ?? new Date().toISOString(),
    reported_date: raw.reported_date ?? new Date().toISOString(),
    credit_limit_cents: raw.credit_limit_cents ?? 0,
    balance_cents: raw.balance_cents ?? 0,
    past_due_cents: raw.past_due_cents ?? 0,
    payment_status: raw.payment_status ?? "current",
    account_status: raw.account_status ?? "open",
    remarks: raw.remarks ?? "",
    created_at: raw.created_at ?? new Date().toISOString(),
  };
}

function normalizeDerogatory(raw: RawDerogatory): CreditDerogatory {
  return {
    id: raw.id ?? "",
    bureau: raw.bureau ?? "Experian",
    derogatory_type: raw.derogatory_type ?? raw.item_type ?? "derogatory",
    creditor_name: raw.creditor_name ?? "Unknown creditor",
    account_number_masked: raw.account_number_masked ?? "****",
    date_reported: raw.date_reported ?? raw.opened_date ?? raw.created_at ?? new Date().toISOString(),
    date_expected_removal: raw.date_expected_removal ?? raw.expected_removal_date ?? new Date().toISOString(),
    description: raw.description ?? raw.status ?? "",
    created_at: raw.created_at ?? new Date().toISOString(),
  };
}

function normalizeBusinessProfile(raw: RawBusinessProfile): BusinessCreditProfile {
  return {
    id: raw.id ?? "",
    business_name: raw.business_name ?? "Business Profile",
    ein: raw.ein ?? "",
    industry: raw.industry ?? "General",
    business_credit_score: raw.business_credit_score ?? 0,
    paydex_score: raw.paydex_score ?? null,
    credit_limit_cents: raw.credit_limit_cents ?? 0,
    data_provider: raw.data_provider ?? "Manual",
    created_at: raw.created_at ?? new Date().toISOString(),
    updated_at: raw.updated_at ?? raw.created_at ?? new Date().toISOString(),
  };
}

export const creditService = {
  getLatestReport: () => safeInvoke<CreditReport | null>("Credit", "get_latest_credit_report"),

  getReport: (id: string) => safeInvoke<CreditReport>("Credit", "get_credit_report", { id }),

  listReports: (startDate?: string, endDate?: string) =>
    safeInvoke<CreditReport[]>("Credit", "list_credit_reports", { startDate, endDate }),

  getScoreHistory: async () => {
    const history = await safeInvoke<Array<ScoreHistoryPoint & { report_date?: string }>>(
      "Credit",
      "get_credit_score_history",
    );
    return history.map((point) => ({
      date: point.date ?? point.report_date ?? new Date().toISOString(),
      score: point.score,
      bureau: point.bureau,
    }));
  },

  createReport: (input: { score: number; bureau: string; score_model?: string }) =>
    safeInvoke<CreditReport>("Credit", "create_credit_report", {
      payload: {
        utilization_rate: 0,
        on_time_rate: 100,
        ...input,
      },
    }),

  deleteReport: (id: string) => safeInvoke<void>("Credit", "delete_credit_report", { id }),

  listDisputeCases: async () => {
    const cases = await safeInvoke<RawDisputeCase[]>("Credit", "list_dispute_cases");
    return cases.map(normalizeDisputeCase);
  },

  createDisputeCase: (input: Partial<DisputeCase>) =>
    safeInvoke<RawDisputeCase>("Credit", "create_dispute_case", {
      payload: {
        dispute_type: input.dispute_type ?? "inaccurate_information",
        bureau: input.bureau ?? "Experian",
        notes: input.notes,
      },
    }).then(normalizeDisputeCase),

  getDisputeCase: async (id: string) => {
    const detail = await safeInvoke<DisputeCaseDetail | RawDisputeCase | null>("Credit", "get_dispute_case", { id });
    if (!detail) {
      throw new Error(`Dispute case not found: ${id}`);
    }
    if ("case" in detail) return detail;

    return {
      case: normalizeDisputeCase(detail),
      letters: [],
      timeline: (detail.timeline ?? []).map(normalizeTimelineEntry),
      negative_item: null as NegativeItem | null,
    };
  },

  updateDisputeCase: (id: string, input: Partial<DisputeCase>) =>
    safeInvoke<RawDisputeCase>("Credit", "update_dispute_case", {
      id,
      payload: {
        status: input.status,
        status_basis: input.reason,
        notes: input.notes,
      },
    }).then(normalizeDisputeCase),

  listDisputeLetters: (caseId: string) =>
    safeInvoke<RawDisputeLetter[]>("Credit", "list_dispute_letters", { caseId })
      .then((letters) => letters.map(normalizeDisputeLetter)),

  generateLetter: (input: GenerateLetterInput) =>
    safeInvoke<RawDisputeLetter>("Credit", "generate_letter", { payload: input })
      .then(normalizeDisputeLetter),

  listLetterTemplates: () =>
    safeInvoke<RawTemplate[]>("Credit", "list_letter_templates")
      .then((templates) => templates.map(normalizeTemplate)),

  getLetterTemplateByCode: (code: string) =>
    safeInvoke<RawTemplate | null>("Credit", "get_letter_template_by_code", { code })
      .then((template) => (template ? normalizeTemplate(template) : null)),

  markLetterSent: (letterId: string, trackingNumber: string) =>
    safeInvoke<RawDisputeLetter>("Credit", "mark_letter_sent", {
      payload: { letter_id: letterId, tracking_number: trackingNumber },
    }).then(normalizeDisputeLetter),

  addTimelineEntry: (caseId: string, eventType: string, description: string) =>
    safeInvoke<RawTimelineEntry>("Credit", "add_timeline_entry", {
      payload: { case_id: caseId, event_type: eventType, description },
    }).then(normalizeTimelineEntry),

  listCreditAccounts: () =>
    safeInvoke<RawCreditAccount[]>("Credit", "get_credit_accounts")
      .then((accounts) => accounts.map(normalizeCreditAccount)),

  createCreditAccount: (input: Partial<CreditAccount>) =>
    safeInvoke<RawCreditAccount>("Credit", "create_credit_account", {
      payload: {
        name: input.creditor_name ?? "Credit Account",
        issuer: input.creditor_name ?? "Unknown issuer",
        account_type: input.account_type ?? "revolving",
        credit_limit_cents: input.credit_limit_cents ?? 0,
        balance_cents: input.balance_cents ?? 0,
        apr: "0",
        payment_status: input.payment_status ?? "current",
        opened_date: input.opened_date,
      },
    }).then(normalizeCreditAccount),

  deleteCreditAccount: (id: string) =>
    safeInvoke<void>("Credit", "delete_credit_account", { id }),

  listCreditInquiries: () => safeInvoke<CreditInquiry[]>("Credit", "list_credit_inquiries"),

  createCreditInquiry: (input: Partial<CreditInquiry>) =>
    safeInvoke<CreditInquiry>("Credit", "create_credit_inquiry", {
      payload: {
        bureau: input.bureau ?? "Experian",
        creditor_name: input.creditor_name ?? "Unknown creditor",
        inquiry_type: input.inquiry_type ?? "hard",
        inquiry_date: input.inquiry_date ?? new Date().toISOString(),
      },
    }),

  listDerogatoryItems: () =>
    safeInvoke<RawDerogatory[]>("Credit", "list_derogatory_items")
      .then((items) => items.map(normalizeDerogatory)),

  createDerogatoryItem: (input: Partial<CreditDerogatory>) =>
    safeInvoke<RawDerogatory>("Credit", "create_derogatory_item", {
      payload: {
        creditor_name: input.creditor_name ?? "Unknown creditor",
        account_number_encrypted: input.account_number_masked,
        item_type: input.derogatory_type ?? "collection",
        amount_cents: 0,
        original_amount_cents: 0,
        status: input.description ?? "disputed",
        opened_date: input.date_reported,
        expected_removal_date: input.date_expected_removal,
        bureau: input.bureau ?? "Experian",
      },
    }).then(normalizeDerogatory),

  getBusinessCreditProfiles: () =>
    safeInvoke<RawBusinessProfile[]>("Credit", "get_business_credit_profiles")
      .then((profiles) => profiles.map(normalizeBusinessProfile)),

  createBusinessCreditProfile: (input: Partial<BusinessCreditProfile>) =>
    safeInvoke<RawBusinessProfile>("Credit", "create_business_credit_profile", {
      payload: {
        business_name: input.business_name ?? "Business Profile",
        ein_encrypted: input.ein,
        paydex_score: input.paydex_score,
        business_credit_score: input.business_credit_score,
      },
    }).then(normalizeBusinessProfile),
};
