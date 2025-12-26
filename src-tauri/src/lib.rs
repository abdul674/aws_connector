mod aws;
mod commands;
mod logs;
mod terminal;

use commands::{
    // AWS profile commands
    add_aws_profile,
    add_aws_sso_profile,
    check_aws_cli,
    check_profile_exists,
    check_ssm_plugin,
    delete_aws_profile,
    get_profile_region,
    list_aws_profiles,
    list_aws_regions,
    sso_login,
    validate_credentials,
    // Resource discovery commands
    discover_resources,
    list_ec2_instances,
    list_ecs_clusters,
    list_ecs_services,
    list_ecs_tasks,
    // Terminal commands
    terminal_close,
    terminal_create_session,
    terminal_get_session,
    terminal_list_sessions,
    terminal_resize,
    terminal_write,
    // CloudWatch Logs commands
    get_cloudwatch_log_events,
    list_cloudwatch_log_groups,
    list_cloudwatch_log_streams,
    list_log_tail_sessions,
    start_log_tail,
    stop_log_tail,
    // S3 commands
    delete_s3_object,
    download_s3_object,
    get_s3_object_content,
    get_s3_presigned_url,
    head_s3_object,
    list_s3_buckets,
    list_s3_objects,
    upload_s3_object,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // AWS profile commands
            list_aws_profiles,
            list_aws_regions,
            check_aws_cli,
            check_ssm_plugin,
            get_profile_region,
            check_profile_exists,
            add_aws_profile,
            add_aws_sso_profile,
            delete_aws_profile,
            sso_login,
            validate_credentials,
            // Resource discovery commands
            discover_resources,
            list_ecs_clusters,
            list_ecs_services,
            list_ecs_tasks,
            list_ec2_instances,
            // Terminal commands
            terminal_create_session,
            terminal_write,
            terminal_resize,
            terminal_close,
            terminal_list_sessions,
            terminal_get_session,
            // CloudWatch Logs commands
            list_cloudwatch_log_groups,
            list_cloudwatch_log_streams,
            get_cloudwatch_log_events,
            start_log_tail,
            stop_log_tail,
            list_log_tail_sessions,
            // S3 commands
            list_s3_buckets,
            list_s3_objects,
            download_s3_object,
            upload_s3_object,
            delete_s3_object,
            get_s3_presigned_url,
            head_s3_object,
            get_s3_object_content,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
