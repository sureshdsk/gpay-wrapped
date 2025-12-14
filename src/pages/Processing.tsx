import { useEffect, useState } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { useDataStore } from '../stores/dataStore';
import { extractZipFile } from '../utils/zipParser';
import styles from './Processing.module.css';

type ProcessingStage = 'extracting' | 'parsing' | 'calculating' | 'complete' | 'error';

export default function Processing() {
  const navigate = useNavigate();
  const location = useLocation();
  const [stage, setStage] = useState<ProcessingStage>('extracting');
  const [error, setError] = useState<string | null>(null);

  const { setRawData, parseRawData, recalculateInsights, selectedYear } = useDataStore();

  useEffect(() => {
    processFile();
  }, []);

  const processFile = async () => {
    try {
      // Get file from navigation state
      const file = location.state?.file as File | undefined;

      if (!file) {
        setError('No file provided');
        setStage('error');
        return;
      }

      // Stage 1: Extract zip file
      setStage('extracting');
      await sleep(500); // Small delay for UX

      const extractResult = await extractZipFile(file);

      if (!extractResult.success || !extractResult.data) {
        setError(extractResult.error || 'Failed to extract zip file');
        setStage('error');
        return;
      }

      // Store raw data
      setRawData(extractResult.data);

      // Stage 2: Parse data
      setStage('parsing');
      await sleep(500);

      parseRawData();

      // Stage 3: Calculate insights
      setStage('calculating');
      await sleep(500);

      recalculateInsights(selectedYear);

      // Stage 4: Complete
      setStage('complete');
      await sleep(500);

      // Navigate to insights page
      navigate('/insights');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error occurred');
      setStage('error');
    }
  };

  const handleRetry = () => {
    navigate('/');
  };

  if (stage === 'error') {
    return (
      <div className={styles.processing}>
        <div className={styles.errorContainer}>
          <div className={styles.errorIcon}>⚠️</div>
          <h2 className={styles.errorTitle}>Processing Failed</h2>
          <p className={styles.errorMessage}>{error}</p>
          <button onClick={handleRetry} className={styles.retryButton}>
            Try Again
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className={styles.processing}>
      <div className={styles.container}>
        <h1 className={styles.title}>Processing Your Data</h1>
        <p className={styles.subtitle}>This will only take a moment...</p>

        <div className={styles.stagesContainer}>
          <ProcessingStage
            icon="📦"
            label="Extracting files"
            active={stage === 'extracting'}
            complete={['parsing', 'calculating', 'complete'].includes(stage)}
          />
          <ProcessingStage
            icon="📊"
            label="Parsing data"
            active={stage === 'parsing'}
            complete={['calculating', 'complete'].includes(stage)}
          />
          <ProcessingStage
            icon="🔮"
            label="Calculating insights"
            active={stage === 'calculating'}
            complete={stage === 'complete'}
          />
        </div>

        <div className={styles.loader}>
          <div className={styles.spinner}></div>
        </div>

        <p className={styles.privacyNote}>
          🔒 All processing happens in your browser. Your data never leaves your device.
        </p>
      </div>
    </div>
  );
}

interface ProcessingStageProps {
  icon: string;
  label: string;
  active: boolean;
  complete: boolean;
}

function ProcessingStage({ icon, label, active, complete }: ProcessingStageProps) {
  const stageClasses = [
    styles.stage,
    active && styles.stageActive,
    complete && styles.stageComplete,
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <div className={stageClasses}>
      <div className={styles.stageIcon}>{complete ? '✓' : icon}</div>
      <div className={styles.stageLabel}>{label}</div>
    </div>
  );
}

// Helper function for delays
function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}
