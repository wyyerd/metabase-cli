table! {
    activity (id) {
        id -> Int4,
        topic -> Varchar,
        timestamp -> Timestamptz,
        user_id -> Nullable<Int4>,
        model -> Nullable<Varchar>,
        model_id -> Nullable<Int4>,
        database_id -> Nullable<Int4>,
        table_id -> Nullable<Int4>,
        custom_id -> Nullable<Varchar>,
        details -> Varchar,
    }
}

table! {
    card_label (id) {
        id -> Int4,
        card_id -> Int4,
        label_id -> Int4,
    }
}

table! {
    collection (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        color -> Bpchar,
        archived -> Bool,
        location -> Varchar,
        personal_owner_id -> Nullable<Int4>,
        slug -> Varchar,
    }
}

table! {
    collection_revision (id) {
        id -> Int4,
        before -> Text,
        after -> Text,
        user_id -> Int4,
        created_at -> Timestamp,
        remark -> Nullable<Text>,
    }
}

table! {
    computation_job (id) {
        id -> Int4,
        creator_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[sql_name = "type"]
        type_ -> Varchar,
        status -> Varchar,
        context -> Nullable<Text>,
        ended_at -> Nullable<Timestamp>,
    }
}

table! {
    computation_job_result (id) {
        id -> Int4,
        job_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        permanence -> Varchar,
        payload -> Text,
    }
}

table! {
    core_session (id) {
        id -> Varchar,
        user_id -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    core_user (id) {
        id -> Int4,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        password -> Varchar,
        password_salt -> Varchar,
        date_joined -> Timestamptz,
        last_login -> Nullable<Timestamptz>,
        is_superuser -> Bool,
        is_active -> Bool,
        reset_token -> Nullable<Varchar>,
        reset_triggered -> Nullable<Int8>,
        is_qbnewb -> Bool,
        google_auth -> Bool,
        ldap_auth -> Bool,
        login_attributes -> Nullable<Text>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    dashboardcard_series (id) {
        id -> Int4,
        dashboardcard_id -> Int4,
        card_id -> Int4,
        position -> Int4,
    }
}

table! {
    dashboard_favorite (id) {
        id -> Int4,
        user_id -> Int4,
        dashboard_id -> Int4,
    }
}

table! {
    data_migrations (id) {
        id -> Varchar,
        timestamp -> Timestamp,
    }
}

table! {
    dependency (id) {
        id -> Int4,
        model -> Varchar,
        model_id -> Int4,
        dependent_on_model -> Varchar,
        dependent_on_id -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    dimension (id) {
        id -> Int4,
        field_id -> Int4,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        human_readable_field_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    label (id) {
        id -> Int4,
        name -> Varchar,
        slug -> Varchar,
        icon -> Nullable<Varchar>,
    }
}

table! {
    metabase_database (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
        description -> Nullable<Text>,
        details -> Nullable<Text>,
        engine -> Varchar,
        is_sample -> Bool,
        is_full_sync -> Bool,
        points_of_interest -> Nullable<Text>,
        caveats -> Nullable<Text>,
        metadata_sync_schedule -> Varchar,
        cache_field_values_schedule -> Varchar,
        timezone -> Nullable<Varchar>,
        is_on_demand -> Bool,
        options -> Nullable<Text>,
    }
}

table! {
    metabase_field (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
        base_type -> Varchar,
        special_type -> Nullable<Varchar>,
        active -> Bool,
        description -> Nullable<Text>,
        preview_display -> Bool,
        position -> Int4,
        table_id -> Int4,
        parent_id -> Nullable<Int4>,
        display_name -> Nullable<Varchar>,
        visibility_type -> Varchar,
        fk_target_field_id -> Nullable<Int4>,
        last_analyzed -> Nullable<Timestamptz>,
        points_of_interest -> Nullable<Text>,
        caveats -> Nullable<Text>,
        fingerprint -> Nullable<Text>,
        fingerprint_version -> Int4,
        database_type -> Varchar,
        has_field_values -> Nullable<Text>,
        settings -> Nullable<Text>,
    }
}

table! {
    metabase_fieldvalues (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        values -> Nullable<Text>,
        human_readable_values -> Nullable<Text>,
        field_id -> Int4,
    }
}

table! {
    metabase_table (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
        rows -> Nullable<Int8>,
        description -> Nullable<Text>,
        entity_name -> Nullable<Varchar>,
        entity_type -> Nullable<Varchar>,
        active -> Bool,
        db_id -> Int4,
        display_name -> Nullable<Varchar>,
        visibility_type -> Nullable<Varchar>,
        schema -> Nullable<Varchar>,
        points_of_interest -> Nullable<Text>,
        caveats -> Nullable<Text>,
        show_in_getting_started -> Bool,
        fields_hash -> Nullable<Text>,
    }
}

table! {
    metric (id) {
        id -> Int4,
        table_id -> Int4,
        creator_id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        archived -> Bool,
        definition -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        points_of_interest -> Nullable<Text>,
        caveats -> Nullable<Text>,
        how_is_this_calculated -> Nullable<Text>,
        show_in_getting_started -> Bool,
    }
}

table! {
    metric_important_field (id) {
        id -> Int4,
        metric_id -> Int4,
        field_id -> Int4,
    }
}

table! {
    permissions (id) {
        id -> Int4,
        object -> Varchar,
        group_id -> Int4,
    }
}

table! {
    permissions_group (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    permissions_group_membership (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
    }
}

table! {
    permissions_revision (id) {
        id -> Int4,
        before -> Text,
        after -> Text,
        user_id -> Int4,
        created_at -> Timestamp,
        remark -> Nullable<Text>,
    }
}

table! {
    pulse (id) {
        id -> Int4,
        creator_id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        skip_if_empty -> Bool,
        alert_condition -> Nullable<Varchar>,
        alert_first_only -> Nullable<Bool>,
        alert_above_goal -> Nullable<Bool>,
        collection_id -> Nullable<Int4>,
        collection_position -> Nullable<Int2>,
        archived -> Nullable<Bool>,
    }
}

table! {
    pulse_card (id) {
        id -> Int4,
        pulse_id -> Int4,
        card_id -> Int4,
        position -> Int4,
        include_csv -> Bool,
        include_xls -> Bool,
    }
}

table! {
    pulse_channel (id) {
        id -> Int4,
        pulse_id -> Int4,
        channel_type -> Varchar,
        details -> Text,
        schedule_type -> Varchar,
        schedule_hour -> Nullable<Int4>,
        schedule_day -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        schedule_frame -> Nullable<Varchar>,
        enabled -> Bool,
    }
}

table! {
    pulse_channel_recipient (id) {
        id -> Int4,
        pulse_channel_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    qrtz_blob_triggers (sched_name, trigger_name, trigger_group) {
        sched_name -> Varchar,
        trigger_name -> Varchar,
        trigger_group -> Varchar,
        blob_data -> Nullable<Bytea>,
    }
}

table! {
    qrtz_calendars (sched_name, calendar_name) {
        sched_name -> Varchar,
        calendar_name -> Varchar,
        calendar -> Bytea,
    }
}

table! {
    qrtz_cron_triggers (sched_name, trigger_name, trigger_group) {
        sched_name -> Varchar,
        trigger_name -> Varchar,
        trigger_group -> Varchar,
        cron_expression -> Varchar,
        time_zone_id -> Nullable<Varchar>,
    }
}

table! {
    qrtz_fired_triggers (sched_name, entry_id) {
        sched_name -> Varchar,
        entry_id -> Varchar,
        trigger_name -> Varchar,
        trigger_group -> Varchar,
        instance_name -> Varchar,
        fired_time -> Int8,
        sched_time -> Nullable<Int8>,
        priority -> Int4,
        state -> Varchar,
        job_name -> Nullable<Varchar>,
        job_group -> Nullable<Varchar>,
        is_nonconcurrent -> Nullable<Bool>,
        requests_recovery -> Nullable<Bool>,
    }
}

table! {
    qrtz_job_details (sched_name, job_name, job_group) {
        sched_name -> Varchar,
        job_name -> Varchar,
        job_group -> Varchar,
        description -> Nullable<Varchar>,
        job_class_name -> Varchar,
        is_durable -> Bool,
        is_nonconcurrent -> Bool,
        is_update_data -> Bool,
        requests_recovery -> Bool,
        job_data -> Nullable<Bytea>,
    }
}

table! {
    qrtz_locks (sched_name, lock_name) {
        sched_name -> Varchar,
        lock_name -> Varchar,
    }
}

table! {
    qrtz_paused_trigger_grps (sched_name, trigger_group) {
        sched_name -> Varchar,
        trigger_group -> Varchar,
    }
}

table! {
    qrtz_scheduler_state (sched_name, instance_name) {
        sched_name -> Varchar,
        instance_name -> Varchar,
        last_checkin_time -> Int8,
        checkin_interval -> Int8,
    }
}

table! {
    qrtz_simple_triggers (sched_name, trigger_name, trigger_group) {
        sched_name -> Varchar,
        trigger_name -> Varchar,
        trigger_group -> Varchar,
        repeat_count -> Int8,
        repeat_interval -> Int8,
        times_triggered -> Int8,
    }
}

table! {
    qrtz_simprop_triggers (sched_name, trigger_name, trigger_group) {
        sched_name -> Varchar,
        trigger_name -> Varchar,
        trigger_group -> Varchar,
        str_prop_1 -> Nullable<Varchar>,
        str_prop_2 -> Nullable<Varchar>,
        str_prop_3 -> Nullable<Varchar>,
        int_prop_1 -> Nullable<Int4>,
        int_prop_2 -> Nullable<Int4>,
        long_prop_1 -> Nullable<Int8>,
        long_prop_2 -> Nullable<Int8>,
        dec_prop_1 -> Nullable<Numeric>,
        dec_prop_2 -> Nullable<Numeric>,
        bool_prop_1 -> Nullable<Bool>,
        bool_prop_2 -> Nullable<Bool>,
    }
}

table! {
    qrtz_triggers (sched_name, trigger_name, trigger_group) {
        sched_name -> Varchar,
        trigger_name -> Varchar,
        trigger_group -> Varchar,
        job_name -> Varchar,
        job_group -> Varchar,
        description -> Nullable<Varchar>,
        next_fire_time -> Nullable<Int8>,
        prev_fire_time -> Nullable<Int8>,
        priority -> Nullable<Int4>,
        trigger_state -> Varchar,
        trigger_type -> Varchar,
        start_time -> Int8,
        end_time -> Nullable<Int8>,
        calendar_name -> Nullable<Varchar>,
        misfire_instr -> Nullable<Int2>,
        job_data -> Nullable<Bytea>,
    }
}

table! {
    report_card (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
        description -> Nullable<Text>,
        display -> Varchar,
        dataset_query -> Text,
        visualization_settings -> Text,
        creator_id -> Int4,
        database_id -> Nullable<Int4>,
        table_id -> Nullable<Int4>,
        query_type -> Nullable<Varchar>,
        archived -> Bool,
        collection_id -> Nullable<Int4>,
        public_uuid -> Nullable<Bpchar>,
        made_public_by_id -> Nullable<Int4>,
        enable_embedding -> Bool,
        embedding_params -> Nullable<Text>,
        cache_ttl -> Nullable<Int4>,
        result_metadata -> Nullable<Text>,
        read_permissions -> Nullable<Text>,
        collection_position -> Nullable<Int2>,
    }
}

table! {
    report_cardfavorite (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        card_id -> Int4,
        owner_id -> Int4,
    }
}

table! {
    report_dashboard (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
        description -> Nullable<Text>,
        creator_id -> Int4,
        parameters -> Text,
        points_of_interest -> Nullable<Text>,
        caveats -> Nullable<Text>,
        show_in_getting_started -> Bool,
        public_uuid -> Nullable<Bpchar>,
        made_public_by_id -> Nullable<Int4>,
        enable_embedding -> Bool,
        embedding_params -> Nullable<Text>,
        archived -> Bool,
        position -> Nullable<Int4>,
        collection_id -> Nullable<Int4>,
        collection_position -> Nullable<Int2>,
    }
}

table! {
    report_dashboardcard (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        sizeX -> Int4,
        sizeY -> Int4,
        row -> Int4,
        col -> Int4,
        card_id -> Nullable<Int4>,
        dashboard_id -> Int4,
        parameter_mappings -> Text,
        visualization_settings -> Text,
    }
}

table! {
    revision (id) {
        id -> Int4,
        model -> Varchar,
        model_id -> Int4,
        user_id -> Int4,
        timestamp -> Timestamptz,
        object -> Varchar,
        is_reversion -> Bool,
        is_creation -> Bool,
        message -> Nullable<Text>,
    }
}

table! {
    segment (id) {
        id -> Int4,
        table_id -> Int4,
        creator_id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        archived -> Bool,
        definition -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        points_of_interest -> Nullable<Text>,
        caveats -> Nullable<Text>,
        show_in_getting_started -> Bool,
    }
}

table! {
    setting (key) {
        key -> Varchar,
        value -> Text,
    }
}

table! {
    task_history (id) {
        id -> Int4,
        task -> Varchar,
        db_id -> Nullable<Int4>,
        started_at -> Timestamp,
        ended_at -> Timestamp,
        duration -> Int4,
        task_details -> Nullable<Text>,
    }
}

table! {
    view_log (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        model -> Varchar,
        model_id -> Int4,
        timestamp -> Timestamptz,
    }
}

joinable!(activity -> core_user (user_id));
joinable!(card_label -> label (label_id));
joinable!(card_label -> report_card (card_id));
joinable!(collection -> core_user (personal_owner_id));
joinable!(collection_revision -> core_user (user_id));
joinable!(computation_job -> core_user (creator_id));
joinable!(computation_job_result -> computation_job (job_id));
joinable!(core_session -> core_user (user_id));
joinable!(dashboard_favorite -> core_user (user_id));
joinable!(dashboard_favorite -> report_dashboard (dashboard_id));
joinable!(dashboardcard_series -> report_card (card_id));
joinable!(dashboardcard_series -> report_dashboardcard (dashboardcard_id));
joinable!(metabase_field -> metabase_table (table_id));
joinable!(metabase_fieldvalues -> metabase_field (field_id));
joinable!(metabase_table -> metabase_database (db_id));
joinable!(metric -> core_user (creator_id));
joinable!(metric -> metabase_table (table_id));
joinable!(metric_important_field -> metabase_field (field_id));
joinable!(metric_important_field -> metric (metric_id));
joinable!(permissions -> permissions_group (group_id));
joinable!(permissions_group_membership -> core_user (user_id));
joinable!(permissions_group_membership -> permissions_group (group_id));
joinable!(permissions_revision -> core_user (user_id));
joinable!(pulse -> collection (collection_id));
joinable!(pulse -> core_user (creator_id));
joinable!(pulse_card -> pulse (pulse_id));
joinable!(pulse_card -> report_card (card_id));
joinable!(pulse_channel -> pulse (pulse_id));
joinable!(pulse_channel_recipient -> core_user (user_id));
joinable!(pulse_channel_recipient -> pulse_channel (pulse_channel_id));
joinable!(report_card -> collection (collection_id));
joinable!(report_card -> metabase_database (database_id));
joinable!(report_card -> metabase_table (table_id));
joinable!(report_cardfavorite -> core_user (owner_id));
joinable!(report_cardfavorite -> report_card (card_id));
joinable!(report_dashboard -> collection (collection_id));
joinable!(report_dashboardcard -> report_card (card_id));
joinable!(report_dashboardcard -> report_dashboard (dashboard_id));
joinable!(revision -> core_user (user_id));
joinable!(segment -> core_user (creator_id));
joinable!(segment -> metabase_table (table_id));
joinable!(view_log -> core_user (user_id));

allow_tables_to_appear_in_same_query!(
    activity,
    card_label,
    collection,
    collection_revision,
    computation_job,
    computation_job_result,
    core_session,
    core_user,
    dashboardcard_series,
    dashboard_favorite,
    data_migrations,
    dependency,
    dimension,
    label,
    metabase_database,
    metabase_field,
    metabase_fieldvalues,
    metabase_table,
    metric,
    metric_important_field,
    permissions,
    permissions_group,
    permissions_group_membership,
    permissions_revision,
    pulse,
    pulse_card,
    pulse_channel,
    pulse_channel_recipient,
    qrtz_blob_triggers,
    qrtz_calendars,
    qrtz_cron_triggers,
    qrtz_fired_triggers,
    qrtz_job_details,
    qrtz_locks,
    qrtz_paused_trigger_grps,
    qrtz_scheduler_state,
    qrtz_simple_triggers,
    qrtz_simprop_triggers,
    qrtz_triggers,
    report_card,
    report_cardfavorite,
    report_dashboard,
    report_dashboardcard,
    revision,
    segment,
    setting,
    task_history,
    view_log,
);
