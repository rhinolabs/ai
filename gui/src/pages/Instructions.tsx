import { useEffect, useState } from 'react';
import { api } from '../api';
import type { Instructions as InstructionsType, IdeInfo } from '../types';
import toast from 'react-hot-toast';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';

export default function Instructions() {
  const [instructions, setInstructions] = useState<InstructionsType | null>(null);
  const [loading, setLoading] = useState(true);
  const [availableIdes, setAvailableIdes] = useState<IdeInfo[]>([]);

  useEffect(() => {
    loadInstructions();
    loadIdes();
  }, []);

  async function loadInstructions() {
    try {
      const data = await api.getInstructions();
      setInstructions(data);
    } catch (err) {
      toast.error('Failed to load instructions');
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
    if (availableIdes.length === 0) {
      toast.error('No IDE available. Install VS Code, Cursor, or Zed.');
      return;
    }
    try {
      await api.openInstructionsInIde(availableIdes[0].command);
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
          <h1>Instructions</h1>
          <p>Your CLAUDE.md instructions file</p>
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
          <strong>Linked to Main-Profile:</strong> These instructions are installed to <code>~/.claude/CLAUDE.md</code> when you run <code>rhinolabs install</code>
        </span>
      </div>

      <div className="card">
        {instructions?.lastModified && (
          <p style={{ color: 'var(--text-secondary)', marginBottom: '1rem', fontSize: '0.875rem' }}>
            Last modified: {new Date(instructions.lastModified).toLocaleString()}
          </p>
        )}

        <div style={{
          border: '1px solid var(--border)',
          borderRadius: '0.5rem',
          overflow: 'auto',
          maxHeight: 'calc(100vh - 320px)',
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
            {instructions?.content || '# No content yet\n\nClick "Edit" to create your instructions.'}
          </SyntaxHighlighter>
        </div>
      </div>

      {/* Quick Reference */}
      <div className="card">
        <h3>Quick Reference</h3>
        <p style={{ color: 'var(--text-secondary)', marginBottom: '1rem' }}>
          Common sections to include in your instructions:
        </p>
        <div className="grid-2">
          <div>
            <h4 style={{ marginBottom: '0.5rem' }}>Structure</h4>
            <ul style={{ color: 'var(--text-secondary)', fontSize: '0.875rem', paddingLeft: '1.25rem' }}>
              <li># Rules - General guidelines</li>
              <li># Personality - Tone and style</li>
              <li># Language - Response language</li>
              <li># Skills - Auto-loaded skills table</li>
            </ul>
          </div>
          <div>
            <h4 style={{ marginBottom: '0.5rem' }}>Tips</h4>
            <ul style={{ color: 'var(--text-secondary)', fontSize: '0.875rem', paddingLeft: '1.25rem' }}>
              <li>Use markdown formatting</li>
              <li>Be specific with rules</li>
              <li>Reference @files for imports</li>
              <li>Keep it concise</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}
