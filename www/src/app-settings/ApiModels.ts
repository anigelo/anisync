export interface ApiConfig {
    MEGA_USER: String | null,
    MEGA_PWD: String | null,
    DOWNLOAD_FOLDER: String,
    LOCAL_MEDIA_ROOT: String,
    REMOTE_MEDIA_ROOT: String | null,
    SCAN_INTERVAL: Number
}

export interface ApiSyncDir {
    remote: String,
    local: String
}

export interface ApiFilters {
    contains: String[],
    not_contains: String[]
}