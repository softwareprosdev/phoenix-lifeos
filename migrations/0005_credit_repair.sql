-- Credit Repair & Legal Dispute Engine
-- Tables for dispute cases, negative items, letter templates, credit accounts,
-- inquiries, derogatory items, business credit profiles, and vendor accounts.

CREATE TABLE IF NOT EXISTS credit_accounts (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    issuer TEXT NOT NULL,
    account_type TEXT NOT NULL DEFAULT 'revolving',
    credit_limit_cents INTEGER NOT NULL DEFAULT 0,
    balance_cents INTEGER NOT NULL DEFAULT 0,
    apr TEXT NOT NULL DEFAULT '0',
    payment_status TEXT NOT NULL DEFAULT 'current',
    opened_date TEXT,
    closed_date TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS credit_inquiries (
    id TEXT PRIMARY KEY,
    bureau TEXT NOT NULL,
    creditor_name TEXT NOT NULL,
    inquiry_type TEXT NOT NULL DEFAULT 'hard',
    inquiry_date TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS credit_derogatory (
    id TEXT PRIMARY KEY,
    creditor_name TEXT NOT NULL,
    account_number_encrypted TEXT,
    item_type TEXT NOT NULL,
    amount_cents INTEGER NOT NULL DEFAULT 0,
    original_amount_cents INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'open',
    opened_date TEXT,
    expected_removal_date TEXT,
    is_disputed INTEGER NOT NULL DEFAULT 0,
    bureau TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS dispute_letter_templates (
    id TEXT PRIMARY KEY,
    template_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    legal_basis TEXT,
    recipient_type TEXT NOT NULL DEFAULT 'bureau',
    body_template TEXT NOT NULL,
    variable_schema TEXT NOT NULL DEFAULT '[]',
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS negative_items (
    id TEXT PRIMARY KEY,
    creditor_name TEXT NOT NULL,
    item_type TEXT NOT NULL,
    bureau TEXT NOT NULL,
    account_number_encrypted TEXT,
    amount_cents INTEGER NOT NULL DEFAULT 0,
    expected_removal_date TEXT,
    is_disputed INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS dispute_cases (
    id TEXT PRIMARY KEY,
    negative_item_id TEXT REFERENCES negative_items(id),
    dispute_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    status_basis TEXT,
    bureau TEXT NOT NULL,
    statute_basis TEXT,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS dispute_letters (
    id TEXT PRIMARY KEY,
    dispute_case_id TEXT NOT NULL REFERENCES dispute_cases(id),
    template_id TEXT REFERENCES dispute_letter_templates(id),
    recipient_type TEXT NOT NULL,
    recipient_name TEXT NOT NULL,
    recipient_address TEXT,
    generated_content_encrypted TEXT,
    tracking_number TEXT,
    sent_date TEXT,
    response_received_date TEXT,
    response_summary TEXT,
    status TEXT NOT NULL DEFAULT 'draft',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS dispute_timeline (
    id TEXT PRIMARY KEY,
    dispute_case_id TEXT NOT NULL REFERENCES dispute_cases(id),
    event_type TEXT NOT NULL,
    description TEXT NOT NULL,
    metadata TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS business_credit_profiles (
    id TEXT PRIMARY KEY,
    business_name TEXT NOT NULL,
    ein_encrypted TEXT,
    duns_number TEXT,
    paydex_score INTEGER,
    business_credit_score INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS vendor_credit_accounts (
    id TEXT PRIMARY KEY,
    business_credit_profile_id TEXT NOT NULL REFERENCES business_credit_profiles(id),
    vendor_name TEXT NOT NULL,
    account_type TEXT NOT NULL DEFAULT 'net30',
    credit_limit_cents INTEGER NOT NULL DEFAULT 0,
    payment_status TEXT NOT NULL DEFAULT 'current',
    reports_to_dnb INTEGER NOT NULL DEFAULT 0,
    reports_to_experian INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Seed dispute letter templates (T01-T12)
INSERT INTO dispute_letter_templates (id, template_code, name, description, legal_basis, recipient_type, body_template, variable_schema)
VALUES
('t01', 'T01', 'Section 609 Credit File Request',
 'Request complete credit file disclosure under FCRA Section 609',
 'FCRA Section 609 (15 U.S.C. § 1681g)',
 'bureau',
 'To Whom It May Concern,\n\nI am requesting a complete and accurate disclosure of my credit file under Section 609 of the Fair Credit Reporting Act (15 U.S.C. § 1681g). This request includes:\n\n1. All information in my file at the time of request\n2. The source of any information reported\n3. Any recipients of my credit report\n4. A list of all inquiries made within the past 2 years\n\nPlease provide the complete file disclosure within 15 days as required by law. Enclosed is a copy of my identification.\n\nSincerely,\n{{consumer_name}}',
 '["consumer_name", "consumer_address", "phone_number"]'),

('t02', 'T02', 'Section 611 Inaccuracy Dispute',
 'Dispute inaccurate information on credit report under FCRA Section 611',
 'FCRA Section 611 (15 U.S.C. § 1681i)',
 'bureau',
 'To Whom It May Concern,\n\nI am disputing the following item(s) on my credit report under Section 611 of the Fair Credit Reporting Act (15 U.S.C. § 1681i). I request that you reinvestigate the following item(s) at no charge:\n\nDisputed Item: {{item_description}}\nAccount Number: {{account_number}}\nCreditor: {{creditor_name}}\nReason: {{dispute_reason}}\n\nI have enclosed copies of supporting documentation. Please complete your investigation within 30 days as required by law and provide me with the results and a free copy of my updated report.\n\nSincerely,\n{{consumer_name}}',
 '["consumer_name", "consumer_address", "item_description", "account_number", "creditor_name", "dispute_reason"]'),

('t03', 'T03', 'Section 623 Furnisher Direct Dispute',
 'Direct dispute to data furnisher under FCRA Section 623',
 'FCRA Section 623 (15 U.S.C. § 1681s-2)',
 'furnisher',
 'To Whom It May Concern,\n\nI am directly disputing the accuracy of information your company has furnished to the credit bureaus regarding the following account:\n\nAccount Number: {{account_number}}\nDisputed Item: {{item_description}}\nReason for Dispute: {{dispute_reason}}\n\nUnder Section 623 of the Fair Credit Reporting Act (15 U.S.C. § 1681s-2), you must:\n1. Conduct a reasonable investigation\n2. Review all relevant information provided\n3. Report the results to the bureaus\n\nPlease complete your investigation within 30 days.\n\nSincerely,\n{{consumer_name}}',
 '["consumer_name", "consumer_address", "account_number", "item_description", "dispute_reason"]'),

('t04', 'T04', 'Debt Validation Request',
 'Request debt validation under FDCPA Section 809',
 'FDCPA Section 809 (15 U.S.C. § 1692g)',
 'collector',
 'To Whom It May Concern,\n\nI am writing in response to your debt collection communication regarding account {{account_number}} in the amount of {{amount}}.\n\nPursuant to FDCPA Section 809 (15 U.S.C. § 1692g), I request validation of this debt, including:\n1. Proof that I am the correct debtor\n2. Copy of the original contract or agreement\n3. Itemized statement showing all charges, fees, and interest\n4. Proof that you are licensed to collect in my state\n5. The statute of limitations for this debt\n\nPlease cease all collection activities until you provide the requested validation.\n\nSincerely,\n{{consumer_name}}',
 '["consumer_name", "consumer_address", "account_number", "amount"]'),

('t05', 'T05', 'Identity Theft Block Request',
 'Request blocking of fraudulent accounts under FCRA Section 605B',
 'FCRA Section 605B (15 U.S.C. § 1681c-2)',
 'bureau',
 'To Whom It May Concern,\n\nI am a victim of identity theft. Under Section 605B of the Fair Credit Reporting Act (15 U.S.C. § 1681c-2), I request that you block the following fraudulent item(s) from my credit report:\n\nItem: {{item_description}}\nAccount Number: {{account_number}}\nCreditor: {{creditor_name}}\n\nI have enclosed:\n1. A copy of my Identity Theft Report (FTC Form 14027B)\n2. Proof of my identity\n3. A description of the fraudulent items\n\nPlease process this block within 4 business days as required by law.\n\nSincerely,\n{{consumer_name}}',
 '["consumer_name", "consumer_address", "item_description", "account_number", "creditor_name"]'),

('t06', 'T06', '7-Year Obsolescence Removal',
 'Request removal of obsolete negative items under FCRA Section 605',
 'FCRA Section 605 (15 U.S.C. § 1681c)',
 'bureau',
 'To Whom It May Concern,\n\nI am requesting removal of the following obsolete item(s) from my credit report under Section 605 of the Fair Credit Reporting Act (15 U.S.C. § 1681c):\n\nNegative Item: {{item_description}}\nCreditor: {{creditor_name}}\nOriginal Date of Delinquency: {{delinquency_date}}\n\nUnder FCRA Section 605(a), most negative information cannot be reported for more than 7 years from the date of the original delinquency. Since this item is beyond the 7-year reporting period, please remove it immediately.\n\nSincerely,\n{{consumer_name}}',
 '["consumer_name", "consumer_address", "item_description", "creditor_name", "delinquency_date"]'),

('t07', 'T07', 'Goodwill Late Payment Removal',
 'Request goodwill deletion of late payment as a courtesy',
 'None (goodwill request)',
 'furnisher',
 'To Whom It May Concern,\n\nI am writing to request a goodwill adjustment to my account {{account_number}}.\n\nI have been a customer since {{account_age}} and have generally maintained my account in good standing. However, I had a late payment on {{late_date}} due to {{circumstances}}.\n\nI have since corrected the issue and all payments have been on time for the past {{on_time_months}} months. I respectfully request that you remove this late payment from my credit report as a courtesy.\n\nThank you for your consideration.\n\nSincerely,\n{{consumer_name}}',
 '["consumer_name", "consumer_address", "account_number", "account_age", "late_date", "circumstances", "on_time_months"]'),

('t08', 'T08', 'Pay-for-Delete Agreement Request',
 'Request pay-for-delete settlement agreement',
 'None (negotiated agreement)',
 'collector',
 'To Whom It May Concern,\n\nI am writing regarding the collection account {{account_number}} in the amount of {{amount}}.\n\nI am willing to settle this debt in exchange for your agreement to remove the collection account from my credit reports at all three bureaus upon payment.\n\nSpecifically, I propose:\n1. Settlement Amount: {{settlement_amount}}\n2. Payment Date: {{payment_date}}\n3. In exchange, you agree to delete or mark the account as "paid in full" and cease reporting\n\nPlease provide written confirmation of these terms before I submit payment.\n\nSincerely,\n{{consumer_name}}',
 '["consumer_name", "consumer_address", "account_number", "amount", "settlement_amount", "payment_date"]'),

('t09', 'T09', 'Cease and Desist Request',
 'Request cessation of communication under FDCPA Section 805(c)',
 'FDCPA Section 805(c) (15 U.S.C. § 1692c(c))',
 'collector',
 'To Whom It May Concern,\n\nPursuant to FDCPA Section 805(c) (15 U.S.C. § 1692c(c)), this letter serves as notice that I demand you cease all communication with me regarding the alleged debt referenced as {{account_number}}.\n\nFurther communication may be directed to my attorney or in writing to advise me that:\n1. Collection efforts are being terminated\n2. You are invoking specific remedies permitted by law\n\nPlease note that any further communication in violation of FDCPA Section 805(c) may result in legal action.\n\nSincerely,\n{{consumer_name}}',
 '["consumer_name", "consumer_address", "account_number"]'),

('t10', 'T10', 'CFPB Complaint Escalation',
 'Escalate unresolved dispute to CFPB',
 'CFPB Regulatory Authority (12 U.S.C. § 5491)',
 'regulator',
 'To the Consumer Financial Protection Bureau,\n\nI am filing this complaint regarding a credit reporting issue that remains unresolved after direct disputes with the credit bureaus and data furnisher.\n\nCompany Name: {{company_name}}\nIssue Type: {{issue_type}}\nDispute Reference: {{dispute_reference}}\nDescription: {{issue_description}}\n\nDespite my attempts to resolve this matter directly, the issue persists. I am requesting CFPB assistance in resolving this matter.\n\nThank you.\n\nSincerely,\n{{consumer_name}}',
 '["consumer_name", "consumer_address", "company_name", "issue_type", "dispute_reference", "issue_description"]'),

('t11', 'T11', 'Business Credit Dispute',
 'Dispute business credit report errors under FCRA Section 623',
 'FCRA Section 623 (15 U.S.C. § 1681s-2)',
 'bureau',
 'To Whom It May Concern,\n\nI am disputing inaccurate information on my business credit profile for {{business_name}} (EIN: {{ein}} / DUNS: {{duns}}).\n\nDisputed Item: {{item_description}}\nAccount/Reference: {{account_reference}}\nReason: {{dispute_reason}}\n\nPlease reinvestigate this item under FCRA Section 623 and correct or remove the inaccurate information within 30 days.\n\nSincerely,\n{{consumer_name}}',
 '["consumer_name", "business_name", "ein", "duns", "item_description", "account_reference", "dispute_reason"]'),

('t12', 'T12', 'Experian Business File Request',
 'Request Experian business credit file disclosure',
 'FCRA Section 609 (15 U.S.C. § 1681g)',
 'bureau',
 'To Experian Business Credit Services,\n\nI am requesting a complete disclosure of my business credit file for {{business_name}} (EIN: {{ein}}).\n\nUnder FCRA Section 609 (15 U.S.C. § 1681g), please provide:\n1. Complete business credit report\n2. Source of all information\n3. List of all inquiries\n4. Any disputes on file\n\nEnclosed is documentation of my business identity.\n\nSincerely,\n{{consumer_name}}',
 '["consumer_name", "business_name", "ein"]');
