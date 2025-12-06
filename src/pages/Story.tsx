import { useMemo } from 'react';
import { useDataStore } from '../stores/dataStore';
import { useNavigate } from 'react-router-dom';
import { convertToINR } from '../utils/categoryUtils';
import { filterTransactionsByYear, filterActivitiesByYear } from '../utils/dateUtils';
import styles from './Story.module.css';

export default function Story() {
  const navigate = useNavigate();
  const { parsedData, insights, selectedYear } = useDataStore();

  // Filter data by selected year
  const filteredData = useMemo(() => {
    if (!parsedData) return null;
    return {
      transactions: filterTransactionsByYear(parsedData.transactions, selectedYear),
      activities: filterActivitiesByYear(parsedData.activities, selectedYear),
    };
  }, [parsedData, selectedYear]);

  // Calculate total spent
  const totalSpent = useMemo(() => {
    if (!filteredData) return 0;

    // Sum from transactions
    const transactionTotal = filteredData.transactions.reduce(
      (sum, t) => sum + convertToINR(t.amount),
      0
    );

    // Sum from activities (sent and paid only)
    const activityTotal = filteredData.activities
      .filter(a => a.amount && (a.transactionType === 'sent' || a.transactionType === 'paid'))
      .reduce((sum, a) => sum + convertToINR(a.amount!), 0);

    return transactionTotal + activityTotal;
  }, [filteredData]);

  if (!parsedData) {
    navigate('/');
    return null;
  }

  const formatAmount = (amount: number) => {
    if (amount >= 100000) {
      return `${(amount / 100000).toFixed(1)}L`;
    } else if (amount >= 1000) {
      return `${(amount / 1000).toFixed(1)}K`;
    }
    return Math.round(amount).toLocaleString();
  };

  return (
    <div className={styles.story}>
      <div className={styles.container}>
        <h1 className={styles.title}>Your GPay Wrapped {selectedYear === 'all' ? '(All Time)' : selectedYear}</h1>

        <div className={styles.debugInfo}>
          <h2>Data Loaded Successfully!</h2>

          {/* Total Spent Highlight */}
          <div className={styles.totalSpentCard}>
            <div className={styles.totalSpentAmount}>₹{formatAmount(totalSpent)}</div>
            <div className={styles.totalSpentLabel}>Total Spent in {selectedYear === 'all' ? 'All Time' : selectedYear}</div>
          </div>

          <div className={styles.stats}>
            <div className={styles.stat}>
              <div className={styles.statNumber}>{filteredData?.activities.length ?? 0}</div>
              <div className={styles.statLabel}>Activities</div>
            </div>
            <div className={styles.stat}>
              <div className={styles.statNumber}>{filteredData?.transactions.length ?? 0}</div>
              <div className={styles.statLabel}>Transactions</div>
            </div>
            <div className={styles.stat}>
              <div className={styles.statNumber}>{parsedData.groupExpenses.length}</div>
              <div className={styles.statLabel}>Group Expenses</div>
            </div>
            <div className={styles.stat}>
              <div className={styles.statNumber}>{parsedData.voucherRewards.length}</div>
              <div className={styles.statLabel}>Vouchers</div>
            </div>
          </div>

          <div className={styles.insightsSection}>
            <h2>{insights.length} Insights Generated</h2>
            {insights.length > 0 ? (
              <div className={styles.insightsList}>
                {insights.map((insight, index) => (
                  <div key={index} className={styles.insightCard}>
                    <h3 className={styles.insightTitle}>{insight.title}</h3>
                    <p className={styles.insightMessage}>{insight.message}</p>
                    <div className={styles.insightMeta}>
                      <span className={styles.insightType}>{insight.type}</span>
                      {insight.tone && <span className={styles.insightTone}>{insight.tone}</span>}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <p className={styles.noInsights}>No insights generated for this time period.</p>
            )}
          </div>

          <div className={styles.actionButtons}>
            <button onClick={() => navigate('/wrapped')} className={styles.wrappedButton}>
              View Your Wrapped
            </button>
            <button onClick={() => navigate('/categories')} className={styles.categoriesButton}>
              View Spending Categories
            </button>
            <button onClick={() => navigate('/')} className={styles.backButton}>
              Upload Another File
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
