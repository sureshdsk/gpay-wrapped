import { useState, useRef, useCallback, useMemo } from 'react';
import { useDataStore } from '../stores/dataStore';
import { useNavigate } from 'react-router-dom';
import html2canvas from 'html2canvas';
import { convertToINR } from '../utils/categoryUtils';
import { filterTransactionsByYear, filterActivitiesByYear } from '../utils/dateUtils';
import styles from './Wrapped.module.css';

interface SlideData {
  id: string;
  title: string;
  value: string;
  subtitle: string;
  icon: string;
  gradient: string;
  detail?: string;
}

const getCategoryIcon = (category: string): string => {
  const icons: Record<string, string> = {
    Food: '🍕',
    Groceries: '🛒',
    Clothing: '👕',
    Entertainment: '🎬',
    'E-commerce': '🛍️',
    'Travel & Transport': '🚗',
    'Utilities & Bills': '💡',
    Healthcare: '🏥',
    Education: '📚',
    Investments: '📈',
    Transfers: '💸',
    'Bank Transfers': '🏦',
    Others: '📦',
  };
  return icons[category] || '📊';
};

export default function Wrapped() {
  const navigate = useNavigate();
  const { parsedData, insights, selectedYear } = useDataStore();
  const [currentSlide, setCurrentSlide] = useState(0);
  const [isSharing, setIsSharing] = useState(false);
  const slideRef = useRef<HTMLDivElement>(null);

  // Generate slides from insights and data
  const slides: SlideData[] = useMemo(() => {
    if (!parsedData) return [];

    const filteredTransactions = filterTransactionsByYear(parsedData.transactions, selectedYear);
    const filteredActivities = filterActivitiesByYear(parsedData.activities, selectedYear);

    // Calculate total spent
    const totalSpent = filteredTransactions.reduce((sum, t) => sum + convertToINR(t.amount), 0) +
      filteredActivities
        .filter(a => a.amount && (a.transactionType === 'sent' || a.transactionType === 'paid'))
        .reduce((sum, a) => sum + convertToINR(a.amount!), 0);

    const formatAmount = (amount: number) => {
      if (amount >= 100000) return `${(amount / 100000).toFixed(1)}L`;
      if (amount >= 1000) return `${(amount / 1000).toFixed(1)}K`;
      return Math.round(amount).toLocaleString();
    };

    const generatedSlides: SlideData[] = [
      // Intro slide
      {
        id: 'intro',
        title: 'Your GPay',
        value: selectedYear === 'all' ? 'All Time' : selectedYear,
        subtitle: 'Wrapped',
        icon: '🎉',
        gradient: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
      },
      // Total spent
      {
        id: 'total',
        title: 'You spent a total of',
        value: `₹${formatAmount(totalSpent)}`,
        subtitle: `across ${filteredActivities.length + filteredTransactions.length} transactions`,
        icon: '💸',
        gradient: 'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)',
      },
    ];

    // Add insight-based slides
    insights.forEach(insight => {
      switch (insight.type) {
        case 'money_flow': {
          const data = insight.data as any;
          generatedSlides.push({
            id: 'money_flow',
            title: data.flowDirection === 'giver' ? 'The Generous One' :
                   data.flowDirection === 'receiver' ? 'Money Magnet' : 'Balanced Flow',
            value: `₹${formatAmount(data.totalSent.value)}`,
            subtitle: `sent to friends & family`,
            icon: data.flowDirection === 'giver' ? '🎁' : data.flowDirection === 'receiver' ? '🧲' : '⚖️',
            gradient: 'linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)',
            detail: `Received ₹${formatAmount(data.totalReceived.value)} back`,
          });
          break;
        }
        case 'spending_category': {
          const data = insight.data as any;
          generatedSlides.push({
            id: 'top_category',
            title: 'Your top spending was',
            value: data.topCategory,
            subtitle: `₹${formatAmount(data.topCategoryAmount.value)} spent`,
            icon: getCategoryIcon(data.topCategory),
            gradient: 'linear-gradient(135deg, #fa709a 0%, #fee140 100%)',
            detail: `${data.topCategoryCount} transactions`,
          });
          break;
        }
        case 'peak_activity': {
          const data = insight.data as any;
          generatedSlides.push({
            id: 'peak_time',
            title: 'Your peak payment time',
            value: `${data.peakDay}s`,
            subtitle: `at ${data.peakHour}:00`,
            icon: data.nightOwlScore > 30 ? '🦉' : '☀️',
            gradient: 'linear-gradient(135deg, #a18cd1 0%, #fbc2eb 100%)',
            detail: data.nightOwlScore > 30 ? `${data.nightOwlScore}% payments after 10pm` : undefined,
          });
          break;
        }
        case 'transaction_partner': {
          const data = insight.data as any;
          if (data.mostFrequentPartner) {
            generatedSlides.push({
              id: 'transaction_partner',
              title: 'Your top payment partner',
              value: data.mostFrequentPartner,
              subtitle: `₹${formatAmount(data.totalAmount.value)}`,
              icon: '👤',
              gradient: 'linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)',
              detail: `${data.transactionCount} transactions`,
            });
          }
          break;
        }
        case 'expensive_day': {
          const data = insight.data as any;
          const date = new Date(data.date);
          generatedSlides.push({
            id: 'expensive_day',
            title: 'Your biggest spending day',
            value: date.toLocaleDateString('en-IN', { month: 'short', day: 'numeric' }),
            subtitle: `₹${formatAmount(data.amount)} spent`,
            icon: '📅',
            gradient: 'linear-gradient(135deg, #ff9a9e 0%, #fecfef 100%)',
          });
          break;
        }
      }
    });

    // Outro slide
    generatedSlides.push({
      id: 'outro',
      title: "That's your",
      value: 'GPay Wrapped',
      subtitle: selectedYear === 'all' ? 'All Time' : selectedYear,
      icon: '✨',
      gradient: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
      detail: 'Share with friends!',
    });

    return generatedSlides;
  }, [parsedData, insights, selectedYear]);

  const nextSlide = useCallback(() => {
    setCurrentSlide(prev => (prev + 1) % slides.length);
  }, [slides.length]);

  const prevSlide = useCallback(() => {
    setCurrentSlide(prev => (prev - 1 + slides.length) % slides.length);
  }, [slides.length]);

  const shareSlide = useCallback(async () => {
    if (!slideRef.current) return;

    setIsSharing(true);
    try {
      const canvas = await html2canvas(slideRef.current, {
        backgroundColor: null,
        scale: 2,
        useCORS: true,
      });

      // Convert to blob and share or download
      canvas.toBlob(async (blob) => {
        if (!blob) return;

        const file = new File([blob], `gpay-wrapped-${slides[currentSlide].id}.png`, {
          type: 'image/png',
        });

        // Try native share if available
        if (navigator.share && navigator.canShare({ files: [file] })) {
          try {
            await navigator.share({
              files: [file],
              title: 'My GPay Wrapped',
              text: 'Check out my GPay Wrapped!',
            });
          } catch (err) {
            // User cancelled or error, fallback to download
            downloadImage(canvas);
          }
        } else {
          // Fallback to download
          downloadImage(canvas);
        }
      }, 'image/png');
    } catch (error) {
      console.error('Error sharing:', error);
    } finally {
      setIsSharing(false);
    }
  }, [currentSlide, slides]);

  const downloadImage = (canvas: HTMLCanvasElement) => {
    const link = document.createElement('a');
    link.download = `gpay-wrapped-${slides[currentSlide].id}.png`;
    link.href = canvas.toDataURL('image/png');
    link.click();
  };

  if (!parsedData || slides.length === 0) {
    navigate('/');
    return null;
  }

  const slide = slides[currentSlide];

  return (
    <div className={styles.wrapped}>
      {/* Progress bar */}
      <div className={styles.progressBar}>
        {slides.map((_, index) => (
          <div
            key={index}
            className={`${styles.progressSegment} ${index <= currentSlide ? styles.active : ''}`}
            onClick={() => setCurrentSlide(index)}
          />
        ))}
      </div>

      {/* Main slide area */}
      <div
        className={styles.slideContainer}
        onClick={(e) => {
          const rect = e.currentTarget.getBoundingClientRect();
          const x = e.clientX - rect.left;
          if (x < rect.width / 3) {
            prevSlide();
          } else if (x > (rect.width * 2) / 3) {
            nextSlide();
          }
        }}
      >
        <div className={styles.slideWrapper}>
          <div
            ref={slideRef}
            className={styles.slide}
            style={{ background: slide.gradient }}
          >
            <div className={styles.slideContent}>
              <div className={styles.slideIcon}>{slide.icon}</div>
              <h2 className={styles.slideTitle}>{slide.title}</h2>
              <div className={styles.slideValue}>{slide.value}</div>
              <p className={styles.slideSubtitle}>{slide.subtitle}</p>
              {slide.detail && <p className={styles.slideDetail}>{slide.detail}</p>}
              <div className={styles.watermark}>GPay Wrapped {selectedYear}</div>
            </div>
          </div>
          {/* Share icon overlay */}
          <button
            className={styles.shareIcon}
            onClick={(e) => {
              e.stopPropagation();
              shareSlide();
            }}
            disabled={isSharing}
            aria-label="Share slide"
          >
            {isSharing ? '...' : '↗'}
          </button>
        </div>
      </div>

      {/* Navigation hints */}
      <div className={styles.navHints}>
        <span className={styles.navHint}>← Tap left</span>
        <span className={styles.slideCounter}>{currentSlide + 1} / {slides.length}</span>
        <span className={styles.navHint}>Tap right →</span>
      </div>

      {/* Action buttons */}
      <div className={styles.actions}>
        <button
          className={styles.exitButton}
          onClick={() => navigate('/story')}
        >
          Exit
        </button>
      </div>
    </div>
  );
}
