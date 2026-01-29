import { Routes, Route, NavLink } from "react-router-dom";
import { Toaster } from "react-hot-toast";
import { useState } from "react";
import Dashboard from "./pages/Dashboard";
import Settings from "./pages/Settings";
import OutputStyle from "./pages/OutputStyle";
import Mcp from "./pages/Mcp";
import Skills from "./pages/Skills";
import Profiles from "./pages/Profiles";
import Diagnostics from "./pages/Diagnostics";
import Release from "./pages/Release";
import NotFound from "./pages/NotFound";

function App() {
  const [sidebarOpen, setSidebarOpen] = useState(false);

  return (
    <div className="app">
      <Toaster position="top-right" />

      {/* Mobile menu button */}
      <button
        className="menu-toggle"
        onClick={() => setSidebarOpen(!sidebarOpen)}
        aria-label={sidebarOpen ? "Close menu" : "Open menu"}
      >
        {sidebarOpen ? "✕" : "☰"}
      </button>

      <nav className={`sidebar ${sidebarOpen ? "open" : ""}`} role="navigation">
        <h1>Rhinolabs AI</h1>
        <ul>
          <li>
            <NavLink to="/" end onClick={() => setSidebarOpen(false)}>
              Dashboard
            </NavLink>
          </li>
          <li>
            <NavLink to="/skills" onClick={() => setSidebarOpen(false)}>
              Skills
            </NavLink>
          </li>
          <li>
            <NavLink to="/profiles" onClick={() => setSidebarOpen(false)}>
              Profiles
            </NavLink>
          </li>
          <li>
            <NavLink to="/output-style" onClick={() => setSidebarOpen(false)}>
              Output Style
            </NavLink>
          </li>
          <li>
            <NavLink to="/mcp" onClick={() => setSidebarOpen(false)}>
              MCP
            </NavLink>
          </li>
          <li>
            <NavLink to="/settings" onClick={() => setSidebarOpen(false)}>
              Settings
            </NavLink>
          </li>
          <li>
            <NavLink to="/diagnostics" onClick={() => setSidebarOpen(false)}>
              Diagnostics
            </NavLink>
          </li>
          <li>
            <NavLink to="/release" onClick={() => setSidebarOpen(false)}>
              Release
            </NavLink>
          </li>
        </ul>
      </nav>

      <main className="content">
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/profiles" element={<Profiles />} />
          <Route path="/skills" element={<Skills />} />
          <Route path="/mcp" element={<Mcp />} />
          <Route path="/output-style" element={<OutputStyle />} />
          <Route path="/settings" element={<Settings />} />
          <Route path="/diagnostics" element={<Diagnostics />} />
          <Route path="/release" element={<Release />} />
          <Route path="*" element={<NotFound />} />
        </Routes>
      </main>
    </div>
  );
}

export default App;
