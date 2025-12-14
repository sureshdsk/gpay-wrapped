import { useCallback, useState } from 'react';
import styles from './DropZone.module.css';

interface DropZoneProps {
  onUpload: (file: File) => void;
  disabled?: boolean;
}

export default function DropZone({ onUpload, disabled = false }: DropZoneProps) {
  const [isDragging, setIsDragging] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const validateFile = (file: File): boolean => {
    setError(null);

    if (!file.name.endsWith('.zip')) {
      setError('Please upload a ZIP file from Google Takeout');
      return false;
    }

    // Max 100MB
    if (file.size > 100 * 1024 * 1024) {
      setError('File size too large. Maximum 100MB allowed');
      return false;
    }

    return true;
  };

  const handleFile = (file: File) => {
    if (validateFile(file)) {
      onUpload(file);
    }
  };

  const handleDrop = useCallback(
    (e: React.DragEvent<HTMLDivElement>) => {
      e.preventDefault();
      setIsDragging(false);

      if (disabled) return;

      const files = Array.from(e.dataTransfer.files);
      if (files.length > 0) {
        handleFile(files[0]);
      }
    },
    [disabled, onUpload]
  );

  const handleDragOver = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(false);
  }, []);

  const handleFileInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length > 0) {
      handleFile(files[0]);
    }
  };

  const dropzoneClasses = [
    styles.dropzone,
    isDragging && styles.dragging,
    disabled && styles.disabled
  ].filter(Boolean).join(' ');

  return (
    <div className={styles.container}>
      <div
        className={dropzoneClasses}
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
      >
        <input
          type="file"
          id="file-upload"
          accept=".zip"
          onChange={handleFileInput}
          disabled={disabled}
          className="hidden"
        />

        <label htmlFor="file-upload" className={styles.label}>
          <div className={styles.icon}>📦</div>
          <h3 className={styles.title}>Upload Your Google Pay Data</h3>
          <p className={styles.description}>Drag and drop your Google Takeout ZIP file here</p>
          <p className={styles.or}>or</p>
          <button
            type="button"
            className={styles.uploadButton}
            onClick={() => document.getElementById('file-upload')?.click()}
            disabled={disabled}
          >
            Browse Files
          </button>
        </label>
      </div>

      {error && (
        <div className={styles.error}>
          ⚠️ {error}
        </div>
      )}
    </div>
  );
}
