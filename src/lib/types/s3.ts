export interface S3Bucket {
  name: string;
  creation_date: number | null;
  region: string | null;
}

export interface S3Object {
  key: string;
  size: number;
  last_modified: number | null;
  storage_class: string | null;
  is_folder: boolean;
}

export interface S3ListResult {
  objects: S3Object[];
  common_prefixes: string[];
  is_truncated: boolean;
  next_continuation_token: string | null;
}

export interface S3BrowserState {
  bucket: string;
  prefix: string;
  objects: S3Object[];
  folders: string[];
  loading: boolean;
  error: string | null;
}

export type S3ViewMode = 'list' | 'grid';

export interface S3UploadProgress {
  filename: string;
  progress: number;
  status: 'pending' | 'uploading' | 'completed' | 'error';
  error?: string;
}
