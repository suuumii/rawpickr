export type PhotoFileType = "jpg" | "raw" | "both";

export interface Photo {
  path: string;
  filename: string;
  rating: number;
  file_type: PhotoFileType;
}

export interface ExifInfo {
  camera: string | null;
  lens: string | null;
  f_number: number | null;
  shutter_speed: string | null;
  iso: number | null;
  focal_length: number | null;
  taken_at: string | null;
}

export interface OrganizerResult {
  folder_count: number;
  moved_count: number;
  skipped_count: number;
  logs: string[];
}

export interface SortResult {
  copied_count: number;
  skipped_count: number;
  logs: string[];
}

export type DeleteMode = "both" | "raw_only";
