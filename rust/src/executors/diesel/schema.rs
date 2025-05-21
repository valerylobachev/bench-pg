// @generated automatically by Diesel CLI.

diesel::table! {
    bm_benchmarks (id) {
        id -> Int8,
        #[max_length = 200]
        name -> Varchar,
        date -> Timestamptz,
        #[max_length = 40]
        db_lib -> Varchar,
        customers -> Int4,
        vendors -> Int4,
        materials -> Int4,
        users -> Int4,
        start_year -> Int4,
        years -> Int4,
        operations -> Int4,
    }
}

diesel::table! {
    bm_metrics (id) {
        id -> Int8,
        benchmark_id -> Int8,
        year -> Int4,
        period -> Nullable<Int4>,
        index -> Int8,
        user_no -> Int4,
        #[max_length = 20]
        action -> Varchar,
        #[max_length = 20]
        material_id -> Nullable<Varchar>,
        #[max_length = 20]
        business_partner_id -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        price -> Nullable<Numeric>,
        amount -> Nullable<Numeric>,
        #[max_length = 40]
        doc_no -> Nullable<Varchar>,
        #[max_length = 40]
        sale_doc_no -> Nullable<Varchar>,
        #[max_length = 40]
        cogs_doc_no -> Nullable<Varchar>,
        posting_date -> Nullable<Date>,
        duration -> Float8,
    }
}

diesel::table! {
    bm_statistics (id) {
        id -> Int8,
        benchmark_id -> Int8,
        #[max_length = 20]
        action -> Varchar,
        year -> Nullable<Int4>,
        month -> Nullable<Int4>,
        total_count -> Int8,
        total_duration -> Float8,
        ops_per_sec -> Float8,
        min -> Float8,
        p50 -> Float8,
        p75 -> Float8,
        p95 -> Float8,
        p99 -> Float8,
        p99_9 -> Float8,
        max -> Float8,
        mean -> Float8,
        std_dev -> Float8,
    }
}

diesel::table! {
    fin_accounts (id) {
        #[max_length = 20]
        id -> Varchar,
        #[max_length = 200]
        name -> Varchar,
        #[max_length = 20]
        updated_by -> Varchar,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    fin_business_partners (id) {
        #[max_length = 20]
        id -> Varchar,
        #[max_length = 200]
        name -> Varchar,
        #[max_length = 20]
        updated_by -> Varchar,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    fin_ledger_items (id) {
        id -> Int8,
        period -> Int4,
        #[max_length = 40]
        doc_no -> Varchar,
        posting_date -> Date,
        #[max_length = 20]
        account_id -> Varchar,
        #[max_length = 20]
        material_id -> Nullable<Varchar>,
        #[max_length = 20]
        business_partner_id -> Nullable<Varchar>,
        #[max_length = 1]
        debt_credit -> Varchar,
        amount -> Numeric,
        debt -> Numeric,
        credit -> Numeric,
        quantity -> Numeric,
        #[max_length = 20]
        updated_by -> Varchar,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    fin_material_periods (material_id, period) {
        #[max_length = 20]
        material_id -> Varchar,
        period -> Int4,
        std_price -> Numeric,
        mov_avg_price -> Numeric,
        actual_price -> Numeric,
        sell_price -> Numeric,
        stock -> Numeric,
        #[max_length = 20]
        updated_by -> Varchar,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    fin_materials (id) {
        #[max_length = 20]
        id -> Varchar,
        #[max_length = 200]
        name -> Varchar,
        next_std_price -> Numeric,
        #[max_length = 20]
        updated_by -> Varchar,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(bm_metrics -> bm_benchmarks (benchmark_id));
diesel::joinable!(fin_ledger_items -> fin_accounts (account_id));
diesel::joinable!(fin_ledger_items -> fin_business_partners (business_partner_id));
diesel::joinable!(fin_ledger_items -> fin_materials (material_id));
diesel::joinable!(fin_material_periods -> fin_materials (material_id));

diesel::allow_tables_to_appear_in_same_query!(
    bm_benchmarks,
    bm_metrics,
    bm_statistics,
    fin_accounts,
    fin_business_partners,
    fin_ledger_items,
    fin_material_periods,
    fin_materials,
);
