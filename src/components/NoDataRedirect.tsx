import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import styles from './NoDataRedirect.module.css';

interface NoDataRedirectProps {
  /** Countdown duration in seconds before redirecting */
  countdown?: number;
}

export default function NoDataRedirect({ countdown = 5 }: NoDataRedirectProps) {
  const navigate = useNavigate();
  const [seconds, setSeconds] = useState(countdown);

  useEffect(() => {
    // Redirect immediately if countdown is 0
    if (countdown === 0) {
      navigate('/');
      return;
    }

    // Start countdown
    const interval = setInterval(() => {
      setSeconds((prev) => {
        if (prev <= 1) {
          navigate('/');
          return 0;
        }
        return prev - 1;
      });
    }, 1000);

    return () => clearInterval(interval);
  }, [countdown, navigate]);

  const handleGoNow = () => {
    navigate('/');
  };

  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div className={styles.icon}>📁</div>
        <h1 className={styles.title}>No Data Found</h1>
        <p className={styles.message}>
          You need to upload your Google Pay data first to view this page.
        </p>
        <p className={styles.instructions}>
          Get your data by exporting from{' '}
          <a
            href="https://takeout.google.com"
            target="_blank"
            rel="noopener noreferrer"
            className={styles.link}
          >
            Google Takeout
          </a>
          , then upload the ZIP file on the home page.
        </p>

        <div className={styles.actions}>
          <button onClick={handleGoNow} className={styles.primaryButton}>
            Go to Upload Page
          </button>
          <p className={styles.countdown}>
            Redirecting automatically in {seconds} second{seconds !== 1 ? 's' : ''}...
          </p>
        </div>
      </div>
    </div>
  );
}
