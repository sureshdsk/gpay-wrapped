import { useEffect } from 'react';
import { useLocation } from 'react-router-dom';

// Extend Window interface to include gtag and dataLayer
declare global {
  interface Window {
    gtag?: (
      command: string,
      targetId: string,
      config?: Record<string, unknown>
    ) => void;
    dataLayer?: unknown[];
  }
}

/**
 * Hook to track page views with Google Analytics
 * Implements retry mechanism and proper page view tracking
 */
export function usePageTracking() {
  const location = useLocation();

  useEffect(() => {
    const trackPageView = () => {
      // Check if gtag is loaded
      if (typeof window.gtag === 'function') {
        const page_path = location.pathname + location.search;
        const page_location = window.location.href;
        const page_title = document.title;

        window.gtag('event', 'page_view', {
          page_path,
          page_location,
          page_title,
        });

        return true;
      }
      return false;
    };

    // Try to track immediately
    if (trackPageView()) {
      return;
    }

    // If gtag not ready, retry with exponential backoff
    let retryCount = 0;
    const maxRetries = 5;

    const retryInterval = setInterval(() => {
      retryCount++;

      if (trackPageView()) {
        clearInterval(retryInterval);
      } else if (retryCount >= maxRetries) {
        clearInterval(retryInterval);
      }
    }, 500); // Retry every 500ms

    return () => clearInterval(retryInterval);
  }, [location]);
}
