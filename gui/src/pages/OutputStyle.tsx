import { useEffect, useState } from 'react';
import { api } from '../api';
import type { OutputStyle as OutputStyleType, IdeInfo } from '../types';
import toast from 'react-hot-toast';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';

export default function OutputStyle() {
  const [style, setStyle] = useState<OutputStyleType | null>(null);
  const [loading, setLoading] = useState(true);
  const [availableIdes, setAvailableIdes] = useState<IdeInfo[]>([]);

  useEffect(() => {
    loadStyle();
    loadIdes();
  }, []);

  async function loadStyle() {
    try {
      // Get the active style or the first available
      const active = await api.getActiveOutputStyle();
      if (active) {
        setStyle(active);
      } else {
        // Try to get any style
        const styles = await api.listOutputStyles();
        if (styles.length > 0) {
          setStyle(styles[0]);
        }
      }
    } catch (err) {
      toast.error('Failed to load output style');
    } finally {
      setLoading(false);
    }
  }

  async function loadIdes() {
    try {
      const ides = await api.listAvailableIdes();
      setAvailableIdes(ides.filter((ide) => ide.available));
    } catch (err) {
      console.error('Failed to load IDEs:', err);
    }
  }

  async function handleOpenInIde() {
    if (!style) {
      toast.error('No output style to edit');
      return;
    }
    if (availableIdes.length === 0) {
      toast.error('No IDE available. Install VS Code, Cursor, or Zed.');
      return;
    }
    try {
      await api.openOutputStyleInIde(style.id, availableIdes[0].command);
      toast.success('Opened in ' + availableIdes[0].name);
    } catch (err) {
      toast.error('Failed to open in IDE');
    }
  }

  if (loading) {
    return (
      <div className="loading">
        <div className="spinner" />
      </div>
    );
  }

  return (
    <div>
      <div className="page-header" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
        <div>
          <h1>Output Style</h1>
          <p>Define Claude's personality, tone, and response format</p>
        </div>
        <button className="btn btn-primary" onClick={handleOpenInIde}>
          Edit
        </button>
      </div>

      {/* Main-Profile Link Notice */}
      <div style={{
        background: 'var(--bg-secondary)',
        border: '1px solid var(--border)',
        borderRadius: '0.5rem',
        padding: '0.75rem 1rem',
        marginBottom: '1rem',
        display: 'flex',
        alignItems: 'center',
        gap: '0.5rem',
        fontSize: '0.875rem',
        color: 'var(--text-secondary)',
      }}>
        <span style={{ color: '#8b5cf6' }}>●</span>
        <span>
          <strong>Linked to Main-Profile:</strong> This output style is installed to <code>~/.claude/output-styles/</code> when you run <code>rhinolabs install</code>
        </span>
      </div>

      <div className="card">
        {style && (
          <div style={{ marginBottom: '1rem' }}>
            <h3 style={{ marginBottom: '0.25rem' }}>{style.name}</h3>
            <p style={{ color: 'var(--text-secondary)', fontSize: '0.875rem' }}>
              {style.description}
            </p>
            <p style={{ color: 'var(--text-secondary)', fontSize: '0.75rem', marginTop: '0.5rem' }}>
              Keep coding instructions: <strong>{style.keepCodingInstructions ? 'Yes' : 'No'}</strong>
            </p>
          </div>
        )}

        <div style={{
          border: '1px solid var(--border)',
          borderRadius: '0.5rem',
          overflow: 'auto',
          maxHeight: 'calc(100vh - 380px)',
        }}>
          <SyntaxHighlighter
            language="markdown"
            style={vscDarkPlus}
            showLineNumbers
            customStyle={{
              margin: 0,
              borderRadius: '0.5rem',
              fontSize: '0.875rem',
            }}
          >
            {style?.content || '# No content yet\n\nClick "Edit" to create your output style.'}
          </SyntaxHighlighter>
        </div>
      </div>

      {/* Quick Reference */}
      <div className="card">
        <h3>Quick Reference</h3>
        <p style={{ color: 'var(--text-secondary)', marginBottom: '1rem' }}>
          Common sections to include in your output style:
        </p>
        <div className="grid-2">
          <div>
            <h4 style={{ marginBottom: '0.5rem' }}>Structure</h4>
            <ul style={{ color: 'var(--text-secondary)', fontSize: '0.875rem', paddingLeft: '1.25rem' }}>
              <li># Personality - Overall character</li>
              <li># Tone - How to communicate</li>
              <li># Language - Response language</li>
              <li># Behavior - Specific actions</li>
            </ul>
          </div>
          <div>
            <h4 style={{ marginBottom: '0.5rem' }}>Tips</h4>
            <ul style={{ color: 'var(--text-secondary)', fontSize: '0.875rem', paddingLeft: '1.25rem' }}>
              <li>Be specific with tone descriptors</li>
              <li>Include example phrases if needed</li>
              <li>Define when to use formal/informal</li>
              <li>Specify preferred terminology</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}
