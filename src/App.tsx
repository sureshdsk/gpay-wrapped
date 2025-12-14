import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Landing from './pages/Landing';
import Processing from './pages/Processing';
import Story from './pages/Story';
import Categories from './pages/Categories';
import Wrapped from './pages/Wrapped';
import DataTable from './pages/DataTable';
import About from './pages/About';
import { usePageTracking } from './hooks/usePageTracking';
import './App.css';

function AppContent() {
  usePageTracking();

  return (
    <Routes>
      <Route path="/" element={<Landing />} />
      <Route path="/processing" element={<Processing />} />
      <Route path="/insights" element={<Story />} />
      <Route path="/categories" element={<Categories />} />
      <Route path="/story" element={<Wrapped />} />
      <Route path="/explore-data" element={<DataTable />} />
      <Route path="/about" element={<About />} />
    </Routes>
  );
}

function App() {
  return (
    <Router>
      <AppContent />
    </Router>
  );
}

export default App;
