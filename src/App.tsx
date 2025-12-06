import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Landing from './pages/Landing';
import Processing from './pages/Processing';
import Story from './pages/Story';
import Categories from './pages/Categories';
import Wrapped from './pages/Wrapped';
import './App.css';

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Landing />} />
        <Route path="/processing" element={<Processing />} />
        <Route path="/story" element={<Story />} />
        <Route path="/categories" element={<Categories />} />
        <Route path="/wrapped" element={<Wrapped />} />
      </Routes>
    </Router>
  );
}

export default App;
