use super::business_partner::BusinessPartner;
use super::{COGS_ACCOUNT, PAYABLE_ACCOUNT, Customer, Material, INVENTORY_ACCOUNT, INVENTORY_DIFF_ACCOUNT, User, Vendor, SALES_ACCOUNT, RECEIVABLE_ACCOUNT};
use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebtCredit {
    Debt,
    Credit,
}

impl DebtCredit {
    pub fn to_db_option(&self) -> &str {
        match self {
            DebtCredit::Debt => "D",
            DebtCredit::Credit => "C",
        }
    }
}

pub struct Document {
    pub posting_date: NaiveDate,
    pub doc_no: String,
    pub updated_by: User,
    pub lines: Vec<DocumentLine>,
}

impl Document {
    pub fn new_purchase_document(
        posting_date: NaiveDate,
        doc_no: String,
        material: Material,
        vendor: Vendor,
        price: Decimal,
        std_price: Decimal,
        quantity: Decimal,
        updated_by: User,
    ) -> Document {
        let amount = price * quantity;
        let std_amount = std_price * quantity;
        let diff_amount = amount - std_amount;

        let mut lines = vec![DocumentLine {
            account: INVENTORY_ACCOUNT,
            material: Some(material),
            business_partner: None,
            debt_credit: DebtCredit::Debt,
            amount: std_amount,
            quantity: Some(quantity),
        }];

        if diff_amount != Decimal::from(0) {
            let debt_credit = if diff_amount > Decimal::from(0) {
                DebtCredit::Debt
            } else {
                DebtCredit::Credit
            };
            let diff_line = DocumentLine {
                account: INVENTORY_DIFF_ACCOUNT,
                material: Some(material),
                business_partner: None,
                debt_credit,
                amount: diff_amount,
                quantity: None,
            };
            lines.push(diff_line);
        }

        lines.push(DocumentLine {
            account: PAYABLE_ACCOUNT,
            material: None,
            business_partner: Some(BusinessPartner::Vendor(vendor)),
            debt_credit: DebtCredit::Credit,
            amount: -amount,
            quantity: None,
        });

        Document {
            posting_date,
            doc_no,
            updated_by,
            lines,
        }
    }

    pub fn new_cost_document(
        posting_date: NaiveDate,
        doc_no: String,
        material: Material,
        vendor: Vendor,
        amount: Decimal,
        updated_by: User,
    ) -> Document {
        let lines = vec![
            DocumentLine {
                account: INVENTORY_DIFF_ACCOUNT,
                material: Some(material),
                business_partner: None,
                debt_credit: DebtCredit::Debt,
                amount,
                quantity: None,
            },
            DocumentLine {
                account: PAYABLE_ACCOUNT,
                material: None,
                business_partner: Some(BusinessPartner::Vendor(vendor)),
                debt_credit: DebtCredit::Credit,
                amount: -amount,
                quantity: None,
            },
        ];

        Document {
            posting_date,
            doc_no,
            updated_by,
            lines,
        }
    }

    pub fn new_cogs_document(
        posting_date: NaiveDate,
        doc_no: String,
        customer: Customer,
        material: Material,
        std_price: Decimal,
        quantity: Decimal,
        updated_by: User,
    ) -> Document {
        let std_amount = std_price * quantity;

        let lines = vec![
            DocumentLine {
                account: COGS_ACCOUNT,
                material: Some(material),
                business_partner: Some(BusinessPartner::Customer(customer)),
                debt_credit: DebtCredit::Debt,
                amount: std_amount,
                quantity: Some(quantity),
            },
            DocumentLine {
                account: INVENTORY_ACCOUNT,
                material: Some(material),
                business_partner: None,
                debt_credit: DebtCredit::Credit,
                amount: -std_amount,
                quantity: Some(quantity),
            },
        ];

        Document {
            posting_date,
            doc_no,
            updated_by,
            lines,
        }
    }
    pub fn new_sale_document(
        posting_date: NaiveDate,
        doc_no: String,
        customer: Customer,
        material: Material,
        price: Decimal,
        quantity: Decimal,
        updated_by: User,
    ) -> Document {
        let amount = price * quantity;

        let lines = vec![
            DocumentLine {
                account: RECEIVABLE_ACCOUNT,
                material: None,
                business_partner: Some(BusinessPartner::Customer(customer)),
                debt_credit: DebtCredit::Debt,
                amount,
                quantity: None,
            },
            DocumentLine {
                account: SALES_ACCOUNT,
                material: Some(material),
                business_partner: Some(BusinessPartner::Customer(customer)),
                debt_credit: DebtCredit::Credit,
                amount: -amount,
                quantity: Some(quantity),
            },
        ];

        Document {
            posting_date,
            doc_no,
            updated_by,
            lines,
        }
    }
}

pub struct DocumentLine {
    pub account: &'static str,
    pub business_partner: Option<BusinessPartner>,
    pub material: Option<Material>,
    pub debt_credit: DebtCredit,
    pub amount: Decimal,
    pub quantity: Option<Decimal>,
}
