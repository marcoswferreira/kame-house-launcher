export type Language = 'en' | 'pt';

export interface StatusResponse {
  version: string;
  language: string;
  games_count: number;
  launch_behavior: string;
  sunshine_url: string;
  sunshine_enabled?: boolean;
  sunshine_running?: boolean;
}

export interface Game {
  id: string;
  title: string;
  platform?: string;
  platform_name?: string;
  path: string;
  custom_exe?: string;
  shortcut_id?: number;
  art?: {
    cover?: string;
    hero?: string;
    logo?: string;
    icon?: string;
  };
}

export interface SunshineAppPrepCommand {
  do?: string;
  undo?: string;
  elevated?: boolean;
}

export interface SunshineApp {
  name: string;
  output?: string;
  cmd?: string;
  index?: number;
  image_path?: string;
  auto_detach?: string;
  wait_all?: string;
  exit_timeout?: string;
  prep_cmd?: SunshineAppPrepCommand[];
  working_dir?: string;
  exclude_global_prep_cmd?: string;
  elevated?: string;
}

export interface SunshineClientDevice {
  uuid: string;
  name: string;
  enabled: boolean;
  ip?: string;
  last_seen?: string;
}

export interface SunshineConfig {
  sunshine_name?: string;
  min_log_level?: number;
  system_tray?: string;

  // Input
  controller?: string;
  gamepad_driver?: string;
  gamepad?: string;
  ds4_back_as_touchpad_click?: string;
  motion_as_ds4?: string;
  mouse?: string;
  keyboard?: string;

  // AV
  audio_sink?: string;
  virtual_sink?: string;
  stream_audio?: string;
  adapter_name?: string;
  output_name?: string;
  dd_resolution_option?: string;
  dd_refresh_rate_option?: string;
  dd_hdr_option?: string;
  max_bitrate?: number;
  minimum_fps_target?: number;

  // Network
  port?: number;
  upnp?: string;
  address_family?: string;
  bind_address?: string;
  lan_encryption_mode?: number;

  // Advanced & Encoders
  fec_percentage?: number;
  qp?: number;
  min_threads?: number;
  hevc_mode?: number;
  av1_mode?: number;
  encoder?: string;

  // NVENC
  nvenc_preset?: number;
  nvenc_twopass?: string;
  nvenc_spatial_aq?: string;
  nvenc_realtime_hags?: string;
  nvenc_latency_over_power?: string;

  // AMD
  amd_usage?: string;
  amd_rc?: string;
  amd_quality?: string;
  amd_vbaq?: string;

  // Intel
  qsv_preset?: string;
  qsv_coder?: string;
  qsv_slow_hevc?: string;
}

