import type {
  Account,
  Budget,
  BudgetItem,
  BudgetProgress,
  BudgetWithItems,
  CashFlow,
  Category,
  Contact,
  CreditAccount,
  CreditDerogatory,
  CreditInquiry,
  CreditReport,
  Debt,
  DebtSummary,
  DisputeCase,
  DisputeCaseDetail,
  DisputeLetter,
  DisputeLetterTemplate,
  DisputeTimelineEntry,
  BusinessCreditProfile,
  Goal,
  Investment,
  SpendingByCategory,
  Transaction,
} from "@/types";

const now = new Date();
const iso = (daysOffset = 0) => {
  const d = new Date(now);
  d.setDate(d.getDate() + daysOffset);
  return d.toISOString();
};

const date = (daysOffset = 0) => iso(daysOffset).slice(0, 10);

const accounts: Account[] = [
  {
    id: "demo-checking",
    name: "Main Checking",
    account_type: "checking",
    institution: "First National Bank",
    balance_cents: 425000,
    currency: "USD",
    is_active: true,
    created_at: iso(-90),
    updated_at: iso(),
  },
  {
    id: "demo-savings",
    name: "High-Yield Savings",
    account_type: "savings",
    institution: "Online Savings Co",
    balance_cents: 1500000,
    currency: "USD",
    is_active: true,
    created_at: iso(-90),
    updated_at: iso(),
  },
  {
    id: "demo-card",
    name: "Platinum Rewards Card",
    account_type: "credit",
    institution: "Chase",
    balance_cents: -34500,
    currency: "USD",
    is_active: true,
    created_at: iso(-90),
    updated_at: iso(),
  },
  {
    id: "demo-brokerage",
    name: "Brokerage Account",
    account_type: "investment",
    institution: "Vanguard",
    balance_cents: 5000000,
    currency: "USD",
    is_active: true,
    created_at: iso(-90),
    updated_at: iso(),
  },
];

const categories: Category[] = [
  { id: "cat-salary", name: "Salary", type: "income", created_at: iso(-90) },
  { id: "cat-groceries", name: "Groceries", type: "expense", created_at: iso(-90) },
  { id: "cat-rent", name: "Rent", type: "expense", created_at: iso(-90) },
  { id: "cat-dining", name: "Dining Out", type: "expense", created_at: iso(-90) },
  { id: "cat-transport", name: "Transport", type: "expense", created_at: iso(-90) },
];

const transactions: Transaction[] = [
  {
    id: "tx-paycheck",
    account_id: "demo-checking",
    amount_cents: 850000,
    currency: "USD",
    transaction_type: "income",
    category_id: "cat-salary",
    merchant: "Employer Inc",
    description: "Salary deposit",
    date: date(-2),
    tags: [],
    is_recurring: true,
    created_at: iso(-2),
    updated_at: iso(-2),
  },
  {
    id: "tx-rent",
    account_id: "demo-checking",
    amount_cents: -250000,
    currency: "USD",
    transaction_type: "expense",
    category_id: "cat-rent",
    merchant: "Landlord",
    description: "Monthly rent",
    date: date(-5),
    tags: [],
    is_recurring: true,
    created_at: iso(-5),
    updated_at: iso(-5),
  },
  {
    id: "tx-groceries",
    account_id: "demo-checking",
    amount_cents: -12500,
    currency: "USD",
    transaction_type: "expense",
    category_id: "cat-groceries",
    merchant: "Whole Foods",
    description: "Grocery run",
    date: date(-4),
    tags: [],
    is_recurring: false,
    created_at: iso(-4),
    updated_at: iso(-4),
  },
  {
    id: "tx-dinner",
    account_id: "demo-card",
    amount_cents: -6500,
    currency: "USD",
    transaction_type: "expense",
    category_id: "cat-dining",
    merchant: "Italian Place",
    description: "Dinner out",
    date: date(-1),
    tags: [],
    is_recurring: false,
    created_at: iso(-1),
    updated_at: iso(-1),
  },
];

const budgets: Budget[] = [
  {
    id: "budget-monthly",
    name: "Monthly Living",
    period: "monthly",
    total_limit_cents: 300000,
    currency: "USD",
    start_date: date(-30),
    end_date: date(30),
    created_at: iso(-30),
  },
];

const budgetItems: BudgetItem[] = [
  {
    id: "budget-item-groceries",
    budget_id: "budget-monthly",
    category_id: "cat-groceries",
    limit_cents: 60000,
    spent_cents: 12500,
  },
  {
    id: "budget-item-dining",
    budget_id: "budget-monthly",
    category_id: "cat-dining",
    limit_cents: 30000,
    spent_cents: 6500,
  },
];

const debts: Debt[] = [
  {
    id: "debt-auto",
    name: "Auto Loan",
    debt_type: "auto",
    principal_cents: 3500000,
    remaining_balance_cents: 3150000,
    interest_rate: "4.5",
    minimum_payment_cents: 45000,
    currency: "USD",
    due_date: date(10),
    creditor: "Capital One Auto",
    created_at: iso(-120),
    updated_at: iso(),
  },
  {
    id: "debt-card",
    name: "Credit Card Balance",
    debt_type: "credit_card",
    principal_cents: 50000,
    remaining_balance_cents: 34500,
    interest_rate: "22.99",
    minimum_payment_cents: 5000,
    currency: "USD",
    due_date: date(7),
    creditor: "Chase",
    created_at: iso(-60),
    updated_at: iso(),
  },
];

const goals: Goal[] = [
  {
    id: "goal-emergency",
    name: "Emergency Fund",
    goal_type: "emergency_fund",
    target_amount_cents: 3000000,
    current_amount_cents: 1500000,
    monthly_contribution_cents: 200000,
    currency: "USD",
    deadline: date(365),
    linked_account_id: "demo-savings",
    created_at: iso(-120),
    updated_at: iso(),
  },
  {
    id: "goal-vacation",
    name: "Vacation Fund",
    goal_type: "vacation",
    target_amount_cents: 500000,
    current_amount_cents: 120000,
    monthly_contribution_cents: 50000,
    currency: "USD",
    deadline: date(180),
    linked_account_id: "demo-savings",
    created_at: iso(-45),
    updated_at: iso(),
  },
];

const creditReports: CreditReport[] = [
  {
    id: "credit-current",
    score: 742,
    score_model: "FICO_8",
    bureau: "Experian",
    utilization_rate: "12.5",
    on_time_rate: "100",
    derogatory_marks: 0,
    total_accounts: 8,
    open_accounts: 5,
    hard_inquiries_12m: 2,
    report_date: iso(-1),
    created_at: iso(-1),
  },
];

const disputeCases: DisputeCase[] = [
  {
    id: "case-collection-portfolio",
    case_number: "CR-2026-001",
    dispute_type: "collection_not_mine",
    status: "in_progress",
    creditor_name: "Portfolio Recovery Associates",
    bureau: "Experian",
    reason: "Account is not recognized and validation has not been provided.",
    notes: "Request debt validation and bureau reinvestigation.",
    created_at: iso(-18),
    updated_at: iso(-6),
    won_at: null,
  },
  {
    id: "case-late-capital",
    case_number: "CR-2026-002",
    dispute_type: "late_payment_error",
    status: "draft",
    creditor_name: "Capital One",
    bureau: "TransUnion",
    reason: "Reported late payment conflicts with bank payment confirmation.",
    notes: "Attach payment confirmation before sending.",
    created_at: iso(-5),
    updated_at: iso(-5),
    won_at: null,
  },
  {
    id: "case-medical-won",
    case_number: "CR-2026-003",
    dispute_type: "medical_collection_paid",
    status: "won",
    creditor_name: "Regional Medical Collections",
    bureau: "Equifax",
    reason: "Paid medical collection should be removed from report.",
    notes: "Deleted after bureau reinvestigation.",
    created_at: iso(-45),
    updated_at: iso(-12),
    won_at: iso(-12),
  },
];

const disputeTemplates: DisputeLetterTemplate[] = [
  {
    id: "tpl-609",
    template_code: "FCRA-609",
    name: "Section 609 Credit File Request",
    description: "Request disclosure of the source and method of verification for disputed items.",
    legal_basis: "FCRA Section 609",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      consumer_address: "Consumer address",
      account_name: "Account name",
      dispute_reason: "Dispute reason",
    },
    default_content:
      "I am requesting disclosure of all information in my credit file related to {{account_name}}. The item is disputed because {{dispute_reason}}.",
    created_at: iso(-60),
  },
  {
    id: "tpl-611",
    template_code: "FCRA-611",
    name: "Reinvestigation Dispute",
    description: "Formal bureau dispute requesting reinvestigation of inaccurate reporting.",
    legal_basis: "FCRA Section 611",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      account_name: "Account name",
      dispute_reason: "Dispute reason",
    },
    default_content:
      "Please reinvestigate the inaccurate {{account_name}} entry. The basis for dispute is: {{dispute_reason}}.",
    created_at: iso(-60),
  },
  {
    id: "tpl-dv",
    template_code: "FDCPA-DV",
    name: "Debt Validation Request",
    description: "Request validation from a debt collector before further collection activity.",
    legal_basis: "FDCPA Section 809",
    recipient_type: "collector",
    variable_schema: {
      consumer_name: "Consumer name",
      collector_name: "Collector name",
      account_name: "Account name",
    },
    default_content:
      "I dispute this debt and request validation for {{account_name}} before any further collection activity.",
    created_at: iso(-60),
  },
];

disputeTemplates.push(
  {
    id: "tpl-t13",
    template_code: "T13",
    name: "Method of Verification Request",
    description: "Request the bureau explain how a previously disputed item was verified.",
    legal_basis: "FCRA Section 611(a)(6) and 611(a)(7)",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      creditor_name: "Creditor or furnisher",
      account_number: "Account number",
      dispute_reference: "Dispute reference",
    },
    default_content:
      "Please provide the method of verification used for {{creditor_name}} account {{account_number}}, including who was contacted and what records were reviewed.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t14",
    template_code: "T14",
    name: "Reinvestigation Results Challenge",
    description: "Challenge a completed dispute when the result did not correct the error.",
    legal_basis: "FCRA Section 611",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      creditor_name: "Creditor or furnisher",
      account_number: "Account number",
      challenge_reason: "Challenge reason",
    },
    default_content:
      "I challenge your reinvestigation result for {{creditor_name}} account {{account_number}} because {{challenge_reason}}. Please reinvestigate and correct or delete the item.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t15",
    template_code: "T15",
    name: "Updated Documentation Submission",
    description: "Submit additional evidence for an existing credit report dispute.",
    legal_basis: "FCRA Section 611",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      dispute_reference: "Dispute reference",
      evidence_summary: "Evidence summary",
    },
    default_content:
      "Please add the enclosed documentation to dispute {{dispute_reference}}. The new evidence is: {{evidence_summary}}.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t16",
    template_code: "T16",
    name: "Mixed File Wrong Person Dispute",
    description: "Dispute information belonging to another consumer.",
    legal_basis: "FCRA Section 611",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      item_description: "Item description",
      wrong_person_details: "Wrong person details",
    },
    default_content:
      "The item {{item_description}} appears to belong to another consumer. The mismatched details are {{wrong_person_details}}. Please delete the mixed-file information.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t17",
    template_code: "T17",
    name: "Duplicate Account Dispute",
    description: "Dispute duplicate reporting of the same account or debt.",
    legal_basis: "FCRA Section 611",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      correct_item: "Correct item",
      duplicate_items: "Duplicate items",
    },
    default_content:
      "The item {{duplicate_items}} duplicates {{correct_item}}. Please investigate and remove duplicate reporting.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t18",
    template_code: "T18",
    name: "Re-aged Debt Dispute",
    description: "Dispute incorrect newer delinquency or reporting dates on old debt.",
    legal_basis: "FCRA Section 605 and Section 611",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      creditor_name: "Creditor or collector",
      reported_date: "Reported date",
      delinquency_date: "Original delinquency date",
    },
    default_content:
      "I dispute {{creditor_name}} because it appears re-aged. Reported date: {{reported_date}}. Correct original delinquency date: {{delinquency_date}}.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t19",
    template_code: "T19",
    name: "Incorrect Balance Dispute",
    description: "Dispute incorrect balance, credit limit, high balance, or past due amount.",
    legal_basis: "FCRA Section 611",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      creditor_name: "Creditor or furnisher",
      incorrect_field: "Incorrect field",
      correct_value: "Correct value",
    },
    default_content:
      "The {{incorrect_field}} reported by {{creditor_name}} is inaccurate. The correct value is {{correct_value}}. Please update or delete the inaccurate reporting.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t20",
    template_code: "T20",
    name: "Incorrect Payment History Dispute",
    description: "Dispute incorrect late payment or payment status history.",
    legal_basis: "FCRA Section 611",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      creditor_name: "Creditor or furnisher",
      late_dates: "Reported late dates",
      dispute_reason: "Dispute reason",
    },
    default_content:
      "I dispute the payment history for {{creditor_name}}. The reported late date(s) {{late_dates}} are inaccurate because {{dispute_reason}}.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t21",
    template_code: "T21",
    name: "Closed Account Reporting Error",
    description: "Dispute an account that reports open or active incorrectly.",
    legal_basis: "FCRA Section 611",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      creditor_name: "Creditor or furnisher",
      reported_status: "Reported status",
      correct_status: "Correct status",
    },
    default_content:
      "{{creditor_name}} is reporting {{reported_status}}, but the correct status is {{correct_status}}. Please correct the account status.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t22",
    template_code: "T22",
    name: "Authorized User Removal Request",
    description: "Request removal of an authorized user tradeline.",
    legal_basis: "FCRA Section 611",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      creditor_name: "Creditor or furnisher",
      account_number: "Account number",
      removal_reason: "Removal reason",
    },
    default_content:
      "Please remove authorized user account {{account_number}} from {{creditor_name}} because {{removal_reason}}. I am not financially responsible for this account.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t23",
    template_code: "T23",
    name: "Obsolete Inquiry Removal",
    description: "Request removal of an old or obsolete inquiry.",
    legal_basis: "FCRA Section 605",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      company_name: "Company name",
      inquiry_date: "Inquiry date",
    },
    default_content:
      "Please remove the obsolete inquiry from {{company_name}} dated {{inquiry_date}} from my credit file.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t24",
    template_code: "T24",
    name: "Unauthorized Hard Inquiry Dispute",
    description: "Dispute a hard inquiry made without permissible purpose.",
    legal_basis: "FCRA Section 604 and Section 611",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      company_name: "Company name",
      inquiry_date: "Inquiry date",
    },
    default_content:
      "I did not authorize the hard inquiry from {{company_name}} dated {{inquiry_date}}. Please verify permissible purpose or remove the inquiry.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t25",
    template_code: "T25",
    name: "Medical Collection Removal Request",
    description: "Request removal or correction of a paid or insurance-resolved medical collection.",
    legal_basis: "FCRA and medical debt reporting rules",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      collector_name: "Collector",
      medical_provider: "Medical provider",
      dispute_reason: "Dispute reason",
    },
    default_content:
      "I dispute the medical collection from {{collector_name}} for {{medical_provider}} because {{dispute_reason}}. Please remove or correct it.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t26",
    template_code: "T26",
    name: "Furnisher Failure to Investigate Follow-Up",
    description: "Follow up when a furnisher does not reasonably investigate a direct dispute.",
    legal_basis: "FCRA Section 623",
    recipient_type: "furnisher",
    variable_schema: {
      consumer_name: "Consumer name",
      account_number: "Account number",
      dispute_date: "Original dispute date",
      issue_description: "Issue description",
    },
    default_content:
      "I submitted a direct dispute on {{dispute_date}} for account {{account_number}}. Please conduct a reasonable investigation into {{issue_description}} and correct your reporting.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t27",
    template_code: "T27",
    name: "Bureau Nonresponse Follow-Up",
    description: "Follow up when a bureau does not timely respond to a dispute.",
    legal_basis: "FCRA Section 611",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      dispute_date: "Dispute date",
      item_description: "Item description",
    },
    default_content:
      "I submitted a dispute on {{dispute_date}} for {{item_description}} and have not received timely results. Please provide results immediately.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t28",
    template_code: "T28",
    name: "Statement of Dispute Request",
    description: "Request a consumer dispute statement be added to the credit file.",
    legal_basis: "FCRA Section 611(b)",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      item_description: "Item description",
      consumer_statement: "Consumer statement",
    },
    default_content:
      "Please add this statement to my credit file for {{item_description}}: {{consumer_statement}}",
    created_at: iso(-60),
  },
  {
    id: "tpl-t29",
    template_code: "T29",
    name: "Complaint Package Cover Letter",
    description: "Cover letter for CFPB or state regulator complaint package.",
    legal_basis: "Consumer protection complaint escalation",
    recipient_type: "regulator",
    variable_schema: {
      consumer_name: "Consumer name",
      company_name: "Company name",
      issue_type: "Issue type",
      issue_summary: "Issue summary",
    },
    default_content:
      "Enclosed is my complaint package regarding {{company_name}}. Issue type: {{issue_type}}. Summary: {{issue_summary}}.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t30",
    template_code: "T30",
    name: "Settlement Confirmation Letter",
    description: "Confirm settlement terms before making payment.",
    legal_basis: "Settlement documentation",
    recipient_type: "collector",
    variable_schema: {
      consumer_name: "Consumer name",
      account_number: "Account number",
      settlement_amount: "Settlement amount",
      payment_date: "Payment date",
    },
    default_content:
      "Please confirm settlement of account {{account_number}} for {{settlement_amount}} due {{payment_date}} before I submit payment.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t31",
    template_code: "T31",
    name: "Paid in Full Zero Balance Update Request",
    description: "Request update to paid in full, settled, or zero balance status.",
    legal_basis: "FCRA Section 623",
    recipient_type: "furnisher",
    variable_schema: {
      consumer_name: "Consumer name",
      account_number: "Account number",
      correct_status: "Correct status",
    },
    default_content:
      "Please update account {{account_number}} to accurately report {{correct_status}}, including a zero balance if applicable.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t32",
    template_code: "T32",
    name: "Delete After Payment Confirmation",
    description: "Request deletion after a negotiated payment agreement.",
    legal_basis: "Settlement documentation",
    recipient_type: "collector",
    variable_schema: {
      consumer_name: "Consumer name",
      account_number: "Account number",
      payment_amount: "Payment amount",
      agreement_reference: "Agreement reference",
    },
    default_content:
      "Please confirm deletion of account {{account_number}} after payment of {{payment_amount}} according to agreement {{agreement_reference}}.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t33",
    template_code: "T33",
    name: "Identity Theft Affidavit Cover Letter",
    description: "Cover letter for submitting an identity theft report package.",
    legal_basis: "FCRA Section 605B",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      fraudulent_items: "Fraudulent items",
      report_number: "Identity theft report number",
    },
    default_content:
      "I am submitting an identity theft report package for {{fraudulent_items}}. Report number: {{report_number}}.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t34",
    template_code: "T34",
    name: "Fraudulent Account Block Request",
    description: "Request blocking of a specific fraudulent account.",
    legal_basis: "FCRA Section 605B",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      creditor_name: "Creditor or furnisher",
      account_number: "Account number",
      report_number: "Identity theft report number",
    },
    default_content:
      "Please block fraudulent account {{account_number}} from {{creditor_name}}. Identity theft report number: {{report_number}}.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t35",
    template_code: "T35",
    name: "Fraud Alert Security Freeze Instruction Letter",
    description: "Instructions for fraud alert or security freeze after identity theft.",
    legal_basis: "FCRA fraud alert and security freeze rights",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      requested_action: "Requested action",
      reason: "Reason",
      phone_number: "Phone number",
    },
    default_content:
      "Please complete the following fraud protection action: {{requested_action}}. Reason: {{reason}}. Contact phone: {{phone_number}}.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t36",
    template_code: "T36",
    name: "Bankruptcy Included in Bankruptcy Correction",
    description: "Correct tradeline status after bankruptcy discharge.",
    legal_basis: "FCRA Section 611",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      creditor_name: "Creditor or furnisher",
      bankruptcy_case: "Bankruptcy case number",
      correct_status: "Correct status",
    },
    default_content:
      "Please correct {{creditor_name}} to reflect bankruptcy case {{bankruptcy_case}} and status {{correct_status}}.",
    created_at: iso(-60),
  },
  {
    id: "tpl-t37",
    template_code: "T37",
    name: "Student Loan Reporting Error Dispute",
    description: "Dispute incorrect student loan payment, status, or servicer transfer reporting.",
    legal_basis: "FCRA Section 611 and Section 623",
    recipient_type: "bureau",
    variable_schema: {
      consumer_name: "Consumer name",
      servicer_name: "Servicer or furnisher",
      reported_error: "Reported error",
      correct_information: "Correct information",
    },
    default_content:
      "I dispute student loan reporting by {{servicer_name}}. Reported error: {{reported_error}}. Correct information: {{correct_information}}.",
    created_at: iso(-60),
  },
);

const disputeTimeline: DisputeTimelineEntry[] = [
  {
    id: "tl-case-created",
    case_id: "case-collection-portfolio",
    event_type: "case_created",
    description: "Case created and item flagged for validation.",
    created_at: iso(-18),
  },
  {
    id: "tl-letter-generated",
    case_id: "case-collection-portfolio",
    event_type: "letter_generated",
    description: "Debt validation letter generated.",
    created_at: iso(-16),
  },
  {
    id: "tl-letter-sent",
    case_id: "case-collection-portfolio",
    event_type: "letter_sent",
    description: "Letter sent by certified mail.",
    created_at: iso(-14),
  },
];

const disputeLetters: DisputeLetter[] = [
  {
    id: "letter-dv-001",
    case_id: "case-collection-portfolio",
    template_id: "tpl-dv",
    template_code: "FDCPA-DV",
    recipient_name: "Portfolio Recovery Associates",
    recipient_address: "120 Corporate Blvd, Norfolk, VA",
    content: "I dispute this debt and request validation before any further collection activity.",
    tracking_number: "9400 1000 0000 0000 0001",
    sent_at: iso(-14),
    created_at: iso(-16),
  },
];

const creditRepairAccounts: CreditAccount[] = [
  {
    id: "cra-chase",
    creditor_name: "Chase Freedom",
    account_type: "revolving",
    account_number_masked: "**** 4481",
    bureau: "Experian",
    opened_date: iso(-900),
    reported_date: iso(-8),
    credit_limit_cents: 850000,
    balance_cents: 112000,
    past_due_cents: 0,
    payment_status: "current",
    account_status: "open",
    remarks: "Pays as agreed",
    created_at: iso(-90),
  },
  {
    id: "cra-capital",
    creditor_name: "Capital One Platinum",
    account_type: "revolving",
    account_number_masked: "**** 1920",
    bureau: "TransUnion",
    opened_date: iso(-640),
    reported_date: iso(-9),
    credit_limit_cents: 300000,
    balance_cents: 210000,
    past_due_cents: 0,
    payment_status: "current",
    account_status: "open",
    remarks: "One disputed late payment",
    created_at: iso(-90),
  },
];

const creditInquiries: CreditInquiry[] = [
  {
    id: "inq-auto",
    bureau: "Experian",
    inquiry_date: iso(-42),
    creditor_name: "Auto Finance Co",
    inquiry_type: "hard",
    created_at: iso(-42),
  },
  {
    id: "inq-prequal",
    bureau: "Equifax",
    inquiry_date: iso(-12),
    creditor_name: "Card Prequalification",
    inquiry_type: "soft",
    created_at: iso(-12),
  },
];

const derogatoryItems: CreditDerogatory[] = [
  {
    id: "derog-portfolio",
    bureau: "Experian",
    derogatory_type: "collection",
    creditor_name: "Portfolio Recovery Associates",
    account_number_masked: "**** 7712",
    date_reported: iso(-220),
    date_expected_removal: iso(365 * 6),
    description: "Collection account disputed as not mine.",
    created_at: iso(-90),
  },
  {
    id: "derog-capital-late",
    bureau: "TransUnion",
    derogatory_type: "late_payment",
    creditor_name: "Capital One",
    account_number_masked: "**** 1920",
    date_reported: iso(-95),
    date_expected_removal: iso(365 * 7),
    description: "30-day late disputed due to payment confirmation.",
    created_at: iso(-80),
  },
];

const businessCreditProfiles: BusinessCreditProfile[] = [
  {
    id: "biz-phoenix",
    business_name: "Phoenix Holdings LLC",
    ein: "**-***4921",
    industry: "Consulting",
    business_credit_score: 78,
    paydex_score: 80,
    credit_limit_cents: 2500000,
    data_provider: "Manual",
    created_at: iso(-40),
    updated_at: iso(-2),
  },
];

const investments: Investment[] = [
  {
    id: "investment-vti",
    account_id: "demo-brokerage",
    asset_type: "stock",
    symbol: "VTI",
    name: "Vanguard Total Stock Market ETF",
    quantity: "110",
    cost_basis_cents: 2200000,
    current_value_cents: 2850000,
    currency: "USD",
    created_at: iso(-300),
    updated_at: iso(),
  },
];

const contacts: Contact[] = [
  {
    id: "contact-advisor",
    name: "Jordan Lee",
    email: "jordan@example.com",
    phone: "555-0105",
    organization: "Financial Planning Co",
    notes: "Annual planning review in August.",
    created_at: iso(-20),
    updated_at: iso(),
  },
];

function netWorth() {
  const totalAssets = accounts
    .filter((a) => !["credit", "loan"].includes(a.account_type))
    .reduce((sum, a) => sum + a.balance_cents, 0);
  const totalLiabilities = accounts
    .filter((a) => ["credit", "loan"].includes(a.account_type))
    .reduce((sum, a) => sum + Math.abs(a.balance_cents), 0);
  return {
    total_assets_cents: totalAssets,
    total_liabilities_cents: totalLiabilities,
    net_worth_cents: totalAssets - totalLiabilities,
    currency: "USD",
  };
}

function cashFlow(): CashFlow {
  const income = transactions
    .filter((t) => t.transaction_type === "income")
    .reduce((sum, t) => sum + t.amount_cents, 0);
  const expenses = Math.abs(
    transactions
      .filter((t) => t.transaction_type === "expense")
      .reduce((sum, t) => sum + t.amount_cents, 0),
  );
  return { income, expenses, net: income - expenses };
}

function spendingByCategory(): SpendingByCategory[] {
  return categories
    .filter((c) => c.type === "expense")
    .map((category) => {
      const matches = transactions.filter((t) => t.category_id === category.id);
      return {
        category_id: category.id,
        category_name: category.name,
        total_cents: Math.abs(matches.reduce((sum, t) => sum + t.amount_cents, 0)),
        transaction_count: matches.length,
      };
    })
    .filter((item) => item.transaction_count > 0);
}

function debtSummary(): DebtSummary {
  const total_balance = debts.reduce((sum, debt) => sum + debt.remaining_balance_cents, 0);
  const total_min_payments = debts.reduce((sum, debt) => sum + debt.minimum_payment_cents, 0);
  const avg_apr =
    debts.reduce((sum, debt) => sum + Number.parseFloat(debt.interest_rate), 0) / debts.length;
  return {
    total_balance,
    total_min_payments,
    debt_count: debts.length,
    avg_apr,
  };
}

function budgetProgress(): BudgetProgress {
  const budget = { ...budgets[0], items: budgetItems } satisfies BudgetWithItems;
  const total_spent_cents = budgetItems.reduce((sum, item) => sum + item.spent_cents, 0);
  return {
    budget,
    total_spent_cents,
    percent_used: Math.round((total_spent_cents / budget.total_limit_cents) * 100),
    is_over_budget: total_spent_cents > budget.total_limit_cents,
  };
}

function disputeCaseDetail(caseId: string): DisputeCaseDetail {
  const disputeCase = disputeCases.find((c) => c.id === caseId) ?? disputeCases[0];
  return {
    case: disputeCase,
    letters: disputeLetters.filter((letter) => letter.case_id === disputeCase.id),
    timeline: disputeTimeline.filter((entry) => entry.case_id === disputeCase.id),
    negative_item: null,
  };
}

export function getDemoResponse<T>(cmd: string, args?: Record<string, unknown>): T | undefined {
  const id = typeof args?.id === "string" ? args.id : undefined;
  const query = typeof args?.query === "string" ? args.query.toLowerCase() : "";

  const responses: Record<string, unknown> = {
    list_accounts: accounts,
    get_account: accounts.find((a) => a.id === id) ?? accounts[0],
    list_transactions: transactions,
    get_transaction: transactions.find((t) => t.id === id) ?? transactions[0],
    search_transactions: transactions.filter((t) =>
      [t.description, t.merchant].some((value) => value?.toLowerCase().includes(query)),
    ),
    list_transactions_by_date_range: transactions,
    list_categories: categories,
    get_net_worth: netWorth(),
    get_cash_flow: cashFlow(),
    get_spending_by_category: spendingByCategory(),
    list_budgets: budgets,
    get_budget: budgets.find((b) => b.id === id) ?? budgets[0],
    get_budget_with_items: { ...budgets[0], items: budgetItems },
    get_budget_progress: budgetProgress(),
    list_budget_items: budgetItems,
    list_debts: debts,
    get_debt: debts.find((d) => d.id === id) ?? debts[0],
    get_debt_summary: debtSummary(),
    list_debts_by_type: debts,
    get_payoff_schedule: [],
    list_goals: goals,
    list_active_goals: goals,
    get_goal: goals.find((g) => g.id === id) ?? goals[0],
    list_goals_by_type: goals,
    list_goals_by_account: goals,
    list_credit_reports: creditReports,
    get_latest_credit_report: creditReports[0],
    get_credit_score_history: creditReports.map((report) => ({
      date: report.report_date,
      report_date: report.report_date,
      score: report.score,
      bureau: report.bureau,
    })),
    list_credit_accounts: creditRepairAccounts,
    get_credit_accounts: creditRepairAccounts,
    list_credit_inquiries: creditInquiries,
    list_derogatory_items: derogatoryItems,
    list_dispute_cases: disputeCases,
    get_dispute_case: disputeCaseDetail(id ?? disputeCases[0].id),
    list_dispute_letters: disputeLetters.filter((letter) => letter.case_id === args?.caseId),
    list_letter_templates: disputeTemplates,
    get_letter_template_by_code:
      disputeTemplates.find((template) => template.template_code === args?.code) ?? null,
    get_business_credit_profiles: businessCreditProfiles,
    list_investments: investments,
    get_investment: investments[0],
    list_investments_by_type: investments,
    list_contacts: contacts,
    search_contacts: contacts,
    list_contact_interactions: [],
    get_settings: {
      theme: "dark",
      ai_provider: "local",
      currency: "USD",
      locale: "en-US",
    },
    list_vault_items: [],
    search_vault_items: [],
    get_vault_access_log: [],
    list_recent_vault_access: [],
    list_documents: [],
    search_documents: [],
    list_reports: [],
    get_income_vs_expenses: {
      income_cents: cashFlow().income,
      expense_cents: cashFlow().expenses,
      net_cents: cashFlow().net,
    },
    get_monthly_report: [],
    get_spending_trend: [],
    list_maintenance_tasks: [],
    list_businesses: [],
    list_business_transactions: [],
  };

  if (cmd.startsWith("create_") || cmd.startsWith("update_") || cmd.startsWith("mark_")) {
    return {
      id: id ?? `demo-${Date.now()}`,
      ...(args?.payload as object | undefined),
      created_at: iso(),
      updated_at: iso(),
    } as T;
  }

  if (cmd.startsWith("delete_") || cmd.startsWith("deactivate_")) {
    return undefined as T;
  }

  return responses[cmd] as T | undefined;
}
